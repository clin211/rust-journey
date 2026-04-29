//! 04. match 表达式：基础、穷尽性、通配符、表达式语义
//!
//! 运行：cargo run --example 04_match_basics
//!
//! 本例覆盖：
//! - match 是表达式（可以赋值给变量）
//! - 穷尽性检查：必须覆盖全部变体
//! - `_` 通配符 / `name` 兜底变量
//! - 多语句臂用 `{}` 包起来
//! - 与 if/else 链的对比

#![allow(dead_code)]

// ============================================================================
// 1. match 是表达式：能被赋值
// ============================================================================
//
// 在 Rust 里，match 不是"控制流语句"，而是一个**表达式**。
// 这意味着：每个臂的最后一个表达式就是它的"值"，
// 整个 match 的值就是被命中那个臂的值，可以直接 `let x = match ...`。
//
// 这一点和 Java/Go 的 switch 不同——它们都不能"返回值"。

#[derive(Debug, Clone, Copy)]
enum Coin {
    Penny,    // 1 分
    Nickel,   // 5 分
    Dime,     // 10 分
    Quarter,  // 25 分
}

fn cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// ============================================================================
// 2. 穷尽性检查：让你绝不会漏写一个变体
// ============================================================================
//
// 这是 Rust match 最让人感动的特性之一：
//
//   编译器会检查 match 是否覆盖了 enum 的所有变体；
//   只要漏一个，就编译失败。
//
// 试试：把下面的 Coin::Quarter 那一臂注释掉，再 cargo build，看到错误：
//
//   error[E0004]: non-exhaustive patterns: `Coin::Quarter` not covered
//
// 这意味着：
//   未来你给 Coin 新增一个变体（比如 HalfDollar），所有写过 match 的地方
//   都会编译报错——你不会"安静地遗漏"任何分支。

// ============================================================================
// 3. `_` 通配符 / 命名兜底变量
// ============================================================================
//
// 当你确实不关心剩下的变体时，可以用 `_` 把它们一次性吃掉。
// 但要小心：`_` 会让编译器无法在新增变体时提醒你！业务代码慎用。

fn coin_class(c: Coin) -> &'static str {
    match c {
        Coin::Penny | Coin::Nickel => "小面额",     // 多模式 |
        Coin::Dime => "中等面额",
        _ => "大面额",                               // 兜底，所有剩余变体进这里
    }
}

// 命名兜底：用一个变量名捕获"所有其它情况"，比 _ 多一个值可用
fn describe_number(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        other => format!("number {other}"),       // ← `other` 是个绑定变量
    }
}

// ============================================================================
// 4. 多语句臂：用 `{}` 包起来
// ============================================================================
//
// 每一臂可以是单个表达式，也可以是一个代码块 `{ ... }`，最后一个表达式作为值。

fn handle(c: Coin) -> u32 {
    match c {
        Coin::Penny => {
            println!("[log] 找到 1 分钱！");
            1
        }
        Coin::Nickel => {
            println!("[log] 找到 5 分钱");
            5
        }
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// ============================================================================
// 5. match vs if/else if 链
// ============================================================================
//
// 写一个等价的 if/else 版本，对比能看出 match 的优势：
// - 更紧凑（不用反复写比较的左操作数）
// - 编译器能做穷尽性检查（if/else 不行）
// - 模式匹配能力远超 == 比较（解构、范围、守卫，下章展开）

fn cents_via_if(c: &Coin) -> u32 {
    if matches!(c, Coin::Penny) {
        1
    } else if matches!(c, Coin::Nickel) {
        5
    } else if matches!(c, Coin::Dime) {
        10
    } else {
        25 // ⚠️ 只能写一个 fallback；漏处理变体编译器不会提醒
    }
}

// ============================================================================
// 6. 把 match 的结果直接拿来用
// ============================================================================

fn shipping_fee(weight_grams: u32) -> u32 {
    // match 的结果可以直接作为 return 表达式
    match weight_grams {
        0..=500 => 5,
        501..=1000 => 10,
        1001..=5000 => 25,
        _ => 50,
    }
}

// 把 match 的结果作为函数返回值（最常见的写法）
fn http_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500..=599 => "Server Error",
        _ => "Unknown",
    }
}

// ============================================================================
// 7. match 配合 enum + 数据：基础解构
// ============================================================================
//
// 第 06 章会展开"解构"，这里先尝个鲜：当变体带数据时，
// 在模式里给数据起个名字，臂体里就能用这个名字。

#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
    Scroll(i32),
    KeyPress(char),
}

fn describe(ev: &Event) -> String {
    match ev {
        Event::Click { x, y } => format!("点击坐标 ({x}, {y})"),
        Event::Scroll(dy) => format!("滚动 {dy} 像素"),
        Event::KeyPress(c) => format!("按键 '{c}'"),
    }
}

fn main() {
    println!("===== 1. match 作为表达式 =====");
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    let mut total = 0;
    for c in coins {
        let v = cents(c);            // ← match 的值可以直接 let
        println!("  {v} cents");
        total += v;
    }
    println!("总额: {total} cents");

    println!("\n===== 2. 通配符 _ + 命名兜底 =====");
    for c in [Coin::Penny, Coin::Dime, Coin::Quarter] {
        println!("  {:?} 属于 {}", c, coin_class(c));
    }

    for n in [0, 1, 2, 3, 100] {
        println!("  {n} -> {}", describe_number(n));
    }

    println!("\n===== 3. 多语句臂 =====");
    println!("handle(Penny) = {}", handle(Coin::Penny));
    println!("handle(Dime)  = {}", handle(Coin::Dime));

    println!("\n===== 4. match vs if/else 等价对比 =====");
    let dime = Coin::Dime;
    println!("cents_via_if(&Dime) = {}", cents_via_if(&dime));
    println!("cents(Dime)         = {}", cents(dime));

    println!("\n===== 5. 范围模式（预告 05） =====");
    for w in [200, 800, 3000, 8000] {
        println!("  {w}g 邮费 = {}", shipping_fee(w));
    }

    println!("\n===== 6. HTTP 状态码描述 =====");
    for code in [200, 404, 500, 599, 999] {
        println!("  {code} {}", http_text(code));
    }

    println!("\n===== 7. enum + 数据 =====");
    let events = [
        Event::Click { x: 10, y: 20 },
        Event::Scroll(-5),
        Event::KeyPress('A'),
    ];
    for ev in &events {
        println!("  {}", describe(ev));
    }

    println!("\n===== 要点回顾 =====");
    println!("· match 是表达式，可赋值，可作返回值");
    println!("· 编译器强制穷尽性检查；新增变体会逼你来更新所有 match");
    println!("· `_` 通配 / `name` 兜底，业务代码尽量不要滥用 `_`");
}
