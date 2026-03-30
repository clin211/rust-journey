use colored::*;

fn main() {
    // 1. 函数定义：无参数、无返回值
    say_hello();

    // 2. 函数参数：必须声明类型
    print_labeled_measurement(30, 'm');

    // 3. 函数返回值：用 -> 声明类型，最后一个表达式就是返回值
    let result = add(3, 5);
    println!("3 + 5 = {}", result);

    // 4. 显式 return vs 表达式返回
    println!("double(5) = {}", double(5)); // double(5) = 10
    println!("absolute_value(-3) = {}", absolute_value(-3)); // absolute_value(-3) = 3
    println!("absolute_value(3) = {}", absolute_value(3)); // absolute_value(3) = 3

    // 5. 没有返回值的函数，返回单元类型 ()
    just_print("hello"); // 无返回值函数返回单元类型 (),参数: hello
}

/// 最简单的函数：无参数、无返回值
fn say_hello() {
    println!("1、函数定义：{}", "Hello, Rust!".green());
}

/// 函数参数：必须声明类型
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("2、函数参数：The measurement is: {value}{unit_label}");
}

/// 返回值：最后一个表达式（无分号）就是返回值
fn add(a: i32, b: i32) -> i32 {
    a + b // 最后一个表达式就是返回值
}

/// 表达式返回（推荐，Rust 风格）
fn double(x: i32) -> i32 {
    x * 2
}

/// 显式 return：用于提前返回
fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    x
}

/// 没有返回值，等价于返回 ()
fn just_print(s: &str) {
    println!("5、无返回值函数返回单元类型 ()，参数: {s}");
}
