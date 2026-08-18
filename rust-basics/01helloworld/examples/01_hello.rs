use colored::*;

/// 1、println! — 输出后自动换行（最常用）
/// 2、print!  — 输出后不换行
fn main() {
    // 1、最基本的 Hello World
    println!("Hello, world!");

    // 2、println! 输出字符串变量
    let lang = "Rust";
    println!("I am learning {}", lang);

    // 3、print! 不换行输出
    print!("Hello, "); // 不会换行
    print!("world! "); // 紧接上一行
    println!(); // 单独换行

    // 4、print! 配合手动换行符
    // 刻意用 print!(\"...\\n\") 演示手动换行，clippy 会建议改用 println!
    #[allow(clippy::print_with_newline)]
    {
        print!("第一行\n");
        print!("第二行\n");
    }

    // 5、输出多个值
    let name = "Alice";
    let age = 30;
    println!(
        "{}{}{}",
        "姓名：".green(),
        name.green(),
        format!("，年龄：{}", age).green()
    );
}
