//! 04. String 基础：创建、追加、拼接、format!、replace、split、parse
//!
//! 运行：cargo run --example 04_string_basics
//!
//! 本例覆盖：
//! - 三种创建方式：String::new / String::from / "...".to_string()
//! - push_str / push / + / format!
//! - replace / split / trim / to_uppercase / to_lowercase / contains / starts_with
//! - parse：从字符串到数值
//! - String 是带 UTF-8 的可变 Vec<u8>

#![allow(dead_code)]
// 注：vec![] 是为了对比演示 String / Vec 的相关性
#![allow(clippy::useless_vec)]

// ============================================================================
// 1. 创建 String
// ============================================================================
//
//   方式                            适用
//   ─────────────────────────────  ────────────────────
//   String::new()                   空字符串, 后续 push_str
//   String::from("hi")              已有 &str → String
//   "hi".to_string()                同上, 等价写法
//   "hi".to_owned()                 同上, 偏底层风格
//   String::with_capacity(n)        预分配, 已知大约长度
//
// String 在内存里是 Vec<u8>，永远存合法 UTF-8。

fn create_demo() {
    let a = String::new();
    let b = String::from("hello");
    let c = "world".to_string();
    let d = String::with_capacity(64);

    println!("  a = {a:?}, len={}", a.len());
    println!("  b = {b:?}, len={}", b.len());
    println!("  c = {c:?}, len={}", c.len());
    println!("  d (cap=64) = {d:?}, cap={}", d.capacity());

    // 通过 collect 从字符迭代器创建
    let e: String = ('a'..='e').collect();
    println!("  collect a..=e = {e:?}");
}

// ============================================================================
// 2. 追加：push_str / push
// ============================================================================
//
//   push_str(&str)  把整段字符串追加到末尾
//   push(char)      追加单个字符
//
// 都是 in-place 修改, 原变量必须 mut。

fn append_demo() {
    let mut s = String::from("hello");
    s.push_str(", world");      // 追加 &str
    s.push('!');                // 追加 char
    println!("  push 后: {s:?}");

    // push_str 接受任何 &str（包括字面量、&String 自动 deref）
    let extra = String::from(" 🦀");
    s.push_str(&extra);
    s.push_str(" Rust");
    println!("  最终: {s:?}");
}

// ============================================================================
// 3. 拼接：+、format!、concat、join
// ============================================================================
//
// `+` 看着方便，但有几个坑要注意：
// - 第一个操作数必须是 String（被消费）
// - 第二个必须是 &str（不能是 String）
// - 返回值是新 String
//
// 推荐**默认就用 format!**——它能拼任意类型, 语义清晰, 不消费原值。

fn concat_demo() {
    // + 操作符
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + ", " + &s2 + "!";   // s1 被消费, s2 必须 &
    println!("  + 拼接: {s3}");
    // println!("{s1}");              // ❌ s1 已被 move 进 + 里

    // format! 宏：最推荐, 不消费任何参数
    let a = String::from("Hello");
    let b = String::from("World");
    let s = format!("{a}, {b}!");
    println!("  format!: {s} （{a} 和 {b} 都还能用）");

    // join: 多段一起接
    let parts = vec!["rust", "is", "fun"];
    let joined: String = parts.join(" ");
    println!("  join \" \": {joined}");

    // concat: 把 Vec<&str> 直接连在一起 (无分隔)
    let pieces = ["abc", "def", "ghi"];
    let concat: String = pieces.concat();
    println!("  concat:  {concat}");
}

// ============================================================================
// 4. 替换 / 切分 / 修剪
// ============================================================================

fn transform_demo() {
    // replace: 全部替换 / replacen 限定次数
    let s = "hello world hello";
    println!("  replace        = {:?}", s.replace("hello", "HEY"));
    println!("  replacen(1)    = {:?}", s.replacen("hello", "HEY", 1));

    // split / splitn / rsplit / split_whitespace
    let csv = "a,b,c,d,e";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("  split ','      = {parts:?}");
    let two: Vec<&str> = csv.splitn(2, ',').collect();
    println!("  splitn(2)      = {two:?}");

    let line = "  hello   world  rust  ";
    let words: Vec<&str> = line.split_whitespace().collect();
    println!("  split_whitespace = {words:?}");

    // trim / trim_start / trim_end
    let padded = "   hi   ";
    println!("  trim           = {:?}", padded.trim());
    println!("  trim_start     = {:?}", padded.trim_start());
    println!("  trim_end       = {:?}", padded.trim_end());

    // 大小写
    let s = "Hello, World!";
    println!("  to_uppercase   = {}", s.to_uppercase());
    println!("  to_lowercase   = {}", s.to_lowercase());
}

// ============================================================================
// 5. 查询：contains / starts_with / ends_with / find
// ============================================================================

fn query_demo() {
    let s = "Rust is a systems programming language";

    println!("  contains \"systems\"     = {}", s.contains("systems"));
    println!("  starts_with \"Rust\"     = {}", s.starts_with("Rust"));
    println!("  ends_with \"language\"   = {}", s.ends_with("language"));

    // find: 返回第一次出现的字节索引
    println!("  find(\"is\")             = {:?}", s.find("is"));
    println!("  rfind(\"a\")             = {:?}", s.rfind('a'));

    // matches: 所有出现位置
    let count = s.matches('s').count();
    println!("  matches('s').count()   = {count}");
}

// ============================================================================
// 6. 解析：parse —— 从字符串到数值/类型
// ============================================================================
//
// `parse` 返回 `Result<T, E>`, 你需要给出"目标类型"（让编译器知道要解析成啥）。

fn parse_demo() {
    // 显式类型注解
    let n: i32 = "42".parse().unwrap();
    println!("  '42'.parse::<i32>() = {n}");

    // turbofish 写法
    let n = "3.14".parse::<f64>().unwrap();
    println!("  turbofish f64        = {n}");

    // 失败时 parse 返回 Err
    let bad = "abc".parse::<i32>();
    println!("  'abc'.parse::<i32>() = {bad:?}");

    // 实际工程中: 用 .ok() 转成 Option, 配合 unwrap_or
    let port: u16 = std::env::var("PORT").ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    println!("  PORT (默认 8080)     = {port}");
}

// ============================================================================
// 7. 让 String 与字符串字面量、&String 自由切换
// ============================================================================
//
// String 实现了 Deref<Target = str>, 所以:
//   - &String 在需要 &str 的地方会自动转成 &str
//   - 反过来不会自动: 要 String 时不能直接传 &str
//
// 这就是为什么: API 参数应优先用 &str.

fn coercion_demo() {
    fn greet(name: &str) {
        println!("  hello {name}");
    }

    let owned = String::from("alice");
    greet(&owned);                    // &String → &str (Deref)
    greet("bob");                     // 字面量直接是 &str
    greet(&owned[..3]);               // 字符串切片也是 &str

    // 反向: 想要 String 时不能传 &str
    fn want_string(s: String) {
        println!("  收到 String: {s}");
    }
    want_string("hi".to_string());    // 显式转
    want_string(String::from("hello"));
}

// ============================================================================
// 8. 容量与内存
// ============================================================================
//
// String 在内存上和 Vec<u8> 完全一样:
//   ┌──────────┬──────────┬──────────┐
//   │   ptr    │   len    │   cap    │
//   └──────────┴──────────┴──────────┘
//
// 区别只在于:
//   - String 永远保证 UTF-8 合法
//   - len 是字节数, 不是字符数 (字符可能占 1-4 字节)

fn memory_demo() {
    use std::mem::size_of;
    println!("  size_of::<String>()  = {} B (栈上)", size_of::<String>());
    println!("  size_of::<Vec<u8>>() = {} B (一样)", size_of::<Vec<u8>>());

    let s = String::from("Hello, 世界 🦀");
    println!("  s.len() (字节)       = {}", s.len());
    println!("  s.chars().count()    = {}", s.chars().count());
    println!("  分别为何: 中文一个字符 3 字节, emoji 4 字节");
}

fn main() {
    println!("===== 1. 创建 =====");
    create_demo();

    println!("\n===== 2. 追加 =====");
    append_demo();

    println!("\n===== 3. 拼接：+ vs format! =====");
    concat_demo();

    println!("\n===== 4. 变换 =====");
    transform_demo();

    println!("\n===== 5. 查询 =====");
    query_demo();

    println!("\n===== 6. 解析 parse =====");
    parse_demo();

    println!("\n===== 7. 与 &str 自由切换 =====");
    coercion_demo();

    println!("\n===== 8. 容量与内存 =====");
    memory_demo();

    println!("\n===== 要点回顾 =====");
    println!("· String = 带 UTF-8 保证的 Vec<u8>");
    println!("· 拼接首选 format!（不消费, 任意类型）");
    println!("· 函数参数用 &str, 不用 &String");
    println!("· len() 是字节数; 想要字符数用 chars().count()");
    println!("· parse::<T>() 返回 Result, 失败要处理");
}
