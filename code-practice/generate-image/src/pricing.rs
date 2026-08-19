use thiserror::Error;

const USD_PER_MILLION_TEXT_INPUT_TOKENS: f64 = 5.0;
const USD_PER_MILLION_IMAGE_INPUT_TOKENS: f64 = 8.0;
const USD_PER_MILLION_IMAGE_OUTPUT_TOKENS: f64 = 30.0;
const TOKENS_PER_MILLION: f64 = 1_000_000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenUsage {
    pub text_input_tokens: u64,
    pub image_input_tokens: u64,
    pub image_output_tokens: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostBreakdown {
    pub text_input_usd: f64,
    pub image_input_usd: f64,
    pub image_output_usd: f64,
}

impl CostBreakdown {
    pub fn total_usd(self) -> f64 {
        self.text_input_usd + self.image_input_usd + self.image_output_usd
    }
}

pub fn calculate_cost(model: &str, usage: TokenUsage) -> Result<CostBreakdown, PricingError> {
    ensure_supported_model(model)?;

    Ok(CostBreakdown {
        text_input_usd: token_cost(usage.text_input_tokens, USD_PER_MILLION_TEXT_INPUT_TOKENS),
        image_input_usd: token_cost(usage.image_input_tokens, USD_PER_MILLION_IMAGE_INPUT_TOKENS),
        image_output_usd: token_cost(
            usage.image_output_tokens,
            USD_PER_MILLION_IMAGE_OUTPUT_TOKENS,
        ),
    })
}

pub fn estimate_output_cost(model: &str, size: &str, quality: &str) -> Result<f64, PricingError> {
    ensure_supported_model(model)?;

    match (quality, size) {
        ("low", "1024x1024") => Ok(0.006),
        ("low", "1024x1536" | "1536x1024") => Ok(0.005),
        ("medium", "1024x1024") => Ok(0.053),
        ("medium", "1024x1536" | "1536x1024") => Ok(0.041),
        ("high", "1024x1024") => Ok(0.211),
        ("high", "1024x1536" | "1536x1024") => Ok(0.165),
        _ => Err(PricingError::UnsupportedOutput {
            size: size.to_owned(),
            quality: quality.to_owned(),
        }),
    }
}

fn token_cost(tokens: u64, price_per_million: f64) -> f64 {
    tokens as f64 * price_per_million / TOKENS_PER_MILLION
}

fn ensure_supported_model(model: &str) -> Result<(), PricingError> {
    if matches!(model, "gpt-image-2" | "gpt-image-2-2026-04-21") {
        Ok(())
    } else {
        Err(PricingError::UnsupportedModel(model.to_owned()))
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PricingError {
    #[error("没有为模型 {0} 配置计价规则")]
    UnsupportedModel(String),
    #[error("没有公开的 gpt-image-2 输出估价: size={size}, quality={quality}")]
    UnsupportedOutput { size: String, quality: String },
}

#[cfg(test)]
mod tests {
    use super::{TokenUsage, calculate_cost, estimate_output_cost};

    #[test]
    fn calculates_cost_from_token_usage() {
        let cost = calculate_cost(
            "gpt-image-2",
            TokenUsage {
                text_input_tokens: 1_000,
                image_input_tokens: 0,
                image_output_tokens: 5_500,
            },
        )
        .expect("gpt-image-2 should have pricing data");

        assert!((cost.text_input_usd - 0.005).abs() < f64::EPSILON);
        assert!((cost.image_output_usd - 0.165).abs() < f64::EPSILON);
        assert!((cost.total_usd() - 0.170).abs() < f64::EPSILON);
    }

    #[test]
    fn estimates_published_portrait_output_cost() {
        let cost = estimate_output_cost("gpt-image-2", "1024x1536", "high")
            .expect("published size and quality should be supported");

        assert!((cost - 0.165).abs() < f64::EPSILON);
    }

    #[test]
    fn accepts_the_published_snapshot() {
        assert!(estimate_output_cost("gpt-image-2-2026-04-21", "1024x1536", "low").is_ok());
    }
}
