//! 05. &str vs String 全对照 + Cow<'a, str>
//!
//! 运行：cargo run --example 05_str_vs_string
//!
//! 本例覆盖：
//! - &str 是什么：胖指针 + UTF-8 借用视图
//! - String 是什么：拥有型, 可变, 堆分配
//! - 三种字符串"形态"：字面量 / 借用 / 拥有
//! - 转换：String → &str, &str → String, &String → &str
//! - Deref 强制转换：为什么 API 参数应该优先 &str
//! - Cow<'a, str>：在"借用 / 拥有"之间灵活切换

#![allow(dead_code)]

use std::borrow::Cow;

// ============================================================================
// 1. 三种字符串"形态" 一览
// ============================================================================
//
//   形态                类型           生命周期    所有权   存储位置
//   ─────────────────  ─────────────  ─────────  ───────  ──────────────
//   字符串字面量          &'static str   'static    借用       .rodata 段
//   String 切片视图       &str           借用      借用       和 String 同位置
//   String                String          ‒        拥有       堆
//
// String 在内存里 = Vec<u8>（24 字节胖指针 + 堆）
// &str 在内存里  = (ptr, len) 双字段（16 字节胖指针, 不带 cap）

fn three_forms() {
    let lit: &'static str = "I'm a literal";          // 编译进 .rodata
    let owned: String = String::from("I'm owned");    // 堆上分配
    let borrowed: &str = &owned[..6];                 // 借用 owned 的前 6 字节

    println!("  字面量 lit       : {lit}");
    println!("  拥有型 owned     : {owned}");
    println!("  借用视图 borrowed: {borrowed}");

    use std::mem::size_of;
    println!("  size_of::<&str>()      = {} B", size_of::<&str>());
    println!("  size_of::<String>()    = {} B", size_of::<String>());
    println!("  size_of::<&String>()   = {} B (一个普通指针 8B)", size_of::<&String>());
}

// ============================================================================
// 2. 互转
// ============================================================================

fn convert_demo() {
    // String → &str：自动 Deref，几乎无感
    let s: String = String::from("hello");
    let r: &str = &s;                      // 等价 s.as_str()
    println!("  String -> &str: {r}");

    // &str → String：要显式分配
    let r: &str = "world";
    let s1: String = r.to_string();        // 通用写法
    let s2: String = String::from(r);      // 等价
    let s3: String = r.to_owned();         // 偏底层
    println!("  &str -> String: {s1}, {s2}, {s3}");

    // &String → &str：自动 Deref，无感
    let owned = String::from("foo");
    let _: &str = &owned;                  // ✅ 自动转换
    let _: &str = owned.as_str();          // 显式版

    // String → Vec<u8>：拿走底层字节
    let s = String::from("hi🦀");
    let bytes: Vec<u8> = s.into_bytes();
    println!("  String -> Vec<u8>: {bytes:?}");

    // Vec<u8> → String：要校验 UTF-8
    let bytes = vec![72, 101, 108, 108, 111];
    match String::from_utf8(bytes) {
        Ok(s) => println!("  Vec<u8> -> String OK: {s}"),
        Err(e) => println!("  非法 UTF-8: {e}"),
    }
}

// ============================================================================
// 3. API 参数：&str 比 &String 好
// ============================================================================
//
// 用 &str 接收参数, 调用方传 String / &String / 字面量 / 切片 都行。
// 用 &String 接收参数, 只能传 &String。

fn param_demo() {
    fn good(s: &str) {                     // ✅ 接受所有形态
        println!("    收到 (&str): {s}");
    }
    fn bad(s: &String) {                   // ❌ 限制太死
        println!("    收到 (&String): {s}");
    }

    let owned = String::from("hello");
    good(&owned);                          // &String 自动转 &str
    good("literal");                       // 字面量本身就是 &str
    good(&owned[..3]);                     // 切片也是 &str

    bad(&owned);                           // 只能这一种
    // bad("literal");                     // ❌ expected `&String`, found `&str`
}

// ============================================================================
// 4. 何时用 String，何时用 &str
// ============================================================================
//
//   场景                                推荐
//   ─────────────────────────────────  ─────────────
//   函数只读参数                        &str
//   函数返回新拼接的字符串              String
//   函数返回输入参数的子串              &str  (生命周期标注)
//   长期持有 / 跨线程 / 序列化          String
//   配置常量                            &'static str
//   不确定就默认                        String

fn first_word(s: &str) -> &str {           // 返回输入的子串 → &str
    s.split_whitespace().next().unwrap_or("")
}

fn shouting(s: &str) -> String {           // 返回拼接的新字符串 → String
    format!("{}!!!", s.to_uppercase())
}

// ============================================================================
// 5. Cow<'a, str>：在"借用"和"拥有"之间灵活切换
// ============================================================================
//
// Cow = "Clone on Write"：
//
//   pub enum Cow<'a, T: ?Sized + 'a> where T: ToOwned {
//       Borrowed(&'a T),
//       Owned(<T as ToOwned>::Owned),
//   }
//
// 工程意义：函数返回类型可以是"原值"也可以是"修改后的副本"——只在必要时才分配。
//
// 经典场景：替换字符串里某些字符，但很多时候根本没东西要替换。

fn maybe_replace(s: &str) -> Cow<'_, str> {
    if s.contains('?') {
        // 真的要修改, 返回拥有型
        Cow::Owned(s.replace('?', "!"))
    } else {
        // 不需要修改, 借用原值即可（零分配）
        Cow::Borrowed(s)
    }
}

fn cow_demo() {
    let a = "Hello, World";
    let b = "Are you ok?";

    let r1 = maybe_replace(a);              // 不需修改, 直接借用
    let r2 = maybe_replace(b);              // 需要替换, 分配新字符串

    println!("  '{a}' -> {r1:?}");
    println!("  '{b}' -> {r2:?}");
    println!("  r1.is_borrowed = {}", matches!(r1, Cow::Borrowed(_)));
    println!("  r2.is_owned    = {}", matches!(r2, Cow::Owned(_)));

    // Cow<str> 实现了 Deref<Target = str>, 用起来和 &str 几乎一样
    println!("  r1.len = {}", r1.len());
    println!("  r2.starts_with(\"Are\") = {}", r2.starts_with("Are"));
}

// ============================================================================
// 6. 字符串字面量的生命周期
// ============================================================================
//
// 所有字面量 "..." 都是 &'static str —— 编译期写进二进制, 整个程序运行期间有效。
// 这就是为什么你能让 const / static 持有它们:

const APP_NAME: &str = "rust-journey";        // &'static str
static DESCRIPTION: &str = "a long-form Rust tutorial";

fn static_lifetime_demo() {
    let n = APP_NAME;
    let d = DESCRIPTION;
    println!("  APP_NAME    = {n}");
    println!("  DESCRIPTION = {d}");

    // 注意: &'static T 是"引用的生命周期是 static"，
    //      而不是"数据本身永远活着"。
    //      Box::leak() 可以把堆上的 String 提升为 &'static str（极少用到）。
}

fn main() {
    println!("===== 1. 三种字符串形态 =====");
    three_forms();

    println!("\n===== 2. 互转 =====");
    convert_demo();

    println!("\n===== 3. API 参数选型 =====");
    param_demo();

    println!("\n===== 4. 何时返回 String, 何时返回 &str =====");
    let sample = "hello world rust";
    println!("  first_word('{sample}')     = {:?}", first_word(sample));
    println!("  shouting('{sample}')       = {:?}", shouting(sample));

    println!("\n===== 5. Cow：按需分配 =====");
    cow_demo();

    println!("\n===== 6. 字面量与 'static =====");
    static_lifetime_demo();

    println!("\n===== 要点回顾 =====");
    println!("· &str 是借用; String 是拥有");
    println!("· 字面量 \"...\" 永远是 &'static str");
    println!("· 函数只读参数用 &str（Deref 兼容所有形态）");
    println!("· 返回输入子串用 &str (生命周期标注); 返回新拼接用 String");
    println!("· Cow<'a, str> 让你按需 0 分配 / 拷贝, 是优化字符串处理的利器");
}
