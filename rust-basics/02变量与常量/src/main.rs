//! 变量与常量综合演示
//!
//! 各知识点已拆分为独立示例，使用 `cargo run --example <名称>` 运行：
//!
//! | 示例 | 主题 | 运行命令 |
//! |------|------|----------|
//! | 01_variables | 变量声明、可变性、解构 | `cargo run --example 01_variables` |
//! | 02_shadowing | 变量遮蔽 | `cargo run --example 02_shadowing` |
//! | 03_const_static | 常量、静态变量、unsafe | `cargo run --example 03_const_static` |

fn main() {
    println!("变量与常量示例已拆分到 examples/ 目录，请使用以下命令运行：\n");
    println!("  cargo run --example 01_variables     # 变量声明、可变性、解构");
    println!("  cargo run --example 02_shadowing     # 变量遮蔽");
    println!("  cargo run --example 03_const_static  # 常量、静态变量、unsafe");
}
