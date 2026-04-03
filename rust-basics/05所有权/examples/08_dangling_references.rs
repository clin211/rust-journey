use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 悬垂引用（Dangling Reference）：
//
//   定义：引用所指向的内存已经被释放，但引用本身还存在
//   结果：指向无效内存 → 未定义行为（C/C++ 的噩梦）
//
//   Rust 的解决方案：借用检查器（Borrow Checker）
//     在编译期静态分析所有引用的生命周期
//     确保"数据存活时间" ≥ "引用存活时间"
//     违反时：编译错误，程序根本无法运行
//
//   核心规则：返回引用时，引用所指向的数据必须在函数外部仍然有效
//     → 通常意味着：引用只能来自输入参数，不能来自函数内局部变量
//
//   生命周期注解（`'a`）：
//     当编译器无法自动推断引用的有效期时，需要程序员显式标注
//     这是 Rust 的高级概念，本文件只做入门铺垫
// ─────────────────────────────────────────────────────────────────────────

// ❌ 这个函数无法编译：返回了局部变量的引用（悬垂引用）
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s                 // ← 返回 s 的引用
// }                      // ← s 在这里 drop，堆内存释放，引用失效
//
// 编译错误：
//   error[E0106]: missing lifetime specifier
//   help: this function's return type contains a borrowed value, but there is
//   no value for it to be borrowed from
//
// Rust 拒绝编译的原因：
//   1. s 是函数内的局部变量
//   2. 函数结束时 s 离开作用域 → drop(s) → 堆内存释放
//   3. 返回的 &String 会指向已释放的内存 → 悬垂引用
//   4. 借用检查器在编译期检测到这个问题，拒绝编译

// ❌ 返回局部 String 的切片也不行：
// fn bad_slice() -> &str {
//     let s = String::from("hello world");
//     &s[..5]            // ← 切片指向 s 的堆内存
// }                      // ← s drop，堆内存释放，切片失效
//
// 根本原因相同：切片的数据来自局部变量，函数结束后数据已释放

// ✅ 正确做法一：返回所有权（不返回引用）
fn no_dangle() -> String {
    String::from("hello")
    // 返回的是 String 本身（所有权转移），不是引用
    // 调用方接管所有权，数据不会被释放
}

// ✅ 正确做法二：从输入参数借用后返回
// 返回的 &str 来自参数 s，生命周期与 s 绑定
// 只要调用方持有的数据有效，返回的引用就有效
fn first_word(s: &str) -> &str {
    // 编译器自动推断：返回值的生命周期 = 参数 s 的生命周期
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// ✅ 正确做法三（进阶）：显式生命周期注解
// 当有多个引用参数时，编译器需要知道返回值的生命周期与哪个参数绑定
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 'a 表示：返回值的有效期 = s1 和 s2 中较短的那个
    // 告诉编译器：只要 s1 和 s2 都有效，返回值就有效
    if s1.len() >= s2.len() { s1 } else { s2 }
}

fn main() {
    println!("{}", "=== 悬垂引用与借用检查器 ===".green().bold());

    println!("\n【什么是悬垂引用？】");
    println!("  悬垂引用 = 引用指向的内存已经释放，引用本身还存在");
    println!("  C 语言中：返回局部变量地址 → 未定义行为（UB）");
    println!("  Rust 中：借用检查器在编译期检测，直接拒绝编译");
    println!("  结果：Rust 程序运行时永远不会出现悬垂引用");

    println!("\n1、悬垂引用：函数返回局部变量的引用");
    println!("  fn dangle() -> &String {{");
    println!("      let s = String::from(\"hello\");");
    println!("      &s   // ← 函数结束时 s 被 drop，&s 指向释放的内存");
    println!("  }}");
    println!("  → 编译错误：this function's return type contains a borrowed value,");
    println!("              but there is no value for it to be borrowed from");
    println!("  小结：局部变量的引用不能逃出创建它的作用域");

    println!("\n2、正确做法一：返回所有权，不返回引用");
    let text = no_dangle();
    println!("text = {text}");
    println!("  返回 String 而非 &String，调用方接管所有权，数据安全");
    println!("  经验法则：函数内创建的数据 → 返回所有权（不是引用）");

    println!("\n3、正确做法二：从输入参数借用并返回（最常见）");
    let sentence = String::from("hello rust world");
    let word = first_word(&sentence);
    println!("first_word = {word}");
    println!("sentence 仍然有效: {sentence}"); // sentence 的数据还在
    println!("  返回值来自参数 s，s 的数据在调用方（sentence）中，生命周期由调用方控制");
    println!("  编译器自动推断：fn first_word(s: &str) -> &str 等价于");
    println!("                  fn first_word<'a>(s: &'a str) -> &'a str");
    println!("  意思：返回的引用最多与 s 一样长");

    println!("\n4、[进阶] 生命周期注解：当编译器需要明确指引时");
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longer(&s1, &s2);
        // 在 s2 的作用域内使用 result，此时 s1 和 s2 都有效
        println!("longer = {result}");
    }
    // result 不能在这里使用：s2 已经 drop，result 可能来自 s2
    // println!("result = {result}"); // ← 如果取消注释，编译错误

    println!("  longer<'a>(s1: &'a str, s2: &'a str) -> &'a str");
    println!("  'a 是生命周期参数，表示：返回值有效期 ≤ min(s1 有效期, s2 有效期)");
    println!("  生命周期注解不改变引用的实际生命周期，只是告诉编译器它们之间的关系");

    println!("\n5、借用检查器工作原理直觉");
    println!("  借用检查器给每个引用分配一个生命周期区间 [创建点, 最后使用点]");
    println!("  检查规则：引用的区间必须 ⊆ 被引用数据的存活区间");
    println!("  如果违反（引用比数据活得更长）→ 编译错误");
    println!("  这一切在编译期完成，零运行时开销");

    println!("\n6、设计经验：如何避免悬垂引用");
    println!("  ✅ 函数内创建数据 → 返回所有权（String、Vec 等）");
    println!("  ✅ 需要返回引用 → 引用来自输入参数");
    println!("  ✅ 多个输入引用 → 用生命周期注解说明返回值与哪个参数绑定");
    println!("  ❌ 不要尝试返回局部变量的引用 → 必然是悬垂引用");
}
