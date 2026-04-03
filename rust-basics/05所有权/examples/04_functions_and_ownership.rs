use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 函数与所有权：每一种参数类型都是一份"资源契约"
//
//   fn foo(s: String)      → 接管所有权，调用后原变量失效
//   fn foo(s: &str)        → 只读借用，调用后原变量仍可用
//   fn foo(s: &String)     → 只读借用（比 &str 接口更窄，不推荐）
//   fn foo(s: &mut String) → 可变借用，可以修改，但有排他限制
//   fn foo() -> String     → 返回值把所有权交给调用方
// ─────────────────────────────────────────────────────────────────────────

fn take_ownership(s: String) {
    println!("take_ownership 收到了: {s}");
} // s 离开作用域，drop(s)，堆内存释放

fn makes_copy(x: i32) {
    println!("makes_copy 收到了: {x}");
} // x 是 Copy 类型，离开作用域时什么都不用做

fn read_text(text: &str) {
    println!("read_text 读取到: {text}");
} // 借用结束，原数据不受影响

fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
} // 可变借用结束，调用方重新获得对 text 的完全控制

fn gives_ownership() -> String {
    String::from("hello from function")
    // 返回值将所有权交给调用方，不会被 drop
}

fn takes_and_gives_back(s: String) -> String {
    println!("takes_and_gives_back 收到了: {s}");
    s // 把接收到的所有权再转移回给调用方
}

// 实战：返回多个值时用元组归还所有权（Rust 1.0 时代的老写法，现在有借用）
fn calculate_length_old_style(s: String) -> (String, usize) {
    let len = s.len();
    (s, len) // 把 s 的所有权归还，同时带回 len
}

fn main() {
    println!("{}", "=== 函数中的所有权传递 ===".green().bold());

    println!("\n1、传递 String 给函数会发生 move");
    let text = String::from("hello");
    take_ownership(text);
    // text 的所有权进入 take_ownership，函数结束时 drop
    // println!("text = {text}");
    // ❌ 编译错误：borrow of moved value: `text`

    println!("\n2、传递 Copy 类型给函数时，原值仍然可用");
    let number = 42_i32;
    makes_copy(number); // number 被 bitwise copy，原 number 不动
    println!("number 仍然可用: {number}");

    println!("\n3、如果函数只是读取数据，应该借用而不是接管所有权");
    let article = String::from("ownership is about resource management");
    read_text(&article); // 传引用，只借不占
    read_text("字符串字面量也能直接传给 &str"); // 字面量本身就是 &str
    println!("article 仍然可用: {article}"); // 所有权从未离开

    println!("\n4、如果函数需要修改数据，用可变借用 &mut");
    let mut message = String::from("hello");
    append_suffix(&mut message, " rust"); // 借用期间函数可修改
    println!("message = {message}"); // 借用结束，message 恢复完全控制权

    println!("\n5、函数通过返回值把所有权交给调用方");
    let s1 = gives_ownership(); // 函数内创建的 String 所有权转移给 s1
    println!("s1 = {s1}");

    println!("\n6、函数可以接收所有权，处理后再归还");
    let s2 = String::from("ownership");
    let s3 = takes_and_gives_back(s2); // s2 move 进去，s3 接收归还的所有权
    // println!("s2 = {s2}"); // ❌ s2 已 move
    println!("s3 = {s3}");

    println!("\n7、老式写法：用元组同时返回所有权和计算结果");
    let s4 = String::from("hello world");
    let (s4, len) = calculate_length_old_style(s4); // 接管后归还
    println!("s4 = {s4}, len = {len}");
    println!("→ 现代写法：直接用 &str 借用更简洁，参见 05_borrowing.rs");

    println!("\n8、函数签名设计原则（资源契约）");
    println!("  fn foo(s: String)      → 接管：调用方永久让出所有权");
    println!("  fn foo(s: &str)        → 只读借用：最通用，推荐默认选择");
    println!("  fn foo(s: &mut String) → 可变借用：调用方允许函数修改");
    println!("  fn foo() -> String     → 产出：函数负责创建并交付所有权");
    println!("\n  原则：接管所有权要有充分理由（存储、转移、消费）");
    println!("        读取就借用，修改就可变借用，创建就返回所有权");
}
