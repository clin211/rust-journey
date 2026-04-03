use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// NLL（Non-Lexical Lifetimes，非词法生命周期）：
//
//   Rust 2018 edition 引入，Rust 2021 edition 默认开启
//
//   旧规则（词法生命周期）：借用在声明它的花括号 } 处结束
//   新规则（NLL）：借用在"最后一次使用"处结束，而不是花括号处
//
//   NLL 的影响：
//     → 更多合法代码得以通过编译（减少不必要的借用冲突报错）
//     → 程序员可以在"用完不可变借用"后立刻开始可变借用
//     → 无需额外引入 {} 来手动限定借用范围
//
//   生命周期直觉（Lifetime Intuition）：
//     每个引用都有一个生命周期区间：[创建点, 最后一次使用点]
//     借用规则检查的是这些区间是否存在非法重叠
//
//   生命周期注解（'a）的必要性：
//     多个引用参数时，编译器不知道返回值的生命周期与哪个参数绑定
//     程序员用 'a 显式描述这种关联关系
//     注解只是"约束关系的声明"，不会改变运行时行为
// ─────────────────────────────────────────────────────────────────────────

// 演示生命周期注解：返回值的生命周期与参数绑定
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a 说明：返回值的有效期 = min(x 的有效期, y 的有效期)
    // 没有 'a 的话，编译器报错：
    //   "missing lifetime specifier"
    //   "this function's return type contains a borrowed value,
    //    but the signature does not say whether it is borrowed from x or y"
    if x.len() >= y.len() { x } else { y }
}

// 只有一个引用输入时，编译器自动推断（生命周期省略规则）
fn first_word(s: &str) -> &str {
    // 等价于 fn first_word<'a>(s: &'a str) -> &'a str
    // 编译器自动插入 'a，无需程序员手写
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    println!("{}", "=== 借用范围与 NLL 直觉 ===".green().bold());

    println!("\n【NLL 核心思想】");
    println!("  旧规则：let r = &x; ... }} ← 借用在这里结束");
    println!("  NLL：   let r = &x; println!(r); ← 借用在最后一次使用后即结束");
    println!("  NLL 让借用检查更精确，减少不必要的「假冲突」");

    println!("\n1、NLL 让「用完就放」的模式自然成立");
    let mut text = String::from("hello");
    let r1 = &text;              // r1 创建
    println!("  r1 = {r1}");    // r1 最后一次使用 → r1 的借用到此结束
    // NLL 分析：r1 在上面这行之后不再使用，借用生命周期 = [let r1, println!(r1)]
    let r2 = &mut text;          // ✅ r1 已结束，现在可以可变借用
    r2.push_str(" world");
    println!("  r2 = {r2}");
    println!("  小结：不可变借用在最后一次使用后结束，之后可以立刻开始可变借用");

    println!("\n2、借用区间重叠 → 编译错误");
    // ❌ 错误情况（如果取消注释）：
    // let mut note = String::from("Rust");
    // let read_ref = &note;
    // let write_ref = &mut note;   // ← 可变借用
    // println!("read_ref = {read_ref}"); // ← 此行让 read_ref 的区间延伸到这里
    // write_ref.push_str(" ownership"); // ← write_ref 区间与 read_ref 重叠
    //
    // 编译错误：cannot borrow `note` as mutable because it is also borrowed as immutable
    // NLL 分析：read_ref 的区间=[let read_ref, println!(read_ref)]
    //           write_ref 的区间=[let write_ref, push_str]
    //           两个区间重叠 → 违反借用规则 C

    // ✅ 正确：调整语句顺序，使区间不重叠
    let mut note = String::from("Rust");
    let read_ref = &note;
    println!("  先用完只读借用 → {read_ref}");  // read_ref 区间在这里结束
    let write_ref = &mut note;                   // ✅ read_ref 已结束
    write_ref.push_str(" ownership");
    println!("  再写入 → {write_ref}");
    println!("  小结：很多借用冲突只需调整语句顺序（把读用完再写）即可解决");

    println!("\n3、可变借用活跃期间，不能创建不可变借用");
    // ❌ 错误情况（如果取消注释）：
    // let mut title = String::from("borrow");
    // let write_ref = &mut title;
    // let read_ref = &title;       // ← 可变借用还未结束
    // println!("{write_ref} {read_ref}");
    //
    // 编译错误：cannot borrow `title` as immutable because it is also borrowed as mutable

    // ✅ 正确：先读后写
    let mut title = String::from("borrow");
    let read_ref = &title;
    println!("  先读 → {read_ref}");            // read_ref 区间结束
    let write_ref = &mut title;
    write_ref.push_str(" checker");
    println!("  再写 → {write_ref}");
    println!("  规则总结：读写之间必须时间上不重叠");

    println!("\n4、用花括号显式限定借用范围（NLL 前的老写法，仍然有效）");
    let mut data = String::from("hello");
    {
        let temp_ref = &mut data;
        temp_ref.push_str(" from block");
        println!("  在块内修改: {temp_ref}");
    } // temp_ref 借用在花括号结束时明确结束
    println!("  块外访问: {data}");
    println!("  小结：花括号仍然有效，适合需要明确标示借用范围时使用");

    println!("\n5、生命周期直觉：把借用看成区间");
    println!("  每个引用有一个存活区间 [创建行号, 最后使用行号]");
    println!("  借用检查规则：");
    println!("    &T 区间之间可以完全重叠（多个不可变借用并存）");
    println!("    &mut T 区间与任何其他借用区间不能重叠");
    println!("  编译器做的就是：检查所有区间对，找出非法重叠");

    println!("\n6、生命周期注解：多个引用参数时的必要声明");
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("xyz");
        let result = longest(&s1, &s2);
        // result 在 s2 的作用域内使用，此时两个参数都有效
        println!("  longest = {result}");
    }
    // ❌ 如果在 s2 作用域外使用 result，编译错误：
    // let s2 = String::from("xyz");
    // let result = longest(&s1, &s2);
    // drop(s2);                  // ← s2 释放
    // println!("{result}");      // ← result 可能指向 s2，已无效

    println!("  longest<'a>(x: &'a str, y: &'a str) -> &'a str");
    println!("  'a 的含义：返回值有效期 ≤ min(x 有效期, y 有效期)");
    println!("  这个约束由编译器在调用点验证，零运行时开销");

    println!("\n7、生命周期省略规则（Lifetime Elision Rules）");
    println!("  当函数签名满足特定模式时，编译器自动推断生命周期：");
    println!("  规则1：每个引用参数得到独立的生命周期参数");
    println!("  规则2：只有一个引用参数时，其生命周期赋给所有输出引用");
    println!("  规则3：有 &self 或 &mut self 时，self 的生命周期赋给所有输出引用");
    let sentence = String::from("hello rust world");
    let word = first_word(&sentence); // 编译器自动推断生命周期
    println!("  first_word = {word}");
    println!("  fn first_word(s: &str) -> &str 等价于");
    println!("  fn first_word<'a>(s: &'a str) -> &'a str （编译器自动补全）");

    println!("\n8、实战技巧：遇到借用冲突怎么办？");
    println!("  步骤1：确认引用的最后一次使用点（借用在哪里结束）");
    println!("  步骤2：看可变/不可变借用的区间是否重叠");
    println!("  步骤3：调整语句顺序，把「用完读借用」放在「开始写借用」之前");
    println!("  步骤4：如果必须同时持有，考虑是否需要 clone");
    println!("  步骤5：复杂场景考虑 RefCell（运行时借用检查，非本章内容）");
}
