mod config;
mod openai;
mod pricing;

use std::{io, time::Instant};

use config::AppConfig;
use openai::OpenAiImageClient;
use thiserror::Error;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    load_dotenv()?;

    let config = AppConfig::from_env()?;
    let client = OpenAiImageClient::from_config(&config)?;

    const POSTER_PROMPT: &str = include_str!("../poster.txt");

    println!("开始生成图片...");
    let started_at = Instant::now();
    let image = client
        .generate_image_streaming(POSTER_PROMPT, |progress| {
            println!(
                "已收到第 {} 张局部预览图，继续生成最终图片...",
                progress.partial_image_index + 1
            );
        })
        .await?;
    let elapsed = started_at.elapsed();

    let output_path = AppConfig::output_path();
    tokio::fs::write(&output_path, image.bytes).await?;
    println!("图片生成完成: {}", output_path.display());
    println!("生成耗时: {:.2} 秒", elapsed.as_secs_f64());
    print_cost(&config, image.usage);

    Ok(())
}

fn print_cost(config: &AppConfig, usage: Option<pricing::TokenUsage>) {
    if let Some(usage) = usage {
        match pricing::calculate_cost(config.model(), usage) {
            Ok(cost) => {
                println!(
                    "Token 用量: 文本输入={}, 图片输入={}, 图片输出={}",
                    usage.text_input_tokens, usage.image_input_tokens, usage.image_output_tokens
                );
                println!(
                    "成本明细: 文本输入=${:.6}, 图片输入=${:.6}, 图片输出=${:.6} USD",
                    cost.text_input_usd, cost.image_input_usd, cost.image_output_usd
                );
                println!("预估成本: ${:.6} USD", cost.total_usd());
            }
            Err(error) => eprintln!("成本计算失败: {error}"),
        }
        return;
    }

    match pricing::estimate_output_cost(config.model(), config.size(), config.quality()) {
        Ok(cost) => println!(
            "预估成本: ${cost:.6} USD（仅图片输出；API 未返回 usage，未包含 prompt 输入成本）"
        ),
        Err(error) => eprintln!("成本计算失败: {error}"),
    }
}

fn load_dotenv() -> Result<(), AppError> {
    match dotenvy::from_path(AppConfig::dotenv_path()) {
        Ok(_) => Ok(()),
        Err(error) if error.not_found() => Ok(()),
        Err(error) => Err(AppError::Dotenv(error)),
    }
}

#[derive(Debug, Error)]
enum AppError {
    #[error("读取 .env 文件失败: {0}")]
    Dotenv(#[from] dotenvy::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    OpenAi(#[from] openai::OpenAiError),
    #[error(transparent)]
    Io(#[from] io::Error),
}
