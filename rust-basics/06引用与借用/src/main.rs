use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 第六章：引用与借用（References & Borrowing）
//
//   引用与借用是 Rust 内存安全模型的核心，让你可以在不转移所有权的
//   情况下访问数据。借用检查器（Borrow Checker）在编译期静态地验证
//   所有引用规则，确保运行时永远不会出现悬垂引用或数据竞争。
//
//   三大借用规则（Rust 的核心契约）：
//     规则 A：同一时刻，允许「多个不可变借用（&T）」并存
//     规则 B：同一时刻，只允许「一个可变借用（&mut T）」，且排斥不可变借用
//     规则 C：引用必须始终有效（借用检查器在编译期保证，彻底杜绝悬垂引用）
//
//   本章示例文件（建议按顺序学习）：
//     01_immutable_references  → 不可变引用基础
//     02_mutable_references    → 可变引用与排他性规则
//     03_borrow_rules          → 三大规则详解与常见错误模式
//     04_dangling_references   → 悬垂引用与借用检查器的防护
//     05_slice_references      → 字符串切片与数组切片
//     06_lifetimes_intro       → 生命周期注解入门（'a 语法）
//     07_first_word            → 综合练习：first_word 函数的多种实现
// ─────────────────────────────────────────────────────────────────────────

fn main() {
    println!("{}", "=== 引用与借用 (References & Borrowing) ===".green().bold());

    println!("\n本章涵盖 Rust 引用与借用系统的核心概念。");
    println!("借用检查器在编译期保证内存安全，零运行时开销。");
    println!("建议按以下顺序运行各示例：");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── 示例列表 ──".cyan().bold());

    println!("\n  {}  不可变引用（Immutable References）",
        "01_immutable_references".yellow().bold());
    println!("     cargo run --example 01_immutable_references");
    println!("     主题：& 符号、解引用操作符（*）、胖指针结构（ptr + len）");
    println!("           引用作为函数参数、&String vs &str 的区别");

    println!("\n  {}  可变引用（Mutable References）",
        "02_mutable_references".yellow().bold());
    println!("     cargo run --example 02_mutable_references");
    println!("     主题：&mut 语法、可变引用的排他性、为什么同时只能有一个");
    println!("           &mut 与数据竞争的关系、mut 变量与 &mut 引用的区别");

    println!("\n  {}  借用三大规则详解（Borrow Rules）",
        "03_borrow_rules".yellow().bold());
    println!("     cargo run --example 03_borrow_rules");
    println!("     主题：规则 A/B/C 逐条演示、NLL（非词法生命周期）原理");
    println!("           常见编译错误及修复思路、借用区间可视化");

    println!("\n  {}  悬垂引用（Dangling References）",
        "04_dangling_references".yellow().bold());
    println!("     cargo run --example 04_dangling_references");
    println!("     主题：什么是悬垂引用、借用检查器如何阻止它");
    println!("           返回局部引用 vs 返回所有权、正确的替代方案");

    println!("\n  {}  切片引用（Slice References）",
        "05_slice_references".yellow().bold());
    println!("     cargo run --example 05_slice_references");
    println!("     主题：&str 字符串切片、&[T] 数组切片、字节索引与字符边界");
    println!("           切片借用对原数据的保护、Unicode 安全截取");

    println!("\n  {}  生命周期注解入门（Lifetime Annotations）",
        "06_lifetimes_intro".yellow().bold());
    println!("     cargo run --example 06_lifetimes_intro");
    println!("     主题：'a 语法与含义、生命周期省略规则（Elision Rules）");
    println!("           何时需要显式注解、多参数引用的生命周期关联");

    println!("\n  {}  综合练习：first_word 函数",
        "07_first_word".yellow().bold());
    println!("     cargo run --example 07_first_word");
    println!("     主题：&str 参数与返回值的生命周期绑定");
    println!("           三种实现版本（字节/迭代器/Unicode）、扩展练习函数");
    println!("           借用规则在真实函数中的体现");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── 三大借用规则核心演示 ──".cyan().bold());

    println!("\n1、规则 A：同一时刻允许多个不可变借用（&T）并存");

    let data = String::from("hello rust");              // data 拥有堆上字符串
    let r1 = &data;                                     // 第一个不可变借用
    let r2 = &data;                                     // 第二个不可变借用（✅ 与 r1 并存）
    let r3 = &data;                                     // 第三个不可变借用（✅ 全部并存）
    println!("  r1={r1}  r2={r2}  r3={r3}");            // 三个 & 引用同时有效
    // ❌ 错误：存在不可变借用时，不能同时创建可变借用
    // let r_mut = &mut data;   // 编译错误：cannot borrow `data` as mutable
    //                          //           because it is also borrowed as immutable
    println!("  ❌ 若同时存在 &mut data，编译器报错：无法同时持有不可变借用");
    println!("小结：读取不冲突，多个 & 引用可以随意并存，共享只读访问");

    println!("\n2、规则 B：可变借用（&mut T）是排他的，同时只能有一个");

    let mut value = String::from("world");              // 必须声明为 mut 才能创建 &mut
    {
        let rm = &mut value;                            // 创建可变借用（独占 value）
        rm.push_str(" updated");                        // 通过 &mut 修改数据
        println!("  通过 &mut 修改后: {rm}");
    }                                                   // rm 借用在这里结束（离开作用域）
    println!("  借用结束后可正常访问 value: {value}");   // ✅ 此时 rm 已不存在
    // ❌ 错误：同时创建两个 &mut 引用
    // let rm1 = &mut value;
    // let rm2 = &mut value;   // 编译错误：cannot borrow `value` as mutable
    //                         //           more than once at a time
    // println!("{rm1} {rm2}");
    println!("  ❌ 若同时有两个 &mut，编译器报错：可变借用最多同时一个");
    println!("小结：&mut 排他独占，防止数据竞争；可变借用结束后其他借用才能开始");

    println!("\n3、规则 C：引用必须始终有效（借用检查器阻止悬垂引用）");

    // ❌ 错误：试图返回局部变量的引用（无法编译，已注释）
    // fn dangle() -> &String {
    //     let s = String::from("hello"); // s 是局部变量
    //     &s                             // ❌ 函数结束时 s 被 drop，&s 失效
    // }                                  //    编译错误：missing lifetime specifier
    println!("  ❌ 返回局部变量引用 → 编译错误（函数结束时局部变量 drop）：");
    println!("     fn dangle() -> &String {{");
    println!("         let s = String::from(\"hello\");");
    println!("         &s  // s 在此 drop，引用悬空");
    println!("     }}");
    // ✅ 正确：返回所有权（让调用方持有数据）
    let owned = {
        let s = String::from("hello");                  // s 是局部 String
        s                                               // ✅ 转移所有权而非返回引用
    };
    println!("  ✅ 正确做法：返回 String 所有权，数据随所有权一起转移，不会释放");
    println!("  owned = \"{owned}\"");
    println!("小结：引用不能比数据活得更长，编译器在编译期静态验证，零运行时开销");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── 快速参考：引用类型一览 ──".cyan().bold());

    println!("\n  {:<12}  只读借用，允许多个并存，不可修改数据",     "&T");
    println!("  {:<12}  可变借用，排他独占，可修改数据",             "&mut T");
    println!("  {:<12}  字符串切片，对 str 数据的只读借用（胖指针）", "&str");
    println!("  {:<12}  切片，对连续 T 类型数据的只读借用",          "&[T]");
    println!("  {:<12}  生命周期参数，描述引用之间的有效期关系",     "'a");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── NLL（非词法生命周期）小贴士 ──".cyan().bold());

    println!("\n  Rust 2018+ 采用 NLL（Non-Lexical Lifetimes）：");
    println!("  借用在「最后一次使用」时结束，而不是在 }} 花括号处结束。");
    println!("  这意味着：用完不可变借用后，可以立刻开始可变借用。");

    let mut s = String::from("nll demo");
    let r = &s;                                         // 不可变借用开始
    println!("  先用完不可变借用 → {r}");               // r 最后一次使用，借用在此结束
    s.push_str("!");                                    // ✅ NLL：r 已结束，可以修改
    println!("  再修改原变量 → {s}");
    println!("  小贴士：遇到借用冲突，先检查能否调整语句顺序，把「读用完」放在「写之前」");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "提示：使用上方 cargo run --example <名称> 命令运行各示例。".bright_black());
}