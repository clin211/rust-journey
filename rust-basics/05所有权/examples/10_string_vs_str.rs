use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// String、&String、&str 的本质区别：
//
//   String（拥有）：
//     栈：[ptr | len | cap]   →  堆：[h][e][l][l][o]
//     拥有堆上的字节数据，负责在 drop 时释放堆内存
//     可增长（push_str、push 等），内容可变
//
//   &str（切片引用）：
//     栈：[ptr | len]         →  指向某段 UTF-8 字节（可以是堆、栈、或 .rodata）
//     不拥有数据，不负责释放，只是"只读窗口"
//     字符串字面量就是 &str，存在二进制的 .rodata 段，生命周期 'static
//
//   &String（引用的引用）：
//     栈：[ptr]               →  指向 String 的胖指针（ptr|len|cap）
//     是对 String 的普通引用，比 &str 多一层间接
//     通过 Deref coercion，&String 可以自动转换成 &str
//
//   内存层次（从外到内）：
//     &String → String[ptr|len|cap] → 堆上字节数据
//     &str    → 直接指向字节数据（少一层间接）
//
//   实践结论：
//     参数只读字符串 → 优先 &str（最通用，&String/字面量/切片都能传）
//     需要修改字符串 → &mut String 或 接管 String
//     需要存储字符串（struct 字段等）→ String（拥有所有权）
// ─────────────────────────────────────────────────────────────────────────

fn take_owned(text: String) {
    // 接管所有权：调用后原变量失效，函数结束时 drop
    println!("  take_owned 拥有并将 drop: {text}");
}

fn take_string_ref(text: &String) {
    // 借用 String：接口最窄，只接受 &String
    println!("  take_string_ref 读取到: {text}");
}

fn take_str(text: &str) {
    // 借用字符串切片：接口最宽，&String/字面量/切片都可以传
    println!("  take_str 读取到: {text}");
}

fn append_to(text: &mut String, suffix: &str) {
    // 可变借用：修改原 String，不需要接管所有权
    text.push_str(suffix);
}

fn greet(name: &str) -> String {
    // 接受 &str，返回新 String（所有权交给调用方）
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", "=== String、&String 与 &str ===".green().bold());

    println!("\n【三种类型的内存模型】");
    println!("  String:   栈 [ptr|len|cap] ──────────────→ 堆 [字节数据]");
    println!("  &str:     栈 [ptr|len] ──────────────────→ 某处字节数据（无需是堆）");
    println!("  &String:  栈 [ptr] ──→ String[ptr|len|cap] ──→ 堆 [字节数据]");
    println!("  &str 比 &String 少一层间接，且来源更广泛");

    println!("\n1、String：拥有堆上的字符串数据");
    let mut owned = String::from("hello");
    owned.push_str(" rust"); // 可以修改，因为是所有者
    println!("  owned = {owned}");
    println!("  String::from 在堆上分配内存，owned 是这段数据的唯一 owner");
    println!("  owned 离开作用域时自动释放堆内存");

    println!("\n2、&String：对 String 的普通借用");
    take_string_ref(&owned); // 传入 &String
    // take_string_ref("字面量"); // ❌ 编译错误：expected &String, found &str
    // take_string_ref(&owned[0..5]); // ❌ 编译错误：切片是 &str，不是 &String
    println!("  &String 接口最窄：只能接受 &String，字面量和切片都不行");
    println!("  极少需要 &String 参数，因为 &str 更通用");

    println!("\n3、&str：对字符串切片的借用，接口最广");
    take_str(&owned);             // &String 自动 Deref 成 &str ✅
    take_str("字符串字面量");      // 字面量就是 &str ✅
    take_str(&owned[0..5]);       // 切片也是 &str ✅
    println!("  &str 参数：String/字面量/切片 统统可以传");
    println!("  Deref coercion：&String → &str 是自动的");
    println!("  编译器看到 &String 传给 &str 参数时，自动插入 .deref() 调用");

    println!("\n4、Deref 强制转换（Deref Coercion）原理");
    println!("  String 实现了 Deref<Target = str>");
    println!("  所以 *(&owned) → str，&*(&owned) → &str");
    println!("  编译器自动为 &String → &str 插入 Deref，程序员无需手写");
    println!("  这就是为什么 take_str(&owned) 能传 String 类型的引用");

    println!("\n5、接管所有权（String 参数）：调用后原变量失效");
    let title = String::from("ownership");
    take_owned(title); // title 的所有权 move 进函数
    // println!("title = {title}"); // ❌ 编译错误：borrow of moved value
    println!("  title 已 move，原变量失效");
    println!("  什么时候用 String 参数：需要存储（结构体字段）或消费（比如转换）");

    println!("\n6、可变借用（&mut String）：函数可以修改，不需要接管所有权");
    let mut message = String::from("hello");
    append_to(&mut message, " world");
    println!("  message = {message}（函数修改了它）");
    println!("  message 仍然在调用方手中，函数只是借用了修改权");

    println!("\n7、返回 String：把新建的字符串所有权交给调用方");
    let greeting = greet("Alice"); // 函数内部创建，所有权转移给 greeting
    println!("  greeting = {greeting}");
    println!("  greet 参数 &str，返回 String：这是最常见的模式");

    println!("\n8、字符串字面量就是 &str，生命周期 'static");
    let literal: &str = "I live in the binary"; // 存储在程序 .rodata 段
    let literal2: &'static str = "so do I";    // 显式标注 'static 生命周期
    println!("  literal = {literal}");
    println!("  literal2 = {literal2}");
    println!("  'static 表示整个程序运行期间都有效，不需要任何 owner");

    println!("\n9、类型选择决策树");
    println!("  ┌─ 需要存储/拥有字符串？");
    println!("  │   → String（struct 字段、HashMap 值等）");
    println!("  ├─ 只读访问字符串？");
    println!("  │   → &str（参数最优选，接受 String/字面量/切片）");
    println!("  ├─ 需要函数修改字符串？");
    println!("  │   → &mut String（可变借用，不转移所有权）");
    println!("  └─ 函数结束后调用方不再需要它？");
    println!("      → String（接管 + 消费，如 into_、from_ 类方法）");
}
