# generate-image

一个使用 Rust 2024 和 OpenAI Image API 生成图片的命令行工具。程序从项目根目录读取 `.env`，将根目录下的 `poster.txt` 作为 Prompt 调用 `gpt-image-2`，把结果保存为 `poster.png`，并输出请求耗时、Token 用量和预估成本。

## 功能

- 从项目根目录的 `.env` 加载图片服务配置
- 使用 OpenAI `v1/images/generations` 的 SSE 流式接口生成图片
- 接收局部预览事件并实时输出生成进度
- 使用根目录下的独立文本文件管理 Prompt
- 将生成结果保存为 `poster.png`
- 统计从发起请求到接收完整图片的耗时
- 根据 API 返回的 Token 用量和 OpenAI 官方单价估算成本
- 对 HTTP 错误、空响应、无效 base64 和文件写入错误提供明确提示

## 环境要求

- 支持 Rust 2024 edition 的稳定版 Rust 工具链
- 可用的 OpenAI API Key
- OpenAI 账户具备 `gpt-image-2` 使用权限；部分账户可能需要完成组织验证

检查 Rust 版本：

```bash
rustc --version
cargo --version
```

## 配置

复制 `.env.example` 为本项目根目录的 `.env`，再填入真实 API Key：

```bash
cp .env.example .env
```

程序通过编译期的 `CARGO_MANIFEST_DIR` 精确定位项目根目录，因此从其他目录启动时也不会错误读取上级目录或当前工作目录中的 `.env`。

`.env` 中的配置项全部必填，程序不会为任何配置项提供默认值：

```dotenv
API_KEY=your_api_key_here
IMAGE_API_ENDPOINT=https://api.openai.com/v1/images/generations
IMAGE_MODEL=gpt-image-2
IMAGE_SIZE=1024x1536
IMAGE_QUALITY=high
```

`IMAGE_MODEL`、`IMAGE_SIZE` 和 `IMAGE_QUALITY` 会原样传给图片 API，请填写目标服务实际支持的值。项目内置的 Token 单价只支持 `gpt-image-2` 和 `gpt-image-2-2026-04-21`；API 未返回 `usage` 时使用的单张图片降级估价只覆盖 `low`、`medium`、`high` 与 `1024x1024`、`1024x1536`、`1536x1024` 的组合。

如果变量缺失、为空或不是有效 UTF-8，程序会在启动时直接报错。

`.env` 已加入 `.gitignore`，不要将真实 API Key 提交到版本库。

## 运行

在本项目目录执行：

```bash
cargo run
```

也可以从工作区根目录执行：

```bash
cargo run -p generate-image
```

成功后会看到类似输出：

```text
开始生成图片...
已收到第 1 张局部预览图，继续生成最终图片...
图片生成完成: /path/to/generate-image/poster.png
生成耗时: 42.31 秒
Token 用量: 文本输入=368, 图片输入=0, 图片输出=5500
成本明细: 文本输入=$0.001840, 图片输入=$0.000000, 图片输出=$0.165000 USD
预估成本: $0.166840 USD
```

耗时统计范围是 OpenAI 图片生成请求本身，不包含随后写入 `poster.png` 的时间。HTTP 客户端未设置请求总超时；连接超时为 30 秒，流读取超时为 5 分钟。程序会持续读取事件，直到收到 `image_generation.completed`；如果流提前结束，则返回错误且不会覆盖现有图片。

程序请求 1 张局部预览图。根据 OpenAI 官方说明，每张局部图片会额外消耗 100 个图片输出 Token；最终成本以完成事件返回的 `usage` 为准。

## 成本计算

项目将计价逻辑独立放在 [`src/pricing.rs`](src/pricing.rs) 中。根据 OpenAI 官方文档，`gpt-image-2` 标准处理的单价为：

| 类型 | 单价 |
| --- | ---: |
| 文本输入 | $5.00 / 1M tokens |
| 图片输入 | $8.00 / 1M tokens |
| 图片输出 | $30.00 / 1M tokens |

计算公式：

```text
文本输入成本 = 文本输入 tokens x 5 / 1,000,000
图片输入成本 = 图片输入 tokens x 8 / 1,000,000
图片输出成本 = 图片输出 tokens x 30 / 1,000,000
总成本 = 文本输入成本 + 图片输入成本 + 图片输出成本
```

当前项目只进行文本生成图片，因此正常情况下图片输入 tokens 为 0。程序优先使用 API 响应中的 `usage` 计算完整成本。如果 API 未返回 `usage`，则使用官方公开的单张图片输出估价作为降级结果：

| 质量 | 1024x1024 | 1024x1536 | 1536x1024 |
| --- | ---: | ---: | ---: |
| low | $0.006 | $0.005 | $0.005 |
| medium | $0.053 | $0.041 | $0.041 |
| high | $0.211 | $0.165 | $0.165 |

降级估价只包含图片输出成本，不包含 Prompt 的文本输入成本。程序输出的是基于公开标准单价的估算值，最终账单以 OpenAI 账户实际结算为准；区域处理、税费或后续价格调整可能导致差异。

官方资料：

- [GPT Image 2 模型说明](https://developers.openai.com/api/docs/models/gpt-image-2)
- [图片生成与成本计算](https://developers.openai.com/api/docs/guides/image-generation#calculating-costs)
- [OpenAI API 定价](https://developers.openai.com/api/docs/pricing#image-generation)

## Prompt

完整 Prompt 位于根目录的 [`poster.txt`](poster.txt)，通过 `include_str!` 在编译时嵌入程序。因此，修改 Prompt 后需要重新编译；执行 `cargo run` 时 Cargo 会自动完成这一步。

当前 Prompt 的目标是生成一张东方幻想风格的女性角色半身肖像，重点要求：

- 侧身回眸、空灵优雅的人物姿态与气质
- 飘动长发、彩色花朵、半透明丝质长裙和鎏金纹理
- 温暖金色逆光、轮廓光、体积光和漂浮光粒
- 干净的浅色渐变背景
- 高端 CG 插画、电影级灯光和精细细节

修改 Prompt 后重新运行程序即可生成新的图片。

## 项目结构

```text
generate-image/
├── .env                         # 本地配置和密钥，不提交到 Git
├── .env.example                 # 配置模板，可提交到 Git
├── Cargo.toml                   # Rust 2024 与依赖配置
├── README.md
├── poster.txt                   # 编译时嵌入的图片 Prompt
├── poster.png                   # 运行后生成或覆盖的图片
└── src/
    ├── config.rs                # 环境变量和输出路径
    ├── main.rs                  # CLI 流程、耗时统计和结果输出
    ├── openai.rs                # OpenAI HTTP 客户端和响应解析
    └── pricing.rs               # gpt-image-2 成本计算
```

## 验证

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```
