use base64::{Engine as _, engine::general_purpose::STANDARD};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

use crate::{config::AppConfig, pricing::TokenUsage};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const PARTIAL_IMAGE_COUNT: u8 = 1;
const STREAM_IDLE_TIMEOUT: Duration = Duration::from_secs(300);

pub struct OpenAiImageClient {
    http_client: Client,
    api_key: String,
    endpoint: String,
    model: String,
    size: String,
    quality: String,
}

impl OpenAiImageClient {
    pub fn from_config(config: &AppConfig) -> Result<Self, OpenAiError> {
        let http_client = Client::builder()
            .user_agent(USER_AGENT)
            .connect_timeout(Duration::from_secs(30))
            .read_timeout(STREAM_IDLE_TIMEOUT)
            .build()
            .map_err(OpenAiError::BuildClient)?;

        Ok(Self {
            http_client,
            api_key: config.api_key().to_owned(),
            endpoint: config.endpoint().to_owned(),
            model: config.model().to_owned(),
            size: config.size().to_owned(),
            quality: config.quality().to_owned(),
        })
    }

    pub async fn generate_image_streaming<F>(
        &self,
        prompt: &str,
        mut on_progress: F,
    ) -> Result<GeneratedImage, OpenAiError>
    where
        F: FnMut(ImageProgress),
    {
        let prompt = prompt.trim();
        if prompt.is_empty() {
            return Err(OpenAiError::EmptyPrompt);
        }

        let request = ImageRequest {
            model: &self.model,
            prompt,
            size: &self.size,
            quality: &self.quality,
            stream: true,
            partial_images: PARTIAL_IMAGE_COUNT,
        };

        let mut response = self
            .http_client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .header(reqwest::header::ACCEPT, "text/event-stream")
            .json(&request)
            .send()
            .await
            .map_err(OpenAiError::Request)?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.map_err(OpenAiError::ReadResponse)?;
            return Err(OpenAiError::Api {
                status: status.as_u16(),
                body: compact_body(&body),
            });
        }

        let mut decoder = SseDecoder::default();
        let mut completed = None;

        while let Some(chunk) = response.chunk().await.map_err(OpenAiError::ReadStream)? {
            for data in decoder.push(&chunk) {
                if let Some(image) = handle_stream_event(&data, &mut on_progress)? {
                    completed = Some(image);
                }
            }
        }

        for data in decoder.finish() {
            if let Some(image) = handle_stream_event(&data, &mut on_progress)? {
                completed = Some(image);
            }
        }

        completed.ok_or(OpenAiError::StreamEndedWithoutCompletion)
    }
}

fn handle_stream_event<F>(
    data: &[u8],
    on_progress: &mut F,
) -> Result<Option<GeneratedImage>, OpenAiError>
where
    F: FnMut(ImageProgress),
{
    if data.is_empty() || data.starts_with(b"[DONE]") {
        return Ok(None);
    }

    let value: serde_json::Value =
        serde_json::from_slice(data).map_err(|source| OpenAiError::InvalidStreamEvent {
            source,
            body: compact_body(&String::from_utf8_lossy(data)),
        })?;
    if let Some(error) = value.get("error") {
        return Err(OpenAiError::StreamApi {
            body: compact_body(&error.to_string()),
        });
    }
    let event: StreamEvent =
        serde_json::from_value(value).map_err(|source| OpenAiError::InvalidStreamEvent {
            source,
            body: compact_body(&String::from_utf8_lossy(data)),
        })?;

    match event {
        StreamEvent::PartialImage {
            partial_image_index,
        } => {
            on_progress(ImageProgress {
                partial_image_index,
            });
            Ok(None)
        }
        StreamEvent::Completed { b64_json, usage } => {
            let bytes = STANDARD
                .decode(b64_json)
                .map_err(OpenAiError::DecodeImage)?;
            if bytes.is_empty() {
                return Err(OpenAiError::MissingImageData);
            }

            Ok(Some(GeneratedImage {
                bytes,
                usage: usage.map(TokenUsage::from),
            }))
        }
    }
}

#[derive(Debug)]
pub struct GeneratedImage {
    pub bytes: Vec<u8>,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageProgress {
    pub partial_image_index: u8,
}

#[derive(Debug, Serialize)]
struct ImageRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    size: &'a str,
    quality: &'a str,
    stream: bool,
    partial_images: u8,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum StreamEvent {
    #[serde(rename = "image_generation.partial_image")]
    PartialImage { partial_image_index: u8 },
    #[serde(rename = "image_generation.completed")]
    Completed {
        b64_json: String,
        #[serde(default)]
        usage: Option<ApiUsage>,
    },
}

#[derive(Debug, Deserialize)]
struct ApiUsage {
    input_tokens: u64,
    output_tokens: u64,
    #[serde(default)]
    input_tokens_details: Option<ApiInputTokenDetails>,
}

#[derive(Debug, Deserialize)]
struct ApiInputTokenDetails {
    text_tokens: u64,
    image_tokens: u64,
}

impl From<ApiUsage> for TokenUsage {
    fn from(usage: ApiUsage) -> Self {
        let (text_input_tokens, image_input_tokens) = usage
            .input_tokens_details
            .map(|details| (details.text_tokens, details.image_tokens))
            .unwrap_or((usage.input_tokens, 0));

        Self {
            text_input_tokens,
            image_input_tokens,
            image_output_tokens: usage.output_tokens,
        }
    }
}

#[derive(Debug, Default)]
struct SseDecoder {
    buffer: Vec<u8>,
}

impl SseDecoder {
    fn push(&mut self, chunk: &[u8]) -> Vec<Vec<u8>> {
        self.buffer.extend_from_slice(chunk);
        self.drain_complete_events()
    }

    fn finish(mut self) -> Vec<Vec<u8>> {
        let mut events = self.drain_complete_events();
        if !self.buffer.iter().all(u8::is_ascii_whitespace) {
            events.push(extract_sse_data(&self.buffer));
        }
        events
    }

    fn drain_complete_events(&mut self) -> Vec<Vec<u8>> {
        let mut events = Vec::new();
        while let Some((boundary_start, boundary_length)) = find_event_boundary(&self.buffer) {
            let raw_event = self.buffer[..boundary_start].to_vec();
            self.buffer.drain(..boundary_start + boundary_length);
            let data = extract_sse_data(&raw_event);
            if !data.is_empty() {
                events.push(data);
            }
        }
        events
    }
}

fn find_event_boundary(buffer: &[u8]) -> Option<(usize, usize)> {
    for index in 0..buffer.len().saturating_sub(1) {
        if buffer[index..].starts_with(b"\n\n") {
            return Some((index, 2));
        }
        if buffer[index..].starts_with(b"\r\n\r\n") {
            return Some((index, 4));
        }
    }
    None
}

fn extract_sse_data(raw_event: &[u8]) -> Vec<u8> {
    let mut data = Vec::new();
    for raw_line in raw_event.split(|byte| *byte == b'\n') {
        let line = raw_line.strip_suffix(b"\r").unwrap_or(raw_line);
        let Some(value) = line.strip_prefix(b"data:") else {
            continue;
        };
        let value = value.strip_prefix(b" ").unwrap_or(value);
        if !data.is_empty() {
            data.push(b'\n');
        }
        data.extend_from_slice(value);
    }
    data
}

#[derive(Debug, Error)]
pub enum OpenAiError {
    #[error("无法创建 HTTP 客户端: {0}")]
    BuildClient(#[source] reqwest::Error),
    #[error("图片请求失败: {0}")]
    Request(#[source] reqwest::Error),
    #[error("读取 OpenAI 错误响应失败: {0}")]
    ReadResponse(#[source] reqwest::Error),
    #[error("读取 OpenAI 流失败: {0}")]
    ReadStream(#[source] reqwest::Error),
    #[error("OpenAI API 返回 HTTP {status}: {body}")]
    Api { status: u16, body: String },
    #[error("OpenAI 流返回错误: {body}")]
    StreamApi { body: String },
    #[error("OpenAI 返回了无法解析的流事件: {source}; body: {body}")]
    InvalidStreamEvent {
        #[source]
        source: serde_json::Error,
        body: String,
    },
    #[error("OpenAI 流在返回完成事件前已结束")]
    StreamEndedWithoutCompletion,
    #[error("OpenAI 响应中没有图片数据")]
    MissingImageData,
    #[error("图片内容不是有效的 base64: {0}")]
    DecodeImage(#[source] base64::DecodeError),
    #[error("图片 prompt 不能为空")]
    EmptyPrompt,
}

fn compact_body(body: &str) -> String {
    const MAX_ERROR_BODY_LENGTH: usize = 2_000;
    let body = body.trim();
    if body.len() <= MAX_ERROR_BODY_LENGTH {
        body.to_owned()
    } else {
        let end = body
            .char_indices()
            .map(|(index, character)| index + character.len_utf8())
            .take_while(|end| *end <= MAX_ERROR_BODY_LENGTH)
            .last()
            .unwrap_or(0);
        format!("{}...", &body[..end])
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    use super::{
        ApiUsage, ImageProgress, ImageRequest, SseDecoder, TokenUsage, compact_body,
        handle_stream_event,
    };

    #[test]
    fn error_bodies_are_bounded() {
        let body = "x".repeat(2_001);
        assert_eq!(compact_body(&body).len(), 2_003);
    }

    #[test]
    fn unicode_error_bodies_are_truncated_on_character_boundaries() {
        let body = "错".repeat(1_000);
        let compacted = compact_body(&body);
        assert!(compacted.ends_with("..."));
        assert!(compacted.len() <= 2_003);
    }

    #[test]
    fn maps_api_usage_to_pricing_usage() {
        let api_usage: ApiUsage = serde_json::from_str(
            r#"{
                "input_tokens": 120,
                "output_tokens": 5500,
                "input_tokens_details": {
                    "text_tokens": 120,
                    "image_tokens": 0
                }
            }"#,
        )
        .expect("usage response should deserialize");

        assert_eq!(
            TokenUsage::from(api_usage),
            TokenUsage {
                text_input_tokens: 120,
                image_input_tokens: 0,
                image_output_tokens: 5_500,
            }
        );
    }

    #[test]
    fn image_request_enables_streaming_and_one_partial_image() {
        let request = ImageRequest {
            model: "gpt-image-2",
            prompt: "prompt",
            size: "1024x1536",
            quality: "high",
            stream: true,
            partial_images: 1,
        };

        let value = serde_json::to_value(request).expect("request should serialize");
        assert_eq!(value["stream"], true);
        assert_eq!(value["partial_images"], 1);
    }

    #[test]
    fn decodes_fragmented_sse_events_with_crlf() {
        let mut decoder = SseDecoder::default();
        assert!(
            decoder
                .push(b"event: image\r\ndata: {\"type\":\"image_")
                .is_empty()
        );
        let events = decoder.push(b"generation.partial_image\",\"partial_image_index\":0}\r\n\r\n");

        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0],
            br#"{"type":"image_generation.partial_image","partial_image_index":0}"#
        );
    }

    #[test]
    fn handles_partial_and_completed_events() {
        let mut progress = Vec::new();
        let partial = br#"{"type":"image_generation.partial_image","partial_image_index":0}"#;
        assert!(
            handle_stream_event(partial, &mut |event| progress.push(event))
                .expect("partial event should parse")
                .is_none()
        );
        assert_eq!(
            progress,
            vec![ImageProgress {
                partial_image_index: 0
            }]
        );

        let encoded = STANDARD.encode(b"image bytes");
        let completed = format!(
            r#"{{"type":"image_generation.completed","b64_json":"{encoded}","usage":{{"input_tokens":2,"output_tokens":3,"input_tokens_details":{{"text_tokens":2,"image_tokens":0}}}}}}"#
        );
        let image = handle_stream_event(completed.as_bytes(), &mut |_| {})
            .expect("completed event should parse")
            .expect("completed event should contain an image");

        assert_eq!(image.bytes, b"image bytes");
        assert_eq!(
            image.usage,
            Some(TokenUsage {
                text_input_tokens: 2,
                image_input_tokens: 0,
                image_output_tokens: 3,
            })
        );
    }

    #[test]
    fn surfaces_stream_error_without_a_type_field() {
        let error = handle_stream_event(
            br#"{"error":{"code":"server_error","message":"try again"}}"#,
            &mut |_| {},
        )
        .expect_err("stream error should be returned");

        assert!(error.to_string().contains("server_error"));
    }
}
