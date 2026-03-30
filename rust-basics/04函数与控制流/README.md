# 函数与控制流 (functions & control flow)

> Rust 中函数和控制流是构建程序逻辑的基础。Rust 的函数设计受函数式编程影响，
> 其中最核心的概念是 **"表达式优先"** —— 几乎一切都可以求值并返回结果。

---

## 示例文件

| 文件 | 主题 | 运行 |
|------|------|------|
| `examples/01_functions.rs` | 函数定义、参数、返回值 | `cargo run --example 01_functions` |
| `examples/02_statement_expression.rs` | 语句 vs 表达式、块表达式 | `cargo run --example 02_statement_expression` |
| `examples/03_if_expression.rs` | if/else if/else 表达式 | `cargo run --example 03_if_expression` |
| `examples/04_loops.rs` | loop、while、for..in、break/continue | `cargo run --example 04_loops` |
| `examples/05_loop_labels.rs` | 循环标签与嵌套控制 | `cargo run --example 05_loop_labels` |
| `examples/06_match.rs` | match 模式匹配基础 | `cargo run --example 06_match` |

---

## 目录

- [函数与控制流 (functions \& control flow)](#函数与控制流-functions--control-flow)
  - [示例文件](#示例文件)
  - [目录](#目录)
  - [一、函数基础](#一函数基础)
    - [1.1 函数定义](#11-函数定义)
    - [1.2 函数参数](#12-函数参数)
    - [1.3 函数返回值](#13-函数返回值)
    - [1.4 显式 return vs 表达式返回](#14-显式-return-vs-表达式返回)
  - [二、语句 vs 表达式（Rust 的灵魂）](#二语句-vs-表达式rust-的灵魂)
    - [2.1 什么是语句，什么是表达式](#21-什么是语句什么是表达式)
    - [2.2 块表达式](#22-块表达式)
    - [2.3 表达式的实际应用场景](#23-表达式的实际应用场景)
  - [三、if 表达式](#三if-表达式)
    - [3.1 基本用法](#31-基本用法)
    - [3.2 if 是表达式，可以赋值](#32-if-是表达式可以赋值)
    - [3.3 多条件分支 else if](#33-多条件分支-else-if)
    - [3.4 let-if 模式的注意事项](#34-let-if-模式的注意事项)
  - [四、循环](#四循环)
    - [4.1 loop — 无限循环](#41-loop--无限循环)
    - [4.2 while — 条件循环](#42-while--条件循环)
    - [4.3 for..in — 迭代循环](#43-forin--迭代循环)
    - [4.4 三种循环的选择指南](#44-三种循环的选择指南)
  - [五、循环控制](#五循环控制)
    - [5.1 break 与 continue](#51-break-与-continue)
    - [5.2 loop 返回值](#52-loop-返回值)
    - [5.3 循环标签](#53-循环标签)
  - [六、match 模式匹配](#六match-模式匹配)
    - [6.1 基本用法](#61-基本用法)
    - [6.2 match 是表达式](#62-match-是表达式)
    - [6.3 匹配范围](#63-匹配范围)
    - [6.4 穷尽性检查](#64-穷尽性检查)
    - [6.5 多模式匹配 |](#65-多模式匹配-)
    - [6.6 match 守卫（guard）](#66-match-守卫guard)
  - [七、综合练习](#七综合练习)
    - [练习 1：温度转换器](#练习-1温度转换器)
    - [练习 2：斐波那契数列](#练习-2斐波那契数列)
    - [练习 3：FizzBuzz](#练习-3fizzbuzz)
    - [练习 4：猜数字游戏（完整版）](#练习-4猜数字游戏完整版)
    - [练习 5：九九乘法表](#练习-5九九乘法表)
  - [要点总结](#要点总结)
    - [函数核心要点](#函数核心要点)
    - [语句 vs 表达式](#语句-vs-表达式)
    - [if 要点](#if-要点)
    - [循环要点](#循环要点)
    - [match 要点](#match-要点)

---

## 一、函数基础

### 1.1 函数定义

Rust 中用 `fn` 关键字定义函数。函数名使用 **snake_case** 风格（小写字母 + 下划线）。

```rust
// 最简单的函数：无参数、无返回值
fn say_hello() {
    println!("Hello, Rust!");
}

// main 是程序入口函数
fn main() {
    say_hello();  // 调用函数
}
```

**要点：**

- `fn` 是关键字，不可变
- 函数名必须是 snake_case，否则编译器会警告
- 函数可以在 `main` 之前或之后定义，Rust 不关心顺序（与 C 不同）
- 函数体放在 `{}` 中

```rust
// ✅ Rust 不关心函数定义的顺序
fn main() {
    greet();       // 可以调用后面定义的函数
    farewell();
}

fn farewell() {
    println!("Goodbye!");
}

fn greet() {
    println!("Welcome!");
}
```

### 1.2 函数参数

函数参数必须 **声明类型**，Rust 不会对函数参数做类型推断。

```rust
// 单个参数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn main() {
    print_labeled_measurement(30, 'm');  // The measurement is: 30m
}
```

**参数是值传递（move 语义）：**

```rust
fn take_string(s: String) {
    println!("I got: {s}");
    // s 在这里离开作用域，被 drop
}

fn main() {
    let text = String::from("hello");
    take_string(text);
    // println!("{text}");  // ❌ 编译错误！text 的所有权已经转移给了函数
    println!("text is no longer valid here");
}
```

如果想在函数使用后继续拥有变量，可以传引用或克隆：

```rust
fn take_string(s: String) {
    println!("I got: {s}");
}

fn main() {
    let text = String::from("hello");

    // 方法1：传引用（后面"引用与借用"会详细学）
    fn borrow_string(s: &String) {
        println!("I borrowed: {s}");
    }
    borrow_string(&text);
    println!("{text} is still valid"); // ✅ text 还有效

    // 方法2：克隆
    take_string(text.clone());
    println!("{text} is still valid too"); // ✅ text 还有效
}
```

### 1.3 函数返回值

返回值类型用 `->` 箭头声明。Rust 函数的返回值等同于函数体最后一个表达式的值。

```rust
// 返回一个 i32
fn add(a: i32, b: i32) -> i32 {
    a + b           // ⚠️ 注意：没有分号！这是一个表达式，它的值就是返回值
}

fn main() {
    let result = add(3, 5);
    println!("3 + 5 = {result}");  // 3 + 5 = 8
}
```

```rust
fn main() {
    broken_add(10, 10);
}

// 如果加了分号，就变成了语句，函数就不返回值了
fn broken_add(a: i32, b: i32) -> i32 {
    a + b;          // ❌ 编译错误！加了分号变成语句，返回 ()
                    // 但函数声明返回 i32，类型不匹配
}
```

源码在：<https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=04a584862392eec569ee46b5c450c901>

编译器会给出非常清晰的错误提示：

```sh
   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:5:34
  |
5 | fn broken_add(a: i32, b: i32) -> i32 {
  |    ----------                    ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
6 |     a + b;          // ❌ 编译错误！加了分号变成语句，返回 ()
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```

### 1.4 显式 return vs 表达式返回

Rust 有两种返回值的方式：

```rust
// 方式1：表达式返回（推荐，Rust 风格）
fn double(x: i32) -> i32 {
    x * 2           // 最后一个表达式就是返回值，没有分号
}

// 方式2：显式 return（提前返回时使用）
fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;  // 提前返回，需要 return 关键字
    }
    x               // 正常返回，不需要 return
}

// 方式3：没有返回值的函数，返回单元类型 ()
fn just_print(s: &str) {
    println!("{s}");
    // 等价于返回 ()，可以写 -> () 但没人这么写
}

fn main() {
    println!("double(5) = {}", double(5));           // 10
    println!("absolute_value(-3) = {}", absolute_value(-3)); // 3
    println!("absolute_value(3) = {}", absolute_value(3));   // 3

    let unit = just_print("hi");
    println!("返回值是: {:?}", unit);  // 返回值是: ()
}
```

**规则总结：**

- 函数最后一个表达式（无分号）= 返回值
- `return` 关键字用于 **提前返回**（如在 if 分支中、在循环中）
- 不写 `->` 返回类型，默认返回 `()`（单元类型，空元组）

```rust
// return 的典型使用场景：提前退出（guard clause 模式）
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;    // 除数为 0，提前返回
    }
    Some(a / b)         // 正常返回
}

fn main() {
    match divide(10.0, 3.0) {
        Some(result) => println!("结果: {result:.2}"),
        None => println!("除数不能为 0"),
    }
}
```

---

## 二、语句 vs 表达式（Rust 的灵魂）

这是 Rust 和 C/Java/Python 最大的区别之一，理解这个概念是学好 Rust 的关键。

### 2.1 什么是语句，什么是表达式

| | 语句 (Statement) | 表达式 (Expression) |
|---|---|---|
| **定义** | 执行操作，**不返回值** | 求值并 **返回一个值** |
| **标志** | 以分号 `;` 结尾 | 没有分号 |
| **例子** | `let x = 5;`、`fn foo() {}` | `5 + 3`、`x * 2`、`{ ... }` |

```rust
fn main() {
    // 表达式：会计算出一个值
    let a = 5 + 3;        // 5 + 3 是表达式，值为 8
    let b = 2 * 4;        // 2 * 4 是表达式，值为 8

    // 语句：执行操作但不返回值
    let c = 10;           // let x = 5; 是一个语句，不返回值

    // ❌ 在 Rust 中，语句不能赋值给变量
    // let x = (let y = 5);  // 编译错误！let 是语句，不返回值

    // ✅ 但在 C 语言中这是合法的: int x = (y = 5);
    // Rust 故意不允许这样做，避免混淆
}
```

### 2.2 块表达式

`{}` 花括号包围的代码块也是一个表达式！它的值是 **最后一个表达式的值**。

```rust
fn main() {
    // 块表达式：用花括号创建一个作用域，最后一个表达式就是块的值
    let y = {
        let x = 3;
        x + 1       // ⚠️ 没有分号！这是块的返回值
    };

    println!("y = {y}");  // y = 4

    // 对比：如果加了分号
    let z = {
        let x = 3;
        x + 1;      // 有分号，变成语句，块返回 ()
    };

    println!("z = {:?}", z);  // z = ()
}
```

**块表达式可以包含复杂的逻辑：**

```rust
fn main() {
    let price = 100;
    let discount = 0.2;

    // 用块表达式计算最终价格
    let final_price = {
        let discounted = price as f64 * (1.0 - discount);
        let tax = 0.08;
        let total = discounted * (1.0 + tax);
        total as i32  // 块的返回值
    };

    println!("最终价格: {final_price} 元");  // 最终价格: 86 元
}
```

### 2.3 表达式的实际应用场景

```rust
fn main() {
    let x = 10;

    // 场景1：if 是表达式，直接赋值
    let description = if x > 5 { "大" } else { "小" };
    println!("{x} 是一个{description}数字");

    // 场景2：块表达式 + 嵌套逻辑
    let grade = {
        let score = 85;
        if score >= 90 {
            'A'
        } else if score >= 80 {
            'B'
        } else if score >= 70 {
            'C'
        } else {
            'D'
        }
    };
    println!("等级: {grade}");

    // 场景3：match 表达式（后面会详细学）
    let language = "Rust";
    let message = match language {
        "Rust" => "🦀",
        "Go" => "🐹",
        "Python" => "🐍",
        _ => "❓",
    };
    println!("{language} {message}");

    // 场景4：loop 也可以作为表达式返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // loop 的返回值
        }
    };
    println!("result = {result}");  // result = 20
}
```

---

## 三、if 表达式

### 3.1 基本用法

```rust
fn main() {
    let number = 7;

    if number > 10 {
        println!("数字大于 10");
    } else if number > 5 {
        println!("数字大于 5 但不大于 10");
    } else {
        println!("数字不大于 5");
    }
}
```

**注意：Rust 的条件必须是 bool 类型，不会自动转换！**

```rust
fn main() {
    let number = 3;

    // ❌ 编译错误！Rust 不会把整数自动转成 bool
    // if number {
    //     println!("number is not zero");
    // }

    // ✅ 必须显式比较
    if number != 0 {
        println!("number is not zero: {number}");
    }
}
```

对比其他语言：

```c
// C 语言：合法，任何非零值都被视为 true
if (number) { ... }

// Python：合法
if number: ...

// Rust：❌ 编译错误，必须是 bool
// if number { ... }    // error: expected `bool`, found integer
```

### 3.2 if 是表达式，可以赋值

因为 if 是表达式，所以它可以直接用在 `let` 语句的右边。

```rust
fn main() {
    let condition = true;

    // if 表达式的值赋给 number
    let number = if condition { 5 } else { 6 };

    println!("number = {number}");  // number = 5
}
```

**两个分支的类型必须一致：**

```rust
fn main() {
    let condition = true;

    // ❌ 编译错误！两个分支返回的类型不同
    // let number = if condition { 5 } else { "six" };
    // error: `if` and `else` have incompatible types

    // ✅ 类型一致
    let number = if condition { 5 } else { 6 };
}
```

### 3.3 多条件分支 else if

```rust
fn main() {
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

    println!("分数 {score} 对应等级: {grade}");  // 分数 78 对应等级: C
}
```

### 3.4 let-if 模式的注意事项

```rust
fn main() {
    // ✅ 正确：两个分支都是 &str 类型
    let status = if true { "active" } else { "inactive" };

    // ❌ 错误：缺少 else 分支。if 作为表达式必须有 else
    // let value = if true { 42 };
    // error: missing an `else` branch

    // ✅ 正确：必须有完整的 if-else
    let value = if true { 42 } else { 0 };
}
```

---

## 四、循环

Rust 提供三种循环：`loop`、`while`、`for..in`。

### 4.1 loop — 无限循环

`loop` 会反复执行，直到你明确告诉它停下（用 `break`）。

```rust
fn main() {
    let mut count = 0;

    loop {
        println!("count = {count}");
        count += 1;

        if count >= 5 {
            break;  // 退出循环
        }
    }
    // 输出: count = 0, 1, 2, 3, 4
}
```

**loop 适用场景：** 你不确定要循环多少次，或者需要一个无限循环（如服务器主循环、游戏主循环）。

### 4.2 while — 条件循环

`while` 在条件为 true 时反复执行。

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("发射！🚀");
    // 输出: 3!  2!  1!  发射！🚀
}
```

**while 遍历数组（不推荐，Rust 会做边界检查，性能较差）：**

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < arr.len() {
        println!("arr[{index}] = {}", arr[index]);
        index += 1;
    }
    // 能工作，但每次访问 arr[index] 都会做运行时边界检查
}
```

### 4.3 for..in — 迭代循环

`for..in` 是 Rust 中最常用、最安全的循环方式。

```rust
fn main() {
    // 遍历数组（推荐方式）
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("值: {element}");
    }

    // 需要索引时，用 .iter().enumerate()
    for (index, value) in arr.iter().enumerate() {
        println!("arr[{index}] = {value}");
    }
}
```

**范围 (Range) 循环：**

```rust
fn main() {
    // 1..5 表示 [1, 5)，即 1, 2, 3, 4
    for i in 1..5 {
        println!("i = {i}");
    }

    // 1..=5 表示 [1, 5]，即 1, 2, 3, 4, 5
    for i in 1..=5 {
        println!("i = {i}");
    }

    // 反向遍历
    for i in (1..=5).rev() {
        println!("倒计时: {i}");
    }
    // 输出: 5, 4, 3, 2, 1
}
```

**for 循环获取元素的所有权 vs 引用：**

```rust
fn main() {
    let arr = [String::from("a"), String::from("b"), String::from("c")];

    // 方式1：for element in arr — 获取所有权（arr 之后不可用）
    // 方式2：for element in &arr — 借用（arr 之后仍可用）✅ 推荐
    // 方式3：for element in &mut arr — 可变借用（可以修改元素）

    for s in &arr {
        println!("{s}");
    }

    // arr 还能继续使用
    println!("数组长度: {}", arr.len());
}
```

### 4.4 三种循环的选择指南

| 循环类型 | 使用场景 | 示例 |
|---|---|---|
| `loop` | 不确定何时停止，或需要循环返回值 | 重试逻辑、服务器主循环 |
| `while` | 有明确条件但不确定次数 | 猜数字游戏、读取到EOF |
| `for..in` | 遍历集合或已知范围 | 遍历数组、计数循环 |

**经验法则：** 如果你在用 `while` 遍历集合，应该改用 `for..in`。

```rust
fn main() {
    // ❌ 不推荐：while 遍历
    let arr = [1, 2, 3, 4, 5];
    let mut i = 0;
    while i < arr.len() {
        println!("{}", arr[i]);
        i += 1;
    }

    // ✅ 推荐：for 遍历（更安全、更快）
    for val in arr {
        println!("{val}");
    }
}
```

---

## 五、循环控制

### 5.1 break 与 continue

```rust
fn main() {
    // break：立即退出整个循环
    println!("=== break 示例 ===");
    let mut i = 0;
    loop {
        if i == 5 {
            break;  // 当 i 等于 5 时退出
        }
        println!("i = {i}");
        i += 1;
    }

    // continue：跳过本次迭代，进入下一次
    println!("=== continue 示例：只打印偶数 ===");
    for i in 1..=10 {
        if i % 2 != 0 {
            continue;  // 奇数跳过
        }
        println!("{i}");
    }
    // 输出: 2, 4, 6, 8, 10
}
```

**实际案例：在循环中查找目标**

```rust
fn main() {
    let numbers = [2, 7, 1, 8, 3, 9, 4];
    let target = 8;
    let mut found_index = -1;

    for (i, &num) in numbers.iter().enumerate() {
        if num == target {
            found_index = i as i32;
            break;  // 找到了，不需要继续
        }
    }

    if found_index >= 0 {
        println!("找到 {target}，索引为 {found_index}");
    } else {
        println!("未找到 {target}");
    }
}
```

### 5.2 loop 返回值

`loop` 是表达式，可以通过 `break 值` 返回结果。这是 Rust 的独特特性。

```rust
fn main() {
    let mut counter = 0;

    // loop 的返回值赋给 result
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // 返回 20
        }
    };

    println!("counter = {counter}, result = {result}");
    // counter = 10, result = 20
}
```

**实际案例：重试逻辑**

```rust
use std::thread;
use std::time::Duration;

// 模拟一个可能失败的操作
fn try_connect() -> bool {
    // 模拟随机成功/失败
    static mut ATTEMPT: i32 = 0;
    unsafe {
        ATTEMPT += 1;
        ATTEMPT >= 3  // 第 3 次才成功
    }
}

fn main() {
    let mut retries = 0;
    let max_retries = 5;

    let connected = loop {
        retries += 1;
        println!("第 {retries} 次尝试连接...");

        if try_connect() {
            break true;    // 连接成功，返回 true
        }

        if retries >= max_retries {
            break false;   // 超过最大重试次数，返回 false
        }

        // 等待 1 秒后重试
        thread::sleep(Duration::from_secs(1));
    };

    if connected {
        println!("连接成功！（共尝试 {retries} 次）");
    } else {
        println!("连接失败，已达最大重试次数");
    }
}
```

### 5.3 循环标签

当有嵌套循环时，`break` 和 `continue` 默认作用于 **最内层** 的循环。
使用循环标签可以指定操作哪一层循环。

```rust
fn main() {
    let mut count = 0;

    // 'outer 是循环标签，以单引号开头
    'outer: loop {
        println!("--- 外层循环，count = {count} ---");

        let mut remaining = 10;

        'inner: loop {
            println!("  内层循环，remaining = {remaining}");

            if remaining == 9 {
                break;  // 默认退出最内层循环（'inner）
            }
            if count == 2 {
                break 'outer;  // 退出指定标签的循环（'outer）
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("最终 count = {count}");
}
```

输出：

```text
--- 外层循环，count = 0 ---
  内层循环，remaining = 10
  内层循环，remaining = 9
--- 外层循环，count = 1 ---
  内层循环，remaining = 10
  内层循环，remaining = 9
--- 外层循环，count = 2 ---
  内层循环，remaining = 10
最终 count = 2
```

**continue 也可以用标签：**

```rust
fn main() {
    // 打印乘法表，但跳过包含 5 的行
    'outer: for i in 1..=9 {
        for j in 1..=9 {
            if i == 5 {
                continue 'outer;  // 跳过 i=5 的整行
            }
            if j == 5 {
                continue;  // 跳过 j=5 的列
            }
            print!("{:4}", i * j);
        }
        println!();
    }
}
```

---

## 六、match 模式匹配

> 对应示例：`examples/06_match.rs`

`match` 是 Rust 中强大的模式匹配控制流，允许将一个值与一系列模式进行比较，
并根据匹配的模式执行代码。编译器会确保所有可能的情况都被处理（穷尽性检查）。

### 6.1 基本用法

```rust
fn main() {
    let number = 3;
    match number {
        1 => println!("匹配到 1"),
        2 => println!("匹配到 2"),
        3 => println!("匹配到 3"),
        _ => println!("匹配到其他值"), // _ 是通配符
    }
}
```

### 6.2 match 是表达式

```rust
fn main() {
    let language = "Rust";
    let message = match language {
        "Rust" => "🦀 系统级语言",
        "Go" => "🐹 云原生语言",
        "Python" => "🐍 胶水语言",
        _ => "❓ 未知语言",
    };
    println!("{language} → {message}");
}
```

### 6.3 匹配范围

```rust
fn main() {
    let score = 78;
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
}
```

### 6.4 穷尽性检查

```rust
fn main() {
    let option_value = Some(42);
    let result = match option_value {
        Some(n) => format!("值为 {n}"),
        None => String::from("值为 None"),
    };
    // ❌ 如果漏掉 None 分支，编译器会报错：non-exhaustive patterns
}
```

### 6.5 多模式匹配 |

```rust
fn main() {
    let day = "周六";
    let kind = match day {
        "周一" | "周二" | "周三" | "周四" | "周五" => "工作日",
        "周六" | "周日" => "周末",
        _ => "未知",
    };
}
```

### 6.6 match 守卫（guard）

```rust
fn main() {
    let num = 4;
    let description = match num {
        n if n % 2 == 0 => "偶数",
        _ => "奇数",
    };
}
```

> 更复杂的模式匹配（解构、`@` 绑定、`if let` 等）将在 **08枚举与模式匹配** 中深入展开。

---

## 七、综合练习

### 练习 1：温度转换器

```rust
/// 将摄氏度转换为华氏度
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// 将华氏度转换为摄氏度
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    let celsius = 37.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius}°C = {fahrenheit}°F");

    let fahrenheit = 98.6;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}°F = {celsius:.1}°C");
}
```

### 练习 2：斐波那契数列

```rust
/// 计算第 n 个斐波那契数（从 0 开始）
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0u64;
    let mut b = 1u64;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

fn main() {
    // 打印前 20 个斐波那契数
    for i in 0..20 {
        println!("fib({i}) = {}", fibonacci(i));
    }
}
```

### 练习 3：FizzBuzz

经典编程练习：打印 1 到 100，遇到 3 的倍数打印 "Fizz"，5 的倍数打印 "Buzz"，15 的倍数打印 "FizzBuzz"。

```rust
fn fizzbuzz(n: u32) -> &'static str {
    if n % 15 == 0 {
        "FizzBuzz"
    } else if n % 3 == 0 {
        "Fizz"
    } else if n % 5 == 0 {
        "Buzz"
    } else {
        // 这里我们无法返回动态字符串，先返回一个占位符
        // 更好的实现后面学了 String 再改进
        "Number"
    }

    // 更实用的实现：直接打印
}

fn main() {
    for n in 1..=100 {
        let output = if n % 15 == 0 {
            "FizzBuzz".to_string()
        } else if n % 3 == 0 {
            "Fizz".to_string()
        } else if n % 5 == 0 {
            "Buzz".to_string()
        } else {
            n.to_string()
        };
        println!("{output}");
    }
}
```

### 练习 4：猜数字游戏（完整版）

综合运用函数、控制流、循环标签：

```rust
use std::io;

fn read_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    input.trim().to_string()
}

fn parse_number(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn main() {
    println!("=== 猜数字游戏 ===");

    let secret = 42; // 实际中可以用 rand crate 生成随机数

    'game: loop {
        let mut attempts = 0;

        'guessing: loop {
            let input = read_input("请输入你猜的数字 (1-100)，输入 q 退出:");
            attempts += 1;

            // 退出游戏
            if input == "q" {
                println!("再见！");
                break 'game;
            }

            // 解析输入
            let guess = match parse_number(&input) {
                Some(n) => n,
                None => {
                    println!("请输入有效数字！");
                    continue;
                }
            };

            // 判断结果
            let result = if guess < secret {
                "太小了！"
            } else if guess > secret {
                "太大了！"
            } else {
                "恭喜你猜对了！"
            };

            println!("{result}");

            if guess == secret {
                println!("你用了 {attempts} 次猜对了！");

                // 询问是否再玩一次
                let again = read_input("再来一局？(y/n)");
                if again != "y" {
                    break 'game;
                }
                break 'guessing;  // 跳出猜测循环，回到游戏循环
            }
        }
    }

    println!("游戏结束！");
}
```

### 练习 5：九九乘法表

```rust
fn main() {
    for i in 1..=9 {
        for j in 1..=i {
            print!("{}×{}={:<3} ", j, i, i * j);
        }
        println!();
    }
}
```

输出：

```text
1×1=1
1×2=2   2×2=4
1×3=3   2×3=6   3×3=9
...
1×9=9   2×9=18  3×9=27  4×9=36  5×9=45  6×9=54  7×9=63  8×9=72  9×9=81
```

---

## 要点总结

### 函数核心要点

1. 用 `fn` 定义，参数必须声明类型
2. 返回值用 `-> 类型` 声明，最后的表达式（无分号）就是返回值
3. `return` 仅用于提前退出，正常返回不需要
4. 没有返回值等价于返回 `()`

### 语句 vs 表达式

1. **语句**执行操作不返回值（以分号结尾）
2. **表达式**求值并返回结果（无分号）
3. `{}` 块是表达式，值是最后一个无分号的行
4. 给表达式加分号就变成了语句，值被丢弃（变成 `()`）

### if 要点

1. 条件必须是 `bool`，不会自动类型转换
2. `if` 是表达式，可以赋值给变量
3. 分支类型必须一致

### 循环要点

1. `loop` — 无限循环，必须用 `break` 退出，可以返回值
2. `while` — 条件循环，条件为 false 自动退出
3. `for..in` — 最常用，遍历集合或范围
4. 优先使用 `for..in`，避免 `while` 遍历集合
5. `break` 退出循环，`continue` 跳过本次
6. 循环标签 `'label:` 解决嵌套循环控制

### match 要点

1. `match` 必须穷尽所有可能（exhaustive checking）
2. `_` 通配符匹配剩余所有情况
3. `|` 可以合并多个模式
4. `..=` 匹配范围
5. `if` 守卫为匹配分支附加条件
6. `match` 是表达式，可以赋值给变量

---

> **下一步：** 将学习 Rust 最核心的概念 —— **所有权 (Ownership)**。
> 这是 Rust 与其他语言最大的区别，也是 Rust 内存安全保证的基础。
