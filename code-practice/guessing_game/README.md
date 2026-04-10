# 猜数字

这是一个基于 Rust 官方教程《猜数字游戏》的练习项目，对应实践参考：<https://rustwiki.org/zh-CN/book/ch02-00-guessing-game-tutorial.html>。

## 实践目标

通过这个小项目，练习 Rust 的基础语法与常见标准库/第三方库用法：

- 使用 `rand` 生成随机数
- 使用 `std::io` 读取终端输入
- 使用 `String` 保存输入内容
- 使用 `trim().parse()` 将字符串转换为数字
- 使用 `match` 处理成功与失败分支
- 使用 `loop` 实现反复猜测
- 使用 `Ordering` 比较两个数字大小
- 在猜中后用 `break` 退出循环

## 项目依赖

当前项目依赖如下：

```toml
[dependencies]
rand = "0.9.0"
colored = "3.1.1"
```

说明：

- `rand`：用于生成随机数
- `colored`：当前项目额外使用，用于把非法输入提示显示为红色

## 核心实现

```rust
use colored::*;
use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = random_range(1..101);

    loop {
        println!("请你输入一个数字：");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}", "输入的不是数字! ".red());
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
```

## 实践拆解

### 1. 生成目标随机数

```rust
let secret_number = random_range(1..101);
```

这里生成 `1` 到 `100` 之间的随机整数，右边界 `101` 不包含在结果内。

### 2. 循环读取玩家输入

```rust
loop {
    println!("请你输入一个数字：");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
}
```

- `loop` 表示无限循环
- `String::new()` 创建一个空字符串
- `read_line(&mut guess)` 将用户输入写入 `guess`
- `&mut` 表示可变借用，因为读取操作会修改字符串内容

### 3. 将输入解析为数字

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        print!("{}", "输入的不是数字! ".red());
        continue;
    }
};
```

- `trim()` 用于去掉换行和空白字符
- `parse()` 尝试把字符串解析为数字
- `u32` 指定目标类型为无符号 32 位整数
- `match` 用于处理解析成功和失败两种情况
- `continue` 表示本轮结束，直接进入下一轮输入

### 4. 比较猜测结果

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

- `cmp(&secret_number)` 会返回一个 `Ordering`
- `Ordering::Less`：猜小了
- `Ordering::Greater`：猜大了
- `Ordering::Equal`：猜对了，使用 `break` 结束循环

## 运行方式

在项目目录下执行：

```bash
cargo run
```

运行示例：

```text
请你输入一个数字：
50
Too small!
请你输入一个数字：
75
Too big!
请你输入一个数字：
63
You win!
```

## 这个练习学到了什么

这个项目虽然简单，但涵盖了 Rust 入门阶段非常重要的内容：

- 变量与可变性
- 标准输入输出
- 基本错误处理
- 模式匹配 `match`
- 循环控制 `loop / continue / break`
- 枚举 `Ordering`
- 第三方 crate 的引入与使用

适合作为 Rust 入门的第一个完整命令行程序练习。
