use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 悬垂引用（Dangling Reference）
//
//   定义：一个引用所指向的内存已经被释放，但该引用本身依然存在。
//         此时引用指向"无人认领"的内存区域，其内容完全不可预测。
//
//   C 语言类比（这是 C/C++ 程序员的经典噩梦）：
//     int* make_int() {
//         int x = 42;        // x 分配在栈帧上
//         return &x;         // 返回栈地址 —— 函数返回后栈帧弹出，地址失效
//     }
//     int* p = make_int();   // p 指向已释放的栈空间 → 未定义行为（UB）
//     printf("%d\n", *p);    // 可能输出 42，可能崩溃，可能输出垃圾值
//
//   Rust 的解决方案：借用检查器（Borrow Checker）
//     · 编译期追踪每个引用的有效区间（生命周期）
//     · 核心检查规则：引用存活区间 ⊆ 数据存活区间
//     · 违反时：编译错误，程序根本跑不起来
//     · 代价：零运行时开销（纯编译期静态分析）
//
//   经验法则（先记住这两条，覆盖 80% 场景）：
//     ① 函数内创建的数据 → 返回所有权（如 String），不返回引用
//     ② 需要返回引用    → 引用必须来自输入参数，由调用方管理数据
//
//   生命周期注解（'a）：
//     当有多个引用参数时，编译器需要知道返回值与哪个参数的生命周期绑定
//     程序员用 'a 显式标注这种关联关系（不改变运行时行为，只是声明）
// ─────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────
// ❌ 悬垂引用场景一：函数返回局部变量的引用
//
// fn dangle() -> &String {           // 返回一个 String 的引用
//     let s = String::from("hello"); // s 是函数内的局部变量（栈控制块 + 堆数据）
//     &s                             // 返回 s 的引用
// }                                  // ← s 在此处离开作用域，drop(s) 被调用
//                                    //   堆内存被释放，但调用方拿到的引用仍指向这里
//
// 编译错误（error[E0106]）：
//   missing lifetime specifier
//   this function's return type contains a borrowed value, but there is
//   no value for it to be borrowed from
//
// 为什么借用检查器能检测到这个问题？
//   · s 的存活区间 = 函数体内（从 let s 到函数末尾的 }）
//   · 返回值 &String 的存活区间需要延伸到调用方
//   · 引用区间（调用方）> 数据区间（函数内）→ 违反「引用 ⊆ 数据」规则
//   · 借用检查器在编译期检测到矛盾，拒绝编译
// ─────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────
// ❌ 悬垂引用场景二：函数返回局部字符串的切片
//
// fn bad_slice() -> &str {
//     let s = String::from("hello world"); // s 拥有堆内存
//     &s[..5]                              // 切片的 ptr 指向 s 的堆内存（字节 0~4）
// }                                        // ← s 在此处 drop，堆内存释放
//                                          //   切片的 ptr 指向已释放的内存 → 悬垂切片
//
// 编译错误（同上 error[E0106]）：
//   missing lifetime specifier
//   this function's return type contains a borrowed value, but there is
//   no value for it to be borrowed from
//
// 根本原因与场景一完全相同：
//   切片（&str）是胖指针（ptr + len），不拥有数据。
//   切片的数据来自局部变量 s，s 一旦 drop，堆内存释放，
//   ptr 失效，切片彻底成为悬垂指针（内容不可预测）。
// ─────────────────────────────────────────────────────────────────────────

// ✅ 修复方法一：返回所有权，而不是引用
// 函数内创建的数据 → 返回 String 本身，所有权移交给调用方
// 调用方持有所有权，数据不会因为函数结束而释放
fn make_string() -> String {
    let s = String::from("hello, ownership!"); // s：栈上控制块 + 堆上数据
    s // 返回 s 本身（所有权转移），不是引用
      // 调用方接管所有权，堆内存由调用方负责管理，不会在这里释放
}

// ✅ 修复方法二：从输入参数借用并返回（最常见的合法返回引用模式）
// 返回的 &str 来自参数 s，数据在调用方的变量中，由调用方负责管理
// 只要调用方的数据有效，返回的引用就有效——生命周期由调用方决定
fn first_word(s: &str) -> &str {
    // 编译器自动推断（生命周期省略规则）：
    //   fn first_word(s: &str) -> &str
    // 等价于（编译器内部补全为）：
    //   fn first_word<'a>(s: &'a str) -> &'a str
    // 含义：返回值的有效期 ≤ 参数 s 的有效期
    let bytes = s.as_bytes(); // 以字节数组视图遍历字符串
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // 找到第一个空格，返回空格前的切片（来自参数 s，安全）
            return &s[..i];
        }
    }
    &s[..] // 没有空格，返回整个字符串切片
}

// ✅ 进阶：显式生命周期注解
// 两个引用参数时，编译器不知道返回值的生命周期与哪个参数绑定
// 需要用 'a 显式标注：返回值有效期 ≤ min(s1 有效期, s2 有效期)
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 'a 是生命周期参数，名字可以是任意标识符，'a 是惯例写法
    // 语义：s1、s2、返回值三者的生命周期存在公共交集 'a
    // 调用方必须保证：在使用返回值期间，s1 和 s2 均有效
    if s1.len() >= s2.len() {
        s1 // 返回 s1 的引用（s1 和 s2 都活着时此引用有效）
    } else {
        s2 // 返回 s2 的引用
    }
}

fn main() {
    println!("{}", "=== 悬垂引用与借用检查器 ===".green().bold());

    println!("\n1、什么是悬垂引用（C 语言类比）");
    println!("  C 语言中：");
    println!("    int* make_int() {{ int x = 42; return &x; }}");
    println!("    int* p = make_int(); // p 指向已回收的栈帧 → 未定义行为");
    println!("    *p = ???             // 可能是 42，可能是垃圾，可能崩溃");
    println!();
    println!("  Rust 中：");
    println!("    借用检查器在编译期阻止这种情况，程序根本不能运行");
    println!("    核心规则：引用存活区间 ⊆ 数据存活区间");
    println!("    违反此规则 → 编译错误，零运行时代价");
    println!("小结：悬垂引用是 C/C++ 的经典 bug 源，Rust 在编译期彻底杜绝");

    println!("\n2、最常见场景：函数返回局部变量的引用");
    println!("  fn dangle() -> &String {{");
    println!("      let s = String::from(\"hello\"); // s 的存活区间开始");
    println!("      &s                             // 返回 s 的引用");
    println!("  }}  // ← drop(s)：堆内存释放，&s 变成悬垂引用");
    println!();
    println!("  编译器报错（error[E0106]）：");
    println!("    missing lifetime specifier");
    println!("    this function's return type contains a borrowed value,");
    println!("    but there is no value for it to be borrowed from");
    println!("小结：局部变量的引用不能逃出其作用域，这是借用检查器的核心保障");

    println!("\n3、第二种悬垂引用形式：返回局部字符串的切片");
    println!("  fn bad_slice() -> &str {{");
    println!("      let s = String::from(\"hello world\"); // s 拥有堆内存");
    println!("      &s[..5]  // 切片的 ptr 指向 s 的堆内存");
    println!("  }}            // drop(s) → 堆内存释放 → 切片 ptr 失效");
    println!();
    println!("  本质原因：切片 = 胖指针（ptr + len），不拥有数据。");
    println!("  数据的 owner（s）消失，切片的 ptr 就指向无效内存。");
    println!("  编译错误与场景一相同（error[E0106]）。");
    println!("小结：切片不拥有数据，其 owner 释放后切片立刻失效");

    println!("\n4、修复方法一：返回所有权而不是引用");
    let owned = make_string(); // 接管 String 的所有权
    println!("  make_string() 返回: \"{}\"", owned);
    println!();
    println!("  fn make_string() -> String {{ ... s }}  // 返回 String 本身");
    println!("  调用方接管所有权，堆内存由调用方负责管理，不会提前释放");
    println!("  经验法则：函数内创建的数据 → 返回所有权，不返回引用");
    println!("小结：返回所有权是最简单直接的解决悬垂引用的方法");

    println!("\n5、修复方法二：引用来自输入参数（最常见的返回引用模式）");
    let sentence = String::from("hello rust world"); // sentence 拥有堆内存
    let word = first_word(&sentence); // 返回的切片指向 sentence 的堆内存
    println!("  sentence = \"{}\"", sentence);
    println!("  first_word 返回: \"{}\"", word);
    // sentence 还在有效作用域内，所以 word 也有效
    println!();
    println!("  word 的数据来自 sentence，sentence 还活着，word 就有效");
    println!("  fn first_word(s: &str) -> &str  →  编译器内部补全为：");
    println!("  fn first_word<'a>(s: &'a str) -> &'a str");
    println!("  含义：返回值的有效期 ≤ 参数 s 的有效期");
    println!("小结：让引用来自输入参数，有效期由调用方数据决定，永远安全");

    println!("\n6、借用检查器工作原理（直觉性解释）");
    println!("  借用检查器给每个引用分配一个存活区间：[创建点, 最后使用点]");
    println!("  检查规则：引用的区间 ⊆ 被引用数据的存活区间");
    println!();
    println!("  ✅ 合法示例（区间包含关系成立）：");
    println!("    let s = String::from(\"hi\");  // s 存活区间开始");
    println!("    let r = &s;                   // r 区间开始，r 区间 ⊆ s 区间");
    println!("    println!(r);                  // r 最后一次使用，r 区间结束");
    println!("                                  // s 区间结束（r 已结束，安全）");
    println!();
    println!("  ❌ 悬垂引用（区间包含关系违反）：");
    println!("    let r;                        // r 声明，尚未绑定");
    println!("    {{");
    println!("        let s = String::from(\"hi\"); // s 存活区间开始");
    println!("        r = &s;                      // r 区间开始");
    println!("    }}                               // s 存活区间结束（s 被 drop）");
    println!("    println!(r);  // r 区间还未结束，但 s 已结束！→ r 区间 ⊄ s 区间 → 违规");
    println!();
    println!("  这一切在编译期完成，零运行时开销，不影响程序性能");
    println!("小结：借用检查器做的就是「区间包含检查」，违反即编译错误");

    println!("\n7、生命周期注解初探：fn longer<'a>(s1, s2) -> &'a str");
    // 演示：在 s2 的作用域内使用 longer 的返回值（两个参数都有效）
    let s1 = String::from("long string is long"); // s1 存活区间开始
    let result; // 声明 result，尚未绑定
    {
        let s2 = String::from("xyz"); // s2 存活区间开始
        result = longer(&s1, &s2); // result 借用自 s1 或 s2
        // 在 s2 的作用域内使用 result：此时 s1 和 s2 都有效，安全
        println!("  longer(\"{s1}\", \"{s2}\") = \"{result}\"");
    } // s2 存活区间结束，result 不能在此之后使用

    // ❌ 错误：s2 已释放，result 可能来自 s2，使用 result 违反 'a 约束
    // println!("result after block = {result}"); // 取消注释 → 编译错误

    println!();
    println!("  fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str");
    println!("  'a 的语义：返回值有效期 ≤ min(s1 有效期, s2 有效期)");
    println!("  'a 不改变运行时行为，只是给编译器的「生命周期约束声明」");
    println!("  使用场景：多个引用参数，返回其中一个 → 必须用 'a 标注");
    println!();
    println!("  生命周期省略规则（Elision Rules）：");
    println!("    单个引用参数时编译器自动推断，无需手写 'a");
    println!("    多个引用参数时编译器无法自动判断，需程序员显式标注");
    println!("小结：'a 是给借用检查器的提示，多输入引用返回引用时必须标注");
}