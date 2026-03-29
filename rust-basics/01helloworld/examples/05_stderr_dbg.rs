use colored::*;

/// stderr 输出与 dbg! 宏
/// 1、eprintln! / eprint! — 向标准错误流输出（用于日志、错误信息）
/// 2、dbg! — 快速调试宏，自动打印 文件名:行号 = 值，并返回原值
fn main() {
    // 1、eprintln! 输出到 stderr（不会混入 stdout 的正常输出）
    println!("{}", "=== eprintln! ===".green());
    eprintln!("这是一条错误信息");
    eprint!("半条错误...");
    eprintln!("另一半"); // eprintln!() 会换行

    // stdout vs stderr 的区别：重定向时可以看出
    // cargo run --example 05_stderr_dbg > output.txt 2> error.txt
    println!("这条到 stdout");
    eprintln!("这条到 stderr");
    println!();

    // 2、dbg! 宏：开发调试利器
    println!("{}", "=== dbg! 宏 ===".green());
    let x = 42;
    let y = dbg!(x * 2); // 打印 "x * 2 = 84"，并返回 84 赋给 y
    println!("y = {}", y);

    // dbg! 可以用在表达式中间
    let result = dbg!(dbg!(1 + 2) + dbg!(3 + 4)); // 嵌套使用
    println!("result = {}", result);
    println!();

    // 3、dbg! 用于复杂数据结构
    println!("{}", "=== dbg! 调试复杂数据 ===".green());
    let data = vec!["apple", "banana", "cherry"];
    dbg!(data.get(1)); // 打印 Option 值

    let config = ("localhost", 8080, true);
    dbg!(&config); // 打印整个元组
    dbg!(&config.0); // 只打印某个字段

    // 4、实际开发场景：区分日志和输出
    println!("{}", "=== 实际场景 ===".green());
    let items = vec!["任务A", "任务B", "任务C"];
    for (i, item) in items.iter().enumerate() {
        eprintln!("[调试] 正在处理第 {} 项：{}", i, item);
        println!("✓ 已完成：{}", item);
    }
}
