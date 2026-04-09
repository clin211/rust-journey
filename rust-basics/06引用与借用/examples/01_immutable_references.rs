use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 不可变引用 &T
//
// 引用（Reference）是 Rust 借用系统的基石。
// 不可变引用 &T 代表对数据的"只读临时访问权"：
//
//   · 不转移所有权 —— owner 仍然是原来的变量
//   · 只能读取，不能修改所指向的数据
//   · 同一时刻可以存在任意数量的不可变引用
//   · 引用本身实现了 Copy，可以随意复制、传递
//
// 内存模型：
//   引用在栈上存储的是目标数据的内存地址（本质上就是指针）
//   但 Rust 的借用检查器保证这个地址在引用存活期间始终有效
//
// 与 C 指针的区别：
//   C 指针可以为 null、可以悬垂（指向已释放的内存）
//   Rust 引用在编译期就保证：非 null、不悬垂、不越界
// ─────────────────────────────────────────────────────────────────────────────

// 接受 &String 的函数：只能传入 String 的引用，字面量或切片不行
fn print_info_string(s: &String) {
    println!("  print_info_string 收到: {s}");
}

// 接受 &str 的函数：更通用，&String、字面量、&str 切片都能传
fn print_info_str(s: &str) {
    println!("  print_info_str 收到: {s}");
}

// 接受 &str 并返回长度：借用不影响调用方的所有权
fn calculate_length(s: &str) -> usize {
    s.len() // 使用完毕后借用自动结束，调用方仍拥有数据
}

// 展示引用是 Copy 的：接受 &str，可以多次调用而不消耗
fn greet(name: &str) {
    println!("  你好，{name}！");
}

fn main() {
    println!("{}", "=== 不可变引用 &T ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、引用的本质：临时访问权，不转移所有权");
    // ─────────────────────────────────────────

    let owner = String::from("Rust");   // owner 持有堆上字符串的所有权

    // &owner 创建一个指向 owner 数据的不可变引用
    // r 只是"借来看看"，不获取所有权
    let r = &owner;

    // owner 和 r 同时可用：owner 仍是所有者，r 只是借用者
    println!("  owner = {owner}");      // 原所有者可以正常使用
    println!("  r     = {r}");          // 借用者也可以读取

    // 借用结束后，owner 依然完整地持有数据
    println!("  借用结束，owner 依然有效: {owner}");

    println!("小结：&T 创建的引用不会夺走所有权，owner 始终有效");

    // ─────────────────────────────────────────
    println!("\n2、基础用法：let r = &x; 读取数据而不获取所有权");
    // ─────────────────────────────────────────

    let score: i32 = 100;               // 栈上的 i32 值
    let r_score = &score;               // &i32：对栈上数据的引用
    println!("  score   = {score}");    // 原变量可用
    println!("  r_score = {r_score}");  // 引用可以直接打印（自动解引用）

    let message = String::from("hello borrowing"); // 堆上的 String
    let r_msg = &message;               // &String：对堆上 String 的引用
    let len = r_msg.len();              // 通过引用调用方法，自动解引用
    println!("  message = {message}, 长度 = {len}");

    // 传入函数后，原变量依然可用
    let lang = String::from("Rust");    // lang 是 owner
    let length = calculate_length(&lang); // 只借用，不移交所有权
    println!("  lang = {lang}, 长度 = {length}"); // lang 仍然有效

    println!("小结：&x 创建引用，函数/变量都能通过引用读取数据而不消耗它");

    // ─────────────────────────────────────────
    println!("\n3、函数参数：&String vs &str，优先选 &str");
    // ─────────────────────────────────────────

    let greeting = String::from("你好，世界");  // 堆上的 String

    // ✅ &String 版本：只能传 String 的引用
    print_info_string(&greeting);       // 传 &String，正常工作

    // ❌ 错误：&str 的字面量不能传给 &String 参数
    // print_info_string("字符串字面量");  // expected &String, found &str
    // ❌ 错误：切片也不是 &String
    // print_info_string(&greeting[0..6]); // expected &String, found &str

    // ✅ &str 版本：&String、字面量、切片都能传（Deref 强制转换）
    print_info_str(&greeting);          // &String 自动 deref 成 &str
    print_info_str("字符串字面量");      // 字面量本身就是 &str
    print_info_str(&greeting[0..6]);    // 切片也是 &str（字节切片，前6字节）

    println!("  原因：String 实现了 Deref<Target=str>，&String 可自动转为 &str");
    println!("小结：只读字符串参数优先写 &str，比 &String 更通用，无额外开销");

    // ─────────────────────────────────────────
    println!("\n4、多个不可变引用可以同时存在");
    // ─────────────────────────────────────────

    let data = String::from("共享数据");   // data 是唯一 owner

    // 同时创建三个不可变引用，全部指向同一块数据
    let r1 = &data;                     // 第一个借用者
    let r2 = &data;                     // 第二个借用者（合法！）
    let r3 = &data;                     // 第三个借用者（也合法！）

    // r1、r2、r3 可以同时存在并同时使用
    println!("  r1 = {r1}");
    println!("  r2 = {r2}");
    println!("  r3 = {r3}");

    // 三个引用都指向同一内存地址
    println!("  r1 的地址: {:p}", r1 as *const String); // 打印内存地址
    println!("  r2 的地址: {:p}", r2 as *const String); // 相同地址
    println!("  r3 的地址: {:p}", r3 as *const String); // 相同地址

    println!("  原理：多个只读访问不会产生冲突，就像多人同时读同一本书");
    println!("小结：&T 可以无限复制、同时存在；读-读不冲突，天然并发安全");

    // ─────────────────────────────────────────
    println!("\n5、引用是只读的：不能通过 &T 修改数据");
    // ─────────────────────────────────────────

    let mut counter = 0i32;             // 可变变量
    let r_counter = &counter;           // 创建不可变引用

    // ✅ 可以通过不可变引用读取数据
    println!("  counter 当前值（通过引用读取）: {r_counter}");

    // ❌ 错误：不能通过不可变引用修改数据
    // *r_counter += 1;     // cannot assign to `*r_counter`, which is behind a `&` reference
    // *r_counter = 99;     // 同样的错误：引用是只读的

    // ✅ 正确：直接修改原变量（引用已经不再使用，借用已结束）
    // 注意：r_counter 的最后一次使用在上面的 println!，此后借用已结束（NLL）
    counter += 1;                       // 可以修改原变量
    println!("  直接修改原变量: counter = {counter}");

    println!("小结：&T 是只读视图，编译器静态拒绝一切通过 &T 进行的写操作");

    // ─────────────────────────────────────────
    println!("\n6、解引用操作符 *：访问引用指向的值");
    // ─────────────────────────────────────────

    let value = 42i32;                  // 栈上的 i32
    let r_value = &value;               // &i32：指向 value 的引用

    // 直接使用引用（许多地方 Rust 会自动解引用）
    println!("  r_value (自动解引用) = {r_value}");

    // 显式解引用：用 * 获取引用指向的值
    let deref_value = *r_value;         // *r_value 得到 i32 类型的值（因为 i32: Copy）
    println!("  *r_value (显式解引用) = {deref_value}");

    // 解引用后可以进行算术运算
    let doubled = *r_value * 2;         // 先解引用得到 42，再乘以 2
    println!("  *r_value * 2 = {doubled}");

    // 方法调用时 Rust 自动插入解引用（. 运算符）
    let text = String::from("hello");
    let r_text = &text;
    let upper = r_text.to_uppercase();  // 等价于 (*r_text).to_uppercase()
    println!("  通过引用调用方法（自动解引用）: {upper}");

    // 比较时也支持自动解引用
    let expected = 42i32;
    if *r_value == expected {           // 显式解引用再比较
        println!("  *r_value == {expected}: true");
    }

    println!("小结：* 显式解引用；. 运算符和 println! 会自动解引用，通常不需要写 *");

    // ─────────────────────────────────────────
    println!("\n7、不可变引用实现了 Copy：可以多次传递");
    // ─────────────────────────────────────────

    let name = String::from("Alice");   // String 本身不实现 Copy
    let r_name: &String = &name;        // &String 实现了 Copy

    // 因为 &String 实现了 Copy，传入函数后 r_name 仍然可用
    greet(r_name);                      // 传入 r_name（复制了引用，不是 String）
    greet(r_name);                      // 再次传入，r_name 仍然有效
    greet(r_name);                      // 第三次，依然有效

    // 可以把引用赋值给多个变量（每次赋值都是 Copy）
    let copy1 = r_name;                 // 复制引用（只复制了指针，8 字节）
    let copy2 = r_name;                 // 再复制一份
    println!("  copy1 = {copy1}, copy2 = {copy2}, r_name = {r_name}");

    // 对比：String 本身不实现 Copy，赋值会 move
    // let s1 = name;                   // 如果取消注释，name 会被 move
    // println!("{name}");              // ❌ name 已被 move，无法使用

    // ✅ String 仍然可用（我们只 Copy 了引用，没有 move String）
    println!("  原始 name 依然有效: {name}");

    println!("小结：引用本身只是一个内存地址（指针大小），Copy 代价极低");

    // ─────────────────────────────────────────
    println!("\n【总结】不可变引用 &T 的核心要点");
    // ─────────────────────────────────────────
    println!("  · 创建：let r = &x;  函数参数：fn foo(x: &T)");
    println!("  · 语义：临时只读访问，不转移所有权");
    println!("  · 限制：不能通过 &T 修改数据");
    println!("  · 并发：多个 &T 可同时存在，读-读不冲突");
    println!("  · Copy ：&T 是 Copy 类型，传递引用开销极低（仅复制指针）");
    println!("  · 解引：* 显式解引用，. 和 println! 自动解引用");
    println!("  · 最佳实践：只读参数优先用 &str 而非 &String，接口更通用");
}