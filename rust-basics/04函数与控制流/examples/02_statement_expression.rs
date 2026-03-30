use colored::*;

fn main() {
    // 1. 语句 vs 表达式
    let a = 5 + 3; // 5 + 3 是表达式，值为 8
    let b = 10; // let 是语句，不返回值
    println!("1、表达式 5 + 3 = {a}，语句 let b = 10 中 b = {b}");
    // ❌ 在 Rust 中，语句不能赋值给变量
    // let x = (let y = 5); // 编译错误！let 是语句，不返回值

    // 2. 块表达式：{} 花括号包围的代码块也是表达式
    let y = {
        let x = 3;
        x + 1 // ⚠️ 没有分号！这是块的返回值
    };
    println!("2、块表达式：{{ let x = 3; x + 1 }} = {}", y); // y = 4

    // 对比：如果加了分号，块返回 ()
    let z = {
        let x = 3;
        x + 1; // 有分号，变成语句，块返回 ()
    };
    println!(
        "2、加了分号后，块表达式返回：{} = {:?}",
        "单元类型".green(),
        z
    ); // z = ()

    // 3. 块表达式的实际应用：计算最终价格
    let price = 100;
    let discount = 0.2;
    let final_price = {
        let discounted = price as f64 * (1.0 - discount);
        let tax = 0.08;
        let total = discounted * (1.0 + tax);
        println!("3、用块表达式计算最终价格: discounted = {discounted} 元, tax = {tax}, total = {total} 元"); // 3、用块表达式计算最终价格: discounted = 80 元, tax = 0.08, total = 86.4 元
        total as i32 // ⚠️ 没有分号！这是块的返回值; 显示类型转换会丢掉小数
    };
    println!("3、用块表达式计算最终价格: {final_price} 元"); // 3、用块表达式计算最终价格: 86 元

    // 4. if 也是表达式，可以直接赋值
    let x = 10;
    let description = if x > 5 { "大" } else { "小" };
    println!("4、if 表达式赋值：{x} 是一个{description}数字"); // 4、if 表达式赋值：10 是一个大数字

    // 5. match 也是表达式
    let language = "Rust";
    let message = match language {
        "Rust" => "🦀",
        "Go" => "🐹",
        "Python" => "🐍",
        _ => "❓",
    };
    println!("5、match 表达式：{language} {message}"); // 5、match 表达式：Rust 🦀

    // 6. loop 也可以作为表达式返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("6、loop 表达式：result = {result}"); // 6、loop 表达式：result = 20
}
