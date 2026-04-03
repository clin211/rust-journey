use colored::*;

fn change(s: &mut String) {
    s.push_str(" world");
}

fn push_score(scores: &mut Vec<i32>, score: i32) {
    scores.push(score);
}

fn main() {
    println!("{}", "=== 可变引用与借用规则 ===".green().bold());

    // ─────────────────────────────────────────────────────────────────
    // 借用规则核心（编译期强制执行）：
    //
    //   规则 A：&T  可以同时存在多个（共享只读）
    //   规则 B：&mut T 同一时刻只能有一个（排他可写）
    //   规则 C：&T 和 &mut T 不能同时活跃（不允许"读写并发"）
    //
    //   这三条规则是数据竞争（data race）的完全消除方案：
    //     两个指针同时指向同一数据 + 至少一个在写 + 没有同步机制 = data race
    //   Rust 在编译期静态排除了这种情况
    // ─────────────────────────────────────────────────────────────────

    println!("\n1、可变借用允许函数修改原数据");
    let mut text = String::from("hello");
    change(&mut text); // 把 text 的可变借用传给 change
    // change 函数内：s 是 &mut String，可以调用 push_str 等修改方法
    println!("text = {text}");
    println!("小结：需要函数原地修改数据时，传 &mut T");

    println!("\n2、同一时刻只能有一个 &mut（排他性规则）");
    // ❌ 错误：两个 &mut 同时活跃
    // let mut language = String::from("Rust");
    // let r1 = &mut language;
    // let r2 = &mut language; // 编译错误：cannot borrow `language` as mutable more than once at a time
    // println!("r1 = {r1}, r2 = {r2}");
    // 如果允许两个 &mut 同时存在，就可能发生 data race

    // ✅ 正确：串行使用，借用不重叠
    let mut language = String::from("Rust");
    {
        let r1 = &mut language;
        r1.push_str(" language");
        println!("r1 使用完毕 -> {r1}");
    } // r1 的可变借用在这里结束

    {
        let r2 = &mut language; // 前一个 &mut 已结束，可以开新的
        r2.push_str(" ownership");
        println!("r2 使用完毕 -> {r2}");
    } // r2 的可变借用在这里结束
    println!("language = {language}");
    println!("小结：同一时刻只能有一个 &mut，但可以串行使用多次");

    println!("\n3、&T 和 &mut T 不能同时活跃（规则 C）");
    // ❌ 错误：不可变借用活跃期间，不能创建可变借用
    // let mut message = String::from("borrow rules");
    // let r1 = &message;
    // let r2 = &mut message; // 编译错误：cannot borrow as mutable because it is also borrowed as immutable
    // println!("r1 = {r1}"); // r1 后续还在用
    // println!("r2 = {r2}");
    // 原因：r2 可能重新分配内存，导致 r1 指向无效地址

    // ✅ 正确：先用完不可变借用，再创建可变借用
    let mut message = String::from("borrow rules");
    let r1 = &message;
    println!("先用完只读借用 -> {r1}"); // r1 最后一次使用，借用结束
    // r1 的借用在上面这行结束（NLL：Non-Lexical Lifetimes）
    let r2 = &mut message; // 此时 r1 已失效，可以安全创建 &mut
    r2.push_str(" are strict but useful");
    println!("再进行可变借用 -> {r2}");
    println!("message = {message}");
    println!("小结：只读借用最后一次使用后，才能开始可变借用");

    println!("\n4、&mut 同样适用于所有需要修改的类型");
    let mut scores = vec![90, 95];
    push_score(&mut scores, 100); // Vec<i32> 的可变借用
    println!("scores = {:?}", scores);

    println!("\n5、[进阶] 为什么这些规则能防止 data race？");
    println!("  data race 的三个条件：");
    println!("    a) 两个或多个指针同时指向同一数据");
    println!("    b) 至少一个指针在写入数据");
    println!("    c) 没有同步机制（锁等）");
    println!("  Rust 的借用规则：");
    println!("    → &mut 排他：不可能出现「写 + 任何其他访问」同时存在");
    println!("    → 多个 &T：全是读，读-读 天然安全");
    println!("    → 编译期静态检查：连运行时的机会都没有");
    println!("  这就是为什么 Rust 能做到「无锁并发安全」的底层基础");
}
