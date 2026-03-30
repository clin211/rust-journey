use colored::*;

fn main() {
    // 1. 基本 if/else if/else
    let number = 7;
    if number > 10 {
        println!("1、数字大于 10");
    } else if number > 5 {
        println!("1、数字大于 5 但不大于 10");
    } else {
        println!("1、数字不大于 5");
    }

    // 2. 条件必须是 bool 类型，不会自动转换
    // ❌ if number { ... } // 编译错误：expected `bool`, found integer
    // ✅ 必须显式比较
    if number != 0 {
        println!("2、Rust 的条件必须是 bool，{number} != 0");
    }

    // 3. if 是表达式，可以赋值给变量
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("3、if 表达式赋值：number = {number}"); // 3、if 表达式赋值：number = 5

    // 两个分支的类型必须一致
    // ❌ let number = if condition { 5 } else { "six" }; // 编译错误：类型不匹配

    // 4. 用 if 表达式实现成绩等级判断
    let score = 78;
    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    };
    println!("4、分数 {score} 对应等级: {}", grade.to_string().green()); // 4、分数 78 对应等级: C

    // 5. let-if 模式：if 作为表达式必须有 else
    // ❌ let value = if true { 42 }; // 编译错误：missing an `else` branch
    // ✅ 必须有完整的 if-else
    let status = if true { "active" } else { "inactive" };
    println!("5、let-if 模式：status = {status}"); // 5、let-if 模式：status = active
}
