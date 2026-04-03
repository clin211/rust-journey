use colored::*;

fn read_resource(name: &str) {
    println!("只读借用资源名: {name}");
}

fn main() {
    println!(
        "{}",
        "=== 所有权基础（ownership basics） ===".green().bold()
    );

    // ─────────────────────────────────────────────────────────────
    // Rust 所有权三条铁律（背下来）：
    //   1. 每个值都有且只有一个 owner（所有者）
    //   2. owner 离开作用域，值立即被释放（drop）
    //   3. 同一时刻，值只能有一个 owner
    // ─────────────────────────────────────────────────────────────

    println!("\n【规则一】每个值都有且只有一个 owner");
    // String::from 在堆上分配内存，language 是它的 owner
    let language = String::from("Rust");
    // 赋值给 new_owner → 所有权转移（move），language 不再有效
    let new_owner = language;
    println!("new_owner = {new_owner}");
    // println!("language = {language}");
    // ❌ 编译错误：value borrowed here after move
    //    → 因为一个值同一时刻只能有一个 owner

    let topic = String::from("borrowing");
    println!("转移前先借用读取 -> {}", &topic); // 借用：不转移所有权
    let topic_owner = topic; // 所有权转移给 topic_owner
    println!("转移后由新 owner 继续使用 -> {topic_owner}");
    println!("小结：move 后旧绑定失效，只能通过新 owner 访问数据");

    println!("\n【规则二】owner 离开作用域，值立即 drop（自动释放）");
    {
        let framework = String::from("Actix Web");
        println!("进入内部作用域: {framework}");
    } // ← 这里编译器自动插入 drop(framework)，堆内存被释放
    // println!("framework = {framework}");
    // ❌ 编译错误：framework 已离开作用域

    // 正确：在外层创建，内层只借用
    let framework = String::from("Actix Web");
    {
        println!("内层借用 -> {}", &framework); // 借用，不影响 owner
    }
    println!("外层 owner 仍可用 -> {framework}");
    println!("小结：drop 是自动的，不需要手动 free，不会忘记释放");

    println!("\n【规则三】同一时刻只有一个 owner，所有权管理的是资源生命周期");
    // 场景：只是"读取"文件名，不需要接管所有权
    let file_name = String::from("app.log");
    // ❌ 错误模式：fn log_file(name: String) → 把所有权吞掉了
    //    log_file(file_name);
    //    println!("file_name = {file_name}"); // 编译错误

    // ✅ 正确：只读场景用借用（&）
    read_resource(&file_name); // 只是借用，不转移所有权
    println!("函数执行后，file_name 仍可用 -> {file_name}");
    println!("小结：函数只读参数优先借用而非接管所有权");

    println!("\n【Copy vs Move】栈上简单值 Copy，堆上复杂值 Move");
    // i32 实现了 Copy trait：赋值时自动复制一份，双方都可用
    let x: i32 = 42;
    let y = x; // 按位复制（bitwise copy），不涉及堆
    println!("x = {x}, y = {y}"); // 两个都能用

    // String 没有实现 Copy：赋值时转移所有权
    // let a = String::from("not copy");
    // let b = a;
    // println!("a = {a}"); // ❌ a 已 move，旧绑定失效

    // ✅ 正确：需要两份独立数据时显式 clone（深拷贝堆数据）
    let a = String::from("need two copies");
    let b = a.clone(); // 在堆上复制一份新数据，b 拥有新数据
    println!("a = {a}, b = {b}"); // 两个都能用，但 clone 有开销

    println!("小结：Copy 类型赋值自动复制；非 Copy 类型赋值默认 move");

    println!("\n【底层直觉】为什么 String 不能随意 Copy？");
    println!("  String 在堆上持有数据，若允许隐式 Copy：");
    println!("  → 两个变量同时持有同一堆地址");
    println!("  → 作用域结束时会 double free → 内存安全问题");
    println!("  Rust 通过 move 语义从根本上杜绝了这个问题");
}
