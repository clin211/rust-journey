use colored::*;
use rand::random_range;
use std::cmp::Ordering;
use std::io; // 引入 io 标准库

fn main() {
    // 生成一个随机数
    let secret_number = random_range(1..101); // 生成 1 到 100 的随机数

    loop {
        println!("请你输入一个数字：");

        let mut guess = String::new();

        // 读取用户输入
        io::stdin()
            .read_line(&mut guess) // 读取用户输入到 guess 中
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}", "输入的不是数字! ".red()); // 打印错误信息，红色
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
