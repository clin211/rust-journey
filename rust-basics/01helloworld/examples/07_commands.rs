/// # 07 rustup 与 cargo 常用命令速查
///
/// 本文件以代码注释的形式整理 rustup 和 cargo 最常用的命令，
/// 方便随时查阅。
///
/// 运行：`cargo run --example 07_commands`

use colored::*;

fn main() {
    println!("{}", "rustup 与 cargo 常用命令速查".green().bold());
    println!("{}", "=".repeat(40));

    // ─────────────────────────────────────
    // rustup — Rust 工具链管理器
    // ─────────────────────────────────────
    println!("\n{}", "【rustup】工具链管理".cyan().bold());

    /*
    rustup update              // 更新到最新稳定版
    rustup default stable      // 设置默认工具链为 stable
    rustup default nightly     // 切换到 nightly（可体验实验性特性）
    rustup toolchain list      // 列出已安装的工具链
    rustup toolchain install nightly  // 安装 nightly 工具链
    rustup show                // 显示当前工具链信息
    rustup self update         // 更新 rustup 自身
    rustup target list         // 列出所有编译目标
    rustup target add x86_64-pc-windows-gnu  // 添加交叉编译目标
    rustup run nightly rustc --version  // 用指定工具链运行命令
    */

    println!("  {}  {}", "•".yellow(), "rustup update — 更新到最新稳定版");
    println!("  {}  {}", "•".yellow(), "rustup default stable/nightly — 切换工具链");
    println!("  {}  {}", "•".yellow(), "rustup toolchain list — 列出已安装工具链");
    println!("  {}  {}", "•".yellow(), "rustup target add <target> — 添加交叉编译目标");

    // ─────────────────────────────────────
    // rustc — Rust 编译器（一般通过 cargo 间接调用）
    // ─────────────────────────────────────
    println!("\n{}", "【rustc】编译器".cyan().bold());

    /*
    rustc --version            // 查看编译器版本
    rustc main.rs              // 直接编译单个文件 → 生成可执行文件 main
    rustc --edition 2024 main.rs  // 指定 edition 编译
    */

    println!("  {}  {}", "•".yellow(), "rustc --version — 查看编译器版本");
    println!("  {}  {}", "•".yellow(), "rustc main.rs — 编译单个文件");

    // ─────────────────────────────────────
    // cargo — Rust 构建工具 & 包管理器
    // ─────────────────────────────────────
    println!("\n{}", "【cargo】构建与包管理".cyan().bold());

    // --- 项目管理 ---
    println!("\n  {}", "项目管理".white().bold());
    /*
    cargo new my_project       // 创建新的二进制项目
    cargo new --lib my_lib     // 创建新的库项目
    cargo init                 // 在当前目录初始化项目
    */
    println!("    cargo new <name>        — 创建新项目");
    println!("    cargo new --lib <name>  — 创建库项目");
    println!("    cargo init              — 在当前目录初始化");

    // --- 构建与运行 ---
    println!("\n  {}", "构建与运行".white().bold());
    /*
    cargo build                // 编译（debug 模式，默认）
    cargo build --release      // 编译（release 模式，开启优化）
    cargo run                  // 编译并运行
    cargo run --release        // release 模式编译并运行
    cargo check                // 只做语法/类型检查，不生成二进制（比 build 快）
    */
    println!("    cargo build              — 编译（debug）");
    println!("    cargo build --release    — 编译（release，开启优化）");
    println!("    cargo run                — 编译并运行");
    println!("    cargo check              — 只检查语法/类型，不生成二进制（最快）");

    // --- 运行 example ---
    println!("\n  {}", "运行 example".white().bold());
    /*
    cargo run --example 01_hello       // 运行 examples/01_hello.rs
    cargo run --example 06_comments    // 运行 examples/06_comments.rs
    */
    println!("    cargo run --example <name> — 运行 examples/ 下的示例");

    // --- 测试 ---
    println!("\n  {}", "测试".white().bold());
    /*
    cargo test                 // 运行所有测试
    cargo test -- --nocapture  // 运行测试并显示 println! 输出
    cargo test test_add        // 只运行名称包含 test_add 的测试
    */
    println!("    cargo test               — 运行所有测试");
    println!("    cargo test <name>        — 只运行匹配的测试");
    println!("    cargo test -- --nocapture — 显示 println! 输出");

    // --- 文档 ---
    println!("\n  {}", "文档".white().bold());
    /*
    cargo doc                  // 生成文档（放在 target/doc/）
    cargo doc --open           // 生成文档并在浏览器中打开
    */
    println!("    cargo doc --open         — 生成并在浏览器打开文档");

    // --- 依赖管理 ---
    println!("\n  {}", "依赖管理".white().bold());
    /*
    cargo add serde            // 添加依赖到 Cargo.toml
    cargo add serde --features derive  // 添加依赖并启用 feature
    cargo rm serde             // 移除依赖
    cargo update               // 更新所有依赖到兼容的最新版本
    cargo tree                 // 以树形结构显示依赖关系
    cargo outdated             // 查看过期的依赖（需安装 cargo-outdated）
    */
    println!("    cargo add <crate>        — 添加依赖");
    println!("    cargo add <crate> --features <f> — 添加依赖并启用 feature");
    println!("    cargo rm <crate>         — 移除依赖");
    println!("    cargo update             — 更新依赖到兼容最新版");
    println!("    cargo tree               — 显示依赖树");

    // --- 格式化与 lint ---
    println!("\n  {}", "格式化与代码检查".white().bold());
    /*
    cargo fmt                  // 自动格式化代码（按 rustfmt 风格）
    cargo fmt -- --check       // 只检查是否格式化，不修改（CI 常用）
    cargo clippy               // 运行 Clippy lint（代码质量检查）
    cargo clippy -- -W clippy::all  // 开启所有 clippy 警告
    */
    println!("    cargo fmt                — 自动格式化代码");
    println!("    cargo fmt -- --check     — 检查格式（CI 常用）");
    println!("    cargo clippy             — 代码质量检查（lint）");

    // --- 发布 ---
    println!("\n  {}", "发布".white().bold());
    /*
    cargo publish              // 将 crate 发布到 crates.io
    cargo login <token>        // 登录 crates.io（只需一次）
    */
    println!("    cargo publish            — 发布 crate 到 crates.io");

    // --- 清理 ---
    println!("\n  {}", "清理".white().bold());
    /*
    cargo clean                // 删除 target/ 目录（释放磁盘空间）
    */
    println!("    cargo clean              — 删除 target/ 释放空间");

    // ─────────────────────────────────────
    // 其他实用工具（cargo 子命令，需单独安装）
    // ─────────────────────────────────────
    println!("\n{}", "【实用 cargo 扩展】（需单独安装）".cyan().bold());
    /*
    cargo install cargo-expand       // 查看宏展开后的代码
    cargo expand                     // 使用：cargo expand
    cargo install cargo-watch        // 文件变化时自动执行命令
    cargo watch -x run               // 文件变化时自动 cargo run
    cargo install cargo-edit         // 提供 cargo add/rm（旧版，新 cargo 已内置）
    */
    println!("  cargo expand              — 查看宏展开代码");
    println!("  cargo watch -x run        — 文件变化自动重新运行");
    println!("  cargo install <tool>      — 安装 cargo 扩展工具");

    println!(
        "\n{}",
        "提示：`cargo --help` 查看所有命令，`cargo <cmd> --help` 查看子命令帮助".yellow()
    );
}
