use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 引用（借用）的两条铁律：
//
//   1. 在任意时刻，要么只能有任意数量的不可变引用（&T）
//                  要么只能有唯一一个可变引用（&mut T）
//                  两者不能同时存在
//
//   2. 引用必须始终有效（不能成为悬垂引用）
//
// 这两条规则在编译期静态检查，零运行时开销
// 本质上是"读者-写者"模型：多读者 OR 单写者，永远安全
// ─────────────────────────────────────────────────────────────────────────

fn calculate_length(s: &str) -> usize {
    s.len()
    // 借用结束，s 所指向的数据由原 owner 继续管理
}

fn print_text(text: &str) {
    println!("print_text 接收到: {text}");
}

fn print_string_ref(text: &String) {
    println!("print_string_ref 接收到: {text}");
}

fn main() {
    println!("{}", "=== 引用与不可变借用 ===".green().bold());

    println!("\n【借用的本质】");
    println!("  &T 是对数据的「临时访问权」，不转移所有权");
    println!("  可以把它理解为：拿到了数据的地址（指针），但 owner 仍是别人");
    println!("  借用期间，owner 不能做会使引用失效的操作（如 drop、move）");

    println!("\n1、借用解决了「只读但又不想失去所有权」的问题");
    let text = String::from("hello rust");

    // ❌ 错误模式：只是读取长度，却把所有权拿走了
    // fn calculate_length_owned(s: String) -> usize { s.len() }
    // let len = calculate_length_owned(text);
    // println!("text = {text}"); // 编译错误：text 已 move 进函数

    // ✅ 正确：借用，不转移所有权
    let len = calculate_length(&text); // &text：只借不占
    println!("'{text}' 的长度是 {len}");
    println!("借用结束后，text 仍然可用: {text}"); // owner 始终是当前作用域
    println!("小结：只读函数应接受 &str/&T，不应接管所有权");

    println!("\n2、可以同时存在多个不可变引用（读-读 不冲突）");
    let r1 = &text;
    let r2 = &text;
    let r3 = &text;
    // r1、r2、r3 同时指向同一块数据，但都只是"读"，不会互相干扰
    println!("r1 = {r1}, r2 = {r2}, r3 = {r3}");
    println!("小结：多个 &T 可以并存，因为读操作天然是并发安全的");

    println!("\n3、&String vs &str：参数类型影响接口灵活性");
    print_string_ref(&text); // ✅ 能接受 &String
    // print_string_ref("字面量");   // ❌ 编译错误：期望 &String，传入了 &str
    // print_string_ref(&text[0..5]); // ❌ 编译错误：切片不是 &String

    // ✅ &str 更通用：String、字面量、切片都能传
    print_text(&text); // String 自动 deref 到 &str
    print_text("字符串字面量也可以直接传入"); // 字面量本身就是 &str
    let hello = &text[0..5]; // 切片也是 &str
    print_text(hello);
    println!("小结：只读字符串参数优先写成 &str，接口更通用（Deref coercion）");

    println!("\n  [Deref 强制转换原理]");
    println!("  &String → &str 是自动的（String 实现了 Deref<Target=str>）");
    println!("  编译器遇到 &String 传给 &str 参数时，自动插入 .deref() 调用");
    println!("  这就是为什么 print_text(&text) 能工作，尽管参数类型是 &str");

    println!("\n4、只读场景不要盲目 clone（clone 有堆分配开销）");
    // ❌ 多余的 clone：只是读取，完全不需要独立副本
    // let copied = text.clone();
    // print_text(&copied);
    // println!("text = {text}");
    // ⚠️ 功能正确，但多分配了一次堆内存，纯属浪费

    // ✅ 直接借用，零开销
    print_text(&text);
    println!("直接借用即可，避免不必要的 clone");
    println!("小结：只读 → 借用；需要独立所有权 → clone");

    println!("\n5、借用的生命周期直觉");
    println!("  借用从创建 &T 的那行开始");
    println!("  借用在最后一次使用 &T 的那行结束（NLL：非词法生命周期）");
    println!("  不是在花括号结束时才结束，这让借用规则更灵活");
    println!("  → 详细示例参见 11_borrowing_lifetimes_intuition.rs");
}
