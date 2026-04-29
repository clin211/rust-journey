//! 09. Result<T, E>：与 Option 同源的"成功/失败"枚举
//!
//! 运行：cargo run --example 09_result_intro
//!
//! 本例覆盖：
//! - Result 的本质（也是个 enum）
//! - Result 与 Option 的关系
//! - 常用 API：is_ok / map / map_err / and_then / or_else / unwrap_or
//! - `?` 运算符：错误传播
//! - 把 Option 和 Result 互相转换
//!
//! 完整的错误处理（自定义 Error、From、anyhow / thiserror）会放到第 10 章。
//! 本节只把 Result 作为"枚举的另一个超有用的实例"先讲清楚。

#![allow(dead_code, unused_variables)]
// 注：本文件作为 Result API 的教学样例, 必须在已知是 Ok / Err 的字面值上
// 演示 unwrap() / unwrap_or() 等的行为, 因此 allow 相关 lint。
#![allow(clippy::unnecessary_literal_unwrap)]

use std::num::ParseIntError;

// ============================================================================
// 1. Result<T, E> 是什么
// ============================================================================
//
// 标准库的定义其实就这几行：
//
//   pub enum Result<T, E> {
//       Ok(T),
//       Err(E),
//   }
//
// 与 Option<T> 的关系一目了然：
//
//   Option<T>     ─→  "可能没值"（None / Some(T)）
//   Result<T, E>  ─→  "可能失败" 并附带 E 描述失败原因
//
// 凡是函数"可能失败"的场景，都返回 Result，比如：
//   parse / read_file / connect / ... 标准库里几乎所有 I/O 函数都是 Result。

fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为 0".to_string())
    } else {
        Ok(a / b)
    }
}

// ============================================================================
// 2. 处理 Result：match 三件套
// ============================================================================

fn handle_basic() {
    let cases = [(10, 2), (10, 0), (-9, 3)];
    for (a, b) in cases {
        match safe_div(a, b) {
            Ok(v) => println!("  {a} / {b} = {v}"),
            Err(e) => println!("  {a} / {b} 失败: {e}"),
        }
    }
}

// ============================================================================
// 3. 常用 API
// ============================================================================
//
// 这些方法和 Option 上同名 / 类似，但要多处理 Err 这一边。

fn api_demo() {
    let ok: Result<i32, &str> = Ok(7);
    let err: Result<i32, &str> = Err("boom");

    // is_ok / is_err
    println!("  Ok(7).is_ok() = {}", ok.is_ok());
    println!("  Err.is_err()  = {}", err.is_err());

    // unwrap / expect / unwrap_or / unwrap_or_else / unwrap_or_default
    println!("  Ok(7).unwrap()              = {}", ok.unwrap());
    println!("  Ok(7).unwrap_or(0)          = {}", ok.unwrap_or(0));
    println!("  Err.unwrap_or(0)            = {}", err.unwrap_or(0));
    println!("  Err.unwrap_or_else(|e| ..)  = {}", err.unwrap_or_else(|e| {
        println!("    [闭包看到的 e = {e:?}]");
        -1
    }));

    // map：变换 Ok 这一侧
    let mapped: Result<i32, &str> = ok.map(|n| n * 10);
    println!("  Ok(7).map(*10)              = {mapped:?}");

    // map_err：变换 Err 这一侧（保留 Ok 不变）
    let mapped_err: Result<i32, String> = err.map_err(|e| format!("converted: {e}"));
    println!("  Err.map_err(...)            = {mapped_err:?}");

    // and_then：Ok 时继续往下，Err 时直接短路
    let chain = safe_div(20, 2).and_then(|v| safe_div(v, 0));
    println!("  20/2 -> /0                  = {chain:?}");

    // or_else：Err 时尝试备选
    let recovered = safe_div(10, 0).or_else(|_| safe_div(10, 5));
    println!("  10/0 fallback 10/5          = {recovered:?}");

    // .ok() / .err() 把 Result 拆成两半的 Option
    println!("  Ok(7).ok()                  = {:?}", ok.ok());
    println!("  Err.err()                   = {:?}", err.err());
}

// ============================================================================
// 4. ? 运算符：错误传播
// ============================================================================
//
// 真实代码里大部分时候"出错就直接往上抛"。
// 写一堆 match 太啰嗦，Rust 提供了 `?`：
//
//   let v = expr?;
//
// 等价于：
//
//   let v = match expr {
//       Ok(v) => v,
//       Err(e) => return Err(From::from(e)),  // ← 注意：会自动 From 转换
//   };
//
// 函数签名的返回值必须也是 Result（错误类型 E 兼容）才能用 `?`。

fn parse_and_double(a: &str) -> Result<i32, ParseIntError> {
    let n: i32 = a.parse()?;                     // parse 返回 Result，? 失败就 return
    Ok(n * 2)
}

fn parse_and_sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

// ============================================================================
// 5. ? 配合不同错误类型：From / Into
// ============================================================================
//
// 当不同步骤的错误类型不一样时，需要一个统一的 Error 类型，并实现 From。
// 这是错误处理的核心用法，下一章会展开 thiserror / anyhow。
// 这里先认识一下这个机制。

#[derive(Debug)]
enum MyError {
    ParseError(ParseIntError),
    Logic(String),
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::ParseError(e)
    }
}

fn pipeline(a: &str, b: &str) -> Result<i32, MyError> {
    let x: i32 = a.parse()?;                     // ParseIntError -> MyError，靠 From
    let y: i32 = b.parse()?;
    if y == 0 {
        return Err(MyError::Logic("除数不能为 0".into()));
    }
    Ok(x / y)
}

// ============================================================================
// 6. Option <-> Result 互转
// ============================================================================

fn opt_to_result_demo() {
    let some_v: Option<i32> = Some(42);
    let none_v: Option<i32> = None;

    // .ok_or() / .ok_or_else()：附加错误信息后转成 Result
    let r1: Result<i32, &str> = some_v.ok_or("缺值");
    let r2: Result<i32, &str> = none_v.ok_or("缺值");
    println!("  Some.ok_or  -> {r1:?}");
    println!("  None.ok_or  -> {r2:?}");

    // .ok()：丢弃错误信息
    let r: Result<i32, &str> = Ok(7);
    let o: Option<i32> = r.ok();
    println!("  Result.ok() -> {o:?}");

    // .err()：拿到 Err 这一边的 Option
    let r: Result<i32, &str> = Err("oops");
    println!("  Result.err()-> {:?}", r.err());
}

// ============================================================================
// 7. 实战：把字符串列表逐个解析为 i32
// ============================================================================
//
// 这是 collect 的一个经典 idiom：
// 把 Vec<&str> 里的每一个尝试 parse 成 i32，遇到任何一个失败立刻整体失败。

fn parse_all(strs: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    // collect 可以收集 Result<Vec<T>, E>：
    // - 全部 Ok 时收集成 Ok(Vec)
    // - 任何一个 Err 立刻短路，整体 Err
    strs.iter().map(|s| s.parse::<i32>()).collect()
}

fn main() {
    println!("===== 1. safe_div + match =====");
    handle_basic();

    println!("\n===== 2. 常用 API =====");
    api_demo();

    println!("\n===== 3. ? 简单错误传播 =====");
    println!("  parse_and_double('10') = {:?}", parse_and_double("10"));
    println!("  parse_and_double('xx') = {:?}", parse_and_double("xx"));
    println!("  parse_and_sum('3','4') = {:?}", parse_and_sum("3", "4"));
    println!("  parse_and_sum('3','x') = {:?}", parse_and_sum("3", "x"));

    println!("\n===== 4. ? + From 跨错误类型 =====");
    println!("  pipeline('10','2') = {:?}", pipeline("10", "2"));
    println!("  pipeline('10','0') = {:?}", pipeline("10", "0"));
    println!("  pipeline('xx','2') = {:?}", pipeline("xx", "2"));

    println!("\n===== 5. Option <-> Result 互转 =====");
    opt_to_result_demo();

    println!("\n===== 6. collect 短路效应 =====");
    println!("  parse_all(['1','2','3']) = {:?}", parse_all(&["1", "2", "3"]));
    println!("  parse_all(['1','x','3']) = {:?}", parse_all(&["1", "x", "3"]));

    println!("\n===== 要点回顾 =====");
    println!("· Result<T,E> 也是 enum，本质和 Option 一脉相承");
    println!("· Ok 这一侧用 map / and_then 串；Err 这一侧用 map_err / or_else");
    println!("· ? 让 Err 短路 + From 自动转换，是写'多步可能失败'最干净的方式");
    println!("· Option ←→ Result 之间用 ok / err / ok_or 自由互转");
    println!("· 第 10 章会深入：自定义 Error、thiserror、anyhow");
}
