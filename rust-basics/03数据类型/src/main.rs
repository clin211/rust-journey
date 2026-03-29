//! 数据类型综合演示
//!
//! 各知识点已拆分为独立示例，使用 `cargo run --example <名称>` 运行：
//!
//! | 示例 | 主题 | 运行命令 |
//! |------|------|----------|
//! | 01_integer | 整数类型、溢出处理 | `cargo run --example 01_integer` |
//! | 02_float | 浮点精度、NaN、容差比较 | `cargo run --example 02_float` |
//! | 03_bool_char | 布尔、字符、字节与字符 | `cargo run --example 03_bool_char` |
//! | 04_never_unit | Never 类型、单元类型 | `cargo run --example 04_never_unit` |
//! | 05_tuple | 元组：解构、索引、单元素 | `cargo run --example 05_tuple` |
//! | 06_array_slice | 数组切片、&str vs String | `cargo run --example 06_array_slice` |
//! | 07_string | 字符串字面量、String 创建 | `cargo run --example 07_string` |

fn main() {
    println!("数据类型示例已拆分到 examples/ 目录，请使用以下命令运行：\n");
    println!("  cargo run --example 01_integer   # 整数类型、溢出处理");
    println!("  cargo run --example 02_float      # 浮点精度、NaN、容差比较");
    println!("  cargo run --example 03_bool_char  # 布尔、字符、字节与字符");
    println!("  cargo run --example 04_never_unit # Never 类型、单元类型");
    println!("  cargo run --example 05_tuple      # 元组：解构、索引、单元素");
    println!("  cargo run --example 06_array_slice # 数组切片、&str vs String");
    println!("  cargo run --example 07_string     # 字符串字面量、String 创建");
}
