use colored::*;

/// 格式化占位符的几种用法
/// 1、{} — 按顺序填充（最常用）
/// 2、{0}, {1} — 位置参数，可重复使用
/// 3、{name} — 命名参数，更清晰
/// 4、format! — 生成格式化字符串而不输出
fn main() {
    // 1、基本 {} 占位符，按顺序填入
    println!("{}", "=== 基本占位符 ===".green());
    println!("{} is a {}", "Rust", "language");
    println!("{} + {} = {}\n", 1, 2, 3);

    // 2、位置参数 {0} {1} {2}...，可以重复使用同一个参数
    println!("{}", "=== 位置参数 ===".green());
    println!("{0}说：\"我 喜欢 {1}，因为 {1} 很安全\"\n", "Alice", "Rust");
    // {0} = "Alice", {1} = "Rust"，{1} 被用了两次

    // 3、命名参数，可读性更好
    println!("{}", "=== 命名参数 ===".green());
    println!(
        "{name} 今年 {age} 岁，住在 {city}\n",
        name = "Bob",
        age = 25,
        city = "北京"
    );

    // 4、混用：位置参数 + 命名参数
    println!("{}", "=== 混合使用 ===".green());
    println!(
        "你好，{0}！{greeting}，你选择的语言是 {1}\n",
        "开发者",
        "Rust",
        greeting = "欢迎"
    );

    // 5、format! 宏：生成字符串，不直接输出
    println!("{}", "=== format! 宏 ===".green());
    let msg = format!("{} x {} = {}", 3, 7, 3 * 7);
    println!("计算结果：{}", msg);

    let info = format!("{name}，分数：{score}", name = "Charlie", score = 95);
    println!("学生信息：{}", info);
}
