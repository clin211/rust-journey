use colored::*;

fn main() {
    // 1. loop — 无限循环，必须用 break 退出
    println!("{}", "=== loop 循环 ===".green());
    let mut count = 0;
    loop {
        count += 1;
        if count >= 5 {
            break;
        }
    }
    println!("1、loop 循环结束后 count = {count}"); // 1、loop 循环结束后 count = 5

    // 2. loop 返回值：通过 break 值 返回结果
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("2、loop 返回值：counter = {counter}, result = {result}"); // 2、loop 返回值：counter = 10, result = 20

    // 3. while — 条件循环
    println!("\n{}", "=== while 循环 ===".green());
    let mut number = 3;
    while number != 0 {
        print!("{number}! ");
        number -= 1;
    }
    println!("发射！🚀");

    // 4. for..in — 迭代循环（最常用、最安全）
    println!("\n{}", "=== for..in 循环 ===".green());
    let arr = [10, 20, 30, 40, 50];

    // 遍历数组元素
    print!("4、遍历数组元素：");
    for element in arr {
        print!("{element} ");
    }
    println!("\n4、带索引遍历：");
    for (index, value) in arr.iter().enumerate() {
        println!("   arr[{index}] = {value}");
    }

    // 5. Range 范围循环
    println!("\n{}", "=== Range 范围 ===".green());
    print!("5、1..5（不包含5）: ");
    for i in 1..5 {
        print!("{i} ");
    }
    print!("\n5、1..=5（包含5）: ");
    for i in 1..=5 {
        print!("{i} ");
    }
    print!("\n5、(1..=5).rev() 反向: ");
    for i in (1..=5).rev() {
        print!("{i} ");
    }

    // 6. break 与 continue
    println!("\n{}", "=== break 与 continue ===".green());
    print!("6、只打印偶数: ");
    for i in 1..=10 {
        if i % 2 != 0 {
            continue; // 奇数跳过
        }
        print!("{i} ");
    }

    // 7. for 循环借用 vs 所有权
    println!("\n{}", "=== 借用 vs 所有权 ===".green());
    let strings = [String::from("a"), String::from("b"), String::from("c")];

    // 借用遍历（推荐）：arr 之后仍可用
    for s in &strings {
        println!("7、借用遍历: {s}");
    }
    println!("7、遍历后数组仍可用，长度: {}", strings.len()); // 7、遍历后数组仍可用，长度: 3
}
