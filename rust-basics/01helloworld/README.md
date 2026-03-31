# 01 Hello World 与格式化输出

## 知识点概览

| 示例文件 | 主题 | 运行命令 |
|----------|------|----------|
| `01_hello.rs` | println! / print! 基本用法 | `cargo run --example 01_hello` |
| `02_format.rs` | 占位符、位置参数、命名参数、format! | `cargo run --example 02_format` |
| `03_debug.rs` | {:?} / {:#?} Debug 输出 | `cargo run --example 03_debug` |
| `04_format_spec.rs` | 宽度、对齐、精度、进制 | `cargo run --example 04_format_spec` |
| `05_stderr_dbg.rs` | stderr 输出、dbg! 宏 | `cargo run --example 05_stderr_dbg` |
| `06_comments.rs` | 行注释、块注释、文档注释 | `cargo run --example 06_comments` |
| `07_commands.rs` | rustup / cargo 常用命令速查 | `cargo run --example 07_commands` |

## 要点总结

### println! vs print

- `println!` — 输出后**自动换行**，最常用
- `print!` — 输出后**不换行**，需要手动加 `\n`

> **为什么末尾有 `!` 呢？**
>
> `!` 表示这是一个**宏（macro）**，不是普通函数。宏和函数的关键区别：
>
> - 函数的参数数量是固定的，而 `println!` 可以接受任意数量的参数（如 `println!("{}", x)` 和 `println!("{} {}", a, b)`）
> - 宏在编译时展开代码，能够对格式化字符串做**编译期检查**——如果占位符和参数数量不匹配，直接编译报错而不是运行时崩溃
>
> 除了 `println!`，本章节用到的 `format!`、`dbg!`、`eprintln!` 也都是宏。普通函数做不到这些，所以 Rust 用宏来实现。

### 占位符

| 写法 | 说明 | 示例 |
|------|------|------|
| `{}` | 按顺序填入 | `println!("{} + {} = {}", 1, 2, 3)` |
| `{0}` `{1}` | 位置参数，可复用 | `println!("{0} likes {1}, {1} is great", "Alice", "Rust")` |
| `{name}` | 命名参数 | `println!("{name} is {age}", name="Bob", age=25)` |
| `{:?}` | Debug 格式 | `println!("{:?}", vec![1,2,3])` |
| `{:#?}` | Pretty Debug（换行缩进） | `println!("{:#?}", my_struct)` |

### 格式化说明符

| 写法 | 效果 | 示例输出 |
|------|------|----------|
| `{:>10}` | 右对齐，宽 10 | `hello` |
| `{:<10}` | 左对齐，宽 10 | `hello` |
| `{:^10}` | 居中，宽 10 | `hello` |
| `{:*>10}` | 用 `*` 填充 | `*****hello` |
| `{:.2}` | 保留 2 位小数 | `3.14` |
| `{:05}` | 补零到 5 位 | `00042` |
| `{:b}` | 二进制 | `101010` |
| `{:x}` / `{:X}` | 十六进制 | `2a` / `2A` |
| `{:o}` | 八进制 | `52` |
| `{:e}` / `{:E}` | 科学计数法 | `1.23e6` |

### format! 宏

生成格式化字符串而不输出到终端，返回 `String`：

```rust
let msg = format!("{} x {} = {}", 3, 7, 21);
// msg = "3 x 7 = 21"
```

### 彩色输出（colored crate）

项目依赖了 [`colored`](https://crates.io/crates/colored) crate，用于终端彩色输出：

```rust
use colored::*;

println!("{}", "成功".green());
println!("{}", "警告".yellow());
println!("{}", "错误".red().bold());
```

常用方法链：

| 方法 | 效果 |
|------|------|
| `.red()` / `.green()` / `.blue()` ... | 前景色 |
| `.on_red()` / `.on_green()` ... | 背景色 |
| `.bold()` | 加粗 |
| `.italic()` | 斜体 |
| `.underline()` | 下划线 |

**Cargo.toml 配置：**

```toml
[dependencies]
colored = "3.1.1"
```

### stderr 输出

| 宏 | 说明 |
|----|------|
| `eprintln!` | 向 stderr 输出（换行） |
| `eprint!` | 向 stderr 输出（不换行） |

**用途：** 日志、错误信息走 stderr，正常输出走 stdout。重定向时可分离：

```bash
cargo run --example 05_stderr_dbg > output.log 2> error.log
```

### dbg! 宏

调试神器，打印 `文件:行号 = 表达式值`，并**返回原值**（不会中断代码流程）：

```rust
let y = dbg!(x * 2);  // 打印 "[src/main.rs:1] x * 2 = 84"，y 得到 84
```

### 注释（Comments）

Rust 支持三种注释：

| 类型 | 语法 | 说明 |
|------|------|------|
| 行注释 | `//` | 最常用，从 `//` 到行末都被忽略 |
| 块注释 | `/* ... */` | 可跨多行，**支持嵌套**（这是 Rust 特色） |
| 文档注释 | `///` | 修饰紧随其后的项（函数、struct 等），支持 Markdown |
| 内部文档注释 | `//!` | 修饰所在的项本身（常用于 crate 根或模块根） |

```rust
// 行注释

/* 块注释
   /* 嵌套块注释 OK */  */

/// 将两个整数相加。
///
/// # Examples
/// ```
/// assert_eq!(add(1, 2), 3);
/// ```
fn add(a: i32, b: i32) -> i32 { a + b }
```

> **生成文档：** `cargo doc --open` 会提取 `///` 和 `//!` 注释生成 HTML 文档。

### rustup 常用命令

| 命令 | 说明 |
|------|------|
| `rustup update` | 更新到最新稳定版 |
| `rustup default stable` | 设置默认工具链为 stable |
| `rustup default nightly` | 切换到 nightly |
| `rustup toolchain list` | 列出已安装的工具链 |
| `rustup toolchain install nightly` | 安装 nightly 工具链 |
| `rustup target add <target>` | 添加交叉编译目标 |
| `rustup show` | 显示当前工具链信息 |

### cargo 常用命令

| 类别 | 命令 | 说明 |
|------|------|------|
| 项目 | `cargo new <name>` | 创建新项目 |
| | `cargo new --lib <name>` | 创建库项目 |
| | `cargo init` | 在当前目录初始化 |
| 构建 | `cargo build` | 编译（debug 模式） |
| | `cargo build --release` | 编译（release，开启优化） |
| | `cargo check` | 只做语法/类型检查（最快） |
| 运行 | `cargo run` | 编译并运行 |
| | `cargo run --example <name>` | 运行 examples/ 下的示例 |
| 测试 | `cargo test` | 运行所有测试 |
| 文档 | `cargo doc --open` | 生成并打开 HTML 文档 |
| 依赖 | `cargo add <crate>` | 添加依赖 |
| | `cargo rm <crate>` | 移除依赖 |
| | `cargo update` | 更新依赖到兼容最新版 |
| | `cargo tree` | 显示依赖树 |
| 格式化 | `cargo fmt` | 自动格式化代码 |
| 检查 | `cargo clippy` | 代码质量 lint |
| 清理 | `cargo clean` | 删除 target/ 释放空间 |
| 发布 | `cargo publish` | 发布 crate 到 crates.io |

> **实用扩展：** `cargo expand`（查看宏展开）、`cargo watch -x run`（文件变化自动重运行），通过 `cargo install cargo-expand` 安装。

## 踩坑记录

1. **`{:?}` 要求类型实现 Debug trait** — 自定义 struct 需要加 `#[derive(Debug)]` 才能用 `{:?}`
2. **`print!` 不会立即刷新缓冲区** — 在某些场景下可能看不到输出，需要手动 `flush` 或使用 `println!`
3. **`{:#x}` 带前缀 vs `{:x}` 不带前缀** — `{:#x}` 输出 `0x2a`，`{:x}` 只输出 `2a`
4. **`dbg!` 会接管所有权** — 如果后续还要用该变量，传引用 `dbg!(&value)`
