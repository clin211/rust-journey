//! 06. UTF-8 与字符串遍历：bytes / chars / char_indices / 字形簇
//!
//! 运行：cargo run --example 06_utf8_iteration
//!
//! 本例覆盖：
//! - String 内部存的是 **UTF-8 字节序列**, 不是 char 数组
//! - .len() 是字节数, 不是字符数
//! - 字节遍历：bytes()  / 字符遍历：chars() / 字符 + 字节索引：char_indices()
//! - 字形簇（grapheme cluster）：用户视角的"一个字符"
//! - 安全切片 vs 字节越界 panic
//! - 一些常见的字符串处理"陷阱"

#![allow(dead_code)]

// ============================================================================
// 1. 三个长度概念，必须分清
// ============================================================================
//
//   .len()                  字节数 (UTF-8 编码后)
//   .chars().count()        Unicode scalar 数 (一般称"代码点")
//   .graphemes(true).count() 字形簇数 (用户视角的"字符")  ← 需要 unicode-segmentation crate
//
// ASCII 字符占 1 字节; 中文一般 3 字节; emoji 多为 4 字节; 复杂 emoji 可能由多个代码点拼成。

fn three_lengths() {
    let s = "Hello, 世界 🦀";

    println!("  text                = {s:?}");
    println!("  .len()  (字节)       = {}", s.len());
    println!("  .chars().count()    = {}", s.chars().count());
    // 字形簇通常 = chars 数; 但 emoji 组合(👨‍👩‍👧‍👦) 等会大于 1 chars/簇

    // 看看 'Hello,' 占多少字节
    let small = "Hello,";
    println!("  ASCII 'Hello,' 字节  = {}", small.len());

    // 看看 '世' 一个字符占多少字节
    println!("  '世'.len_utf8()      = {}", '世'.len_utf8());
    println!("  '🦀'.len_utf8()      = {}", '🦀'.len_utf8());
}

// ============================================================================
// 2. 三种遍历：bytes / chars / char_indices
// ============================================================================

fn iteration_demo() {
    let s = "Aa中🦀";

    // bytes() 产 u8: 每次产生一个字节
    print!("  bytes:        ");
    for b in s.bytes() {
        print!("{b:#x} ");
    }
    println!();

    // chars() 产 char: 一个 Unicode scalar
    print!("  chars:        ");
    for c in s.chars() {
        print!("'{c}' ");
    }
    println!();

    // char_indices() 产 (字节起始索引, char)：处理切片时最有用
    println!("  char_indices:");
    for (i, c) in s.char_indices() {
        println!("    byte_idx={i}, char='{c}', utf8_len={}", c.len_utf8());
    }
}

// ============================================================================
// 3. 字符串切片：必须落在合法的字符边界
// ============================================================================
//
// `&s[i..j]` 的 i 和 j 是**字节**索引。
// 必须落在 UTF-8 字符边界, 否则 **运行时 panic**。

fn slice_safety() {
    let s = String::from("中文Rust");

    // 字符的字节起始位置: 0(中)、3(文)、6(R)、7(u)、8(s)、9(t)
    let chinese = &s[0..3];        // ✅ 0..3 是 "中"
    let english = &s[6..];         // ✅ 6.. 是 "Rust"
    println!("  '{s}'");
    println!("  &s[0..3] = {chinese:?}");
    println!("  &s[6..]  = {english:?}");

    // ❌ 下面这行会 panic, 因为字节 1 不是 char 边界
    // let bad = &s[0..1];

    // 安全做法: 先用 char_indices 找边界
    let prefix: &str = s
        .char_indices()
        .nth(2)                    // 取第 3 个字符的起始位置
        .map(|(i, _)| &s[..i])     // 切到那里
        .unwrap_or(&s);
    println!("  前 2 个字符 = {prefix:?}");

    // 安全做法 2: 标准库的 split_at_checked / floor_char_boundary（部分稳定）
    if let Some((lhs, rhs)) = s.split_at_checked(6) {
        println!("  split_at(6) = ({lhs:?}, {rhs:?})");
    }
    // ❌ 字节索引 1 不在边界
    // assert!(s.split_at_checked(1).is_none());
}

// ============================================================================
// 4. 反向遍历 + 倒着取字符
// ============================================================================

fn reverse_demo() {
    let s = "hello 🌍 world";

    // 字节反向 (危险, 字节流可能不是合法 UTF-8 的反向)
    let bytes_rev: Vec<u8> = s.bytes().rev().collect();
    let _ = bytes_rev;       // 拿到的字节序列不一定是合法字符串

    // 字符反向 (推荐)
    let chars_rev: String = s.chars().rev().collect();
    println!("  chars 反向 = {chars_rev:?}");
}

// ============================================================================
// 5. 字符筛选 / 转换
// ============================================================================
//
// chars 上的 char 自带很多 is_xxx / to_xxx 方法。

fn char_filter_demo() {
    let s = "Hello, World 123!";

    // 全部数字
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    println!("  digits  = {digits:?}");

    // 全部字母
    let letters: String = s.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    println!("  letters = {letters:?}");

    // 大写字母 → 小写, 其它保持
    let lowered: String = s
        .chars()
        .map(|c| if c.is_ascii_uppercase() { c.to_ascii_lowercase() } else { c })
        .collect();
    println!("  lowered = {lowered:?}");

    // 大小写翻转 (Unicode 安全)
    let toggled: String = s.chars()
        .map(|c| if c.is_lowercase() { c.to_ascii_uppercase() }
                 else if c.is_uppercase() { c.to_ascii_lowercase() }
                 else { c })
        .collect();
    println!("  toggle  = {toggled:?}");
}

// ============================================================================
// 6. 字形簇陷阱（grapheme cluster）
// ============================================================================
//
// 一个 emoji 可能由多个 char 拼成:
//   "👨‍👩‍👧‍👦" 包含 7 个 char (4 个人物 + 3 个 ZWJ 连接符)
//   但用户看上去就是"一个字符"。
//
// 标准库不提供字形簇 API（需要 `unicode-segmentation` crate）。
// 这里只用 chars 演示直观的"看上去是 1 个字符, 但有多个 chars"。

fn grapheme_pitfall() {
    let single_char = "🦀";
    let zwj_emoji = "👨‍👩‍👧‍👦";

    println!("  '{single_char}'");
    println!("    .len() bytes        = {}", single_char.len());
    println!("    .chars().count()    = {}", single_char.chars().count());

    println!("  '{zwj_emoji}'");
    println!("    .len() bytes        = {}", zwj_emoji.len());
    println!("    .chars().count()    = {}  ← 视觉上 1 个, 实际 7 个 char", zwj_emoji.chars().count());

    println!("  → 想数'用户看到的字符', 用 unicode-segmentation crate");
}

// ============================================================================
// 7. 综合例子：验证回文
// ============================================================================
//
// 真正"按字符"做处理的关键是 chars(), 而不是字节。

fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

fn palindrome_demo() {
    for s in [
        "level",
        "Hello",
        "A man a plan a canal Panama",
        "上海自来水来自海上",
        "racecar",
    ] {
        println!("  '{s}' -> palindrome? {}", is_palindrome(s));
    }
}

fn main() {
    println!("===== 1. 三个长度概念 =====");
    three_lengths();

    println!("\n===== 2. 三种遍历 =====");
    iteration_demo();

    println!("\n===== 3. 切片安全 =====");
    slice_safety();

    println!("\n===== 4. 反向遍历 =====");
    reverse_demo();

    println!("\n===== 5. 字符筛选 / 转换 =====");
    char_filter_demo();

    println!("\n===== 6. 字形簇陷阱 =====");
    grapheme_pitfall();

    println!("\n===== 7. 综合：回文判定 =====");
    palindrome_demo();

    println!("\n===== 要点回顾 =====");
    println!("· String / &str 内部是 UTF-8 字节, len() 是字节数");
    println!("· bytes / chars / char_indices 三种遍历对应三个层级");
    println!("· 切片必须落在字符边界, 否则 panic; 用 char_indices 找边界");
    println!("· chars 数 != 用户视觉字符数 (字形簇), 后者需第三方 crate");
}
