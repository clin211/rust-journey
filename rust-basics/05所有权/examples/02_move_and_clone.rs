use colored::*;

fn takes_ownership(text: String) {
    println!("takes_ownership 收到了: {text}");
} // text 在这里 drop，堆内存释放

fn read_only(text: &str) {
    println!("只读借用: {text}");
} // 借用结束，原数据不受影响

fn main() {
    println!("{}", "=== move 与 clone ===".green().bold());

    // ─────────────────────────────────────────────────────────────────
    // move 的底层机制（非常重要，理解了就真正理解了所有权）：
    //
    //   String 在栈上存储一个"胖指针"，包含三个字段：
    //     ptr  → 指向堆上实际字符串数据的指针
    //     len  → 当前字符串的字节长度
    //     cap  → 堆上分配的总容量
    //
    //   赋值时，栈上的三个字段被 bitwise copy 到新变量
    //   旧变量被编译器静态标记为"无效"（零运行时开销）
    //   堆上的实际数据从未移动，只是 owner 换了人
    //
    //   这就解释了为什么 Rust 不允许多个变量同时拥有同一堆数据：
    //   否则作用域结束时会 double free → 未定义行为
    // ─────────────────────────────────────────────────────────────────

    println!("\n1、String 赋值默认会 move（所有权转移，堆数据不动）");
    let s1 = String::from("hello");
    //         ┌──栈──────────────────┐
    // s1  →   │ ptr → 堆: "hello"   │
    //         │ len = 5              │
    //         │ cap = 5              │
    //         └──────────────────────┘
    let s2 = s1;
    // s1 的三个栈字段 bitwise copy 给 s2
    // s1 被标记为无效，堆数据转由 s2 管理
    println!("s2 = {s2}");
    // println!("s1 = {s1}");
    // ❌ 编译错误：borrow of moved value: `s1`
    //    因为只能有一个 owner，s1 已让出所有权

    println!("\n2、move 是零成本的：只复制栈上 3 个字段，堆数据不动");
    let title = String::from("ownership");
    let new_title = title; // 栈上 3 个字段复制，堆 "ownership" 字节不动
    println!("new_title = {new_title}");
    println!("move 后旧绑定失效，底层数据由新 owner 管理");

    println!("\n3、clone 是真正的深拷贝：在堆上开辟新内存并复制数据");
    let book1 = String::from("The Rust Book");
    let book2 = book1.clone();
    //         book1 → 堆A: "The Rust Book"  (原始数据)
    //         book2 → 堆B: "The Rust Book"  (全新的副本)
    // 两者完全独立，drop 时各自释放各自的堆内存
    println!("book1 = {book1}");
    println!("book2 = {book2}");
    println!("clone 涉及堆分配，比 move 慢；只在真正需要两份数据时使用");

    println!("\n4、函数传参也会发生 move（传 String 就是把所有权交进函数）");
    let lesson = String::from("borrow checker");
    takes_ownership(lesson);
    // lesson 的所有权进入函数，函数结束后 drop，堆内存立即释放
    // println!("lesson = {lesson}");
    // ❌ lesson 已 move，所有权已不在当前作用域

    println!("\n5、如果函数只是读取，借用比 move 更合适");
    let note = String::from("能借用就先借用");
    read_only(&note); // &note：只是借用地址，不转移所有权
    println!("note 仍然可用: {note}"); // note 的所有权从未离开

    println!("\n6、把值 push 进 Vec 时，也会发生 move");
    let skill = String::from("Rust");
    let mut skills = Vec::new();
    skills.push(skill); // skill 的所有权 move 进了 Vec 内部
    println!("skills = {:?}", skills);
    // println!("skill = {skill}");
    // ❌ skill 已 move 进 Vec，Vec 成为新 owner

    println!("\n7、只有确实需要两份独立所有权时，才用 clone");
    let topic = String::from("ownership");
    let mut tags = Vec::new();
    tags.push(topic.clone()); // 克隆一份放进 Vec，原值保留
    println!("topic = {topic}"); // 原值 topic 所有权未变
    println!("tags = {:?}", tags); // Vec 持有独立副本

    println!("\n【核心决策口诀】");
    println!("  只读数据  → 借用 &T      零开销，首选");
    println!("  修改数据  → 借用 &mut T  零开销，明确意图");
    println!("  转交数据  → move T       零开销，清晰语义");
    println!("  复制数据  → .clone()     有堆开销，按需使用");
}
