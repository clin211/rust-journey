use colored::*;

fn main() {
    // 1. match 基本用法：匹配字面值
    println!("{}", "=== match 基本用法 ===".green());
    let number = 3;
    match number {
        1 => println!("1、匹配到 1"),
        2 => println!("1、匹配到 2"),
        3 => println!("1、匹配到 3"),
        _ => println!("1、匹配到其他值"), // _ 是通配符，匹配所有剩余情况
    }

    // 2. match 是表达式，可以赋值
    println!("\n{}", "=== match 表达式赋值 ===".green());
    let language = "Rust";
    let message = match language {
        "Rust" => "🦀 系统级语言",
        "Go" => "🐹 云原生语言",
        "Python" => "🐍 胶水语言",
        _ => "❓ 未知语言",
    };
    println!("2、{language} → {message}"); // Rust → 🦀 系统级语言

    // 3. 匹配范围
    println!("\n{}", "=== 匹配范围 ===".green());
    let score = 78;
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("3、分数 {score} 对应等级: {}", grade.to_string().green()); // 3、分数 78 对应等级: C

    // 4. match 必须穷尽所有可能（exhaustive checking）
    println!("\n{}", "=== 穷尽性检查 ===".green());
    let option_value = Some(42);
    let result = match option_value {
        Some(n) => format!("4、Some 中的值为 {n}"),
        None => String::from("4、值为 None"),
    };
    println!("{result}"); // 4、Some 中的值为 42
                          // ❌ 如果漏掉 None 分支，编译器会报错：non-exhaustive patterns

    // 5. 多模式匹配用 |
    println!("\n{}", "=== 多模式匹配 ===".green());
    let day = "周六";
    let kind = match day {
        "周一" | "周二" | "周三" | "周四" | "周五" => "工作日",
        "周六" | "周日" => "周末",
        _ => "未知",
    };
    println!("5、{day} 是{}", kind); // 5、周六 是周末

    // 6. match 守卫（guard）：附加条件
    println!("\n{}", "=== match 守卫 ===".green());
    let num = 4;
    let description = match num {
        n if n % 2 == 0 => "偶数",
        _ => "奇数",
    };
    println!("6、{num} 是{description}"); // 6、4 是偶数
}
