use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 结构体与 Vec 的所有权：
//
//   结构体（struct）拥有其字段的所有权
//     → 字段进入 struct 时发生 move（对 String/Vec 等堆类型）
//     → struct 离开作用域 → drop 所有字段（递归释放）
//
//   Vec<T> 拥有其元素的所有权
//     → push(value) 时元素 move 进 Vec
//     → Vec drop 时所有元素一并 drop
//
//   关键概念：
//     - 结构体字段借用：可以借用单个字段（编译器独立跟踪每个字段的借用）
//     - 部分 move：从 struct 中 move 出某个字段后，整个 struct 不能再被整体使用
//     - Vec 索引返回引用：不能从 Vec 中 move 出元素（会留下"空洞"）
// ─────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    city: String,
    login_count: u32,
}

impl User {
    fn new(name: &str, email: &str, city: &str) -> User {
        User {
            name: name.to_string(),
            email: email.to_string(),
            city: city.to_string(),
            login_count: 0,
        }
    }

    // &self：只借用，不消费
    fn greet(&self) {
        println!("  [greet] 你好，{}！来自 {}", self.name, self.city);
    }

    // 返回字段的借用，生命周期与 &self 绑定
    fn name(&self) -> &str {
        &self.name // 返回 self.name 的切片，与 self 生命周期绑定
    }
}

fn main() {
    println!("{}", "=== 结构体与 Vec 的所有权 ===".green().bold());

    println!("\n【底层模型】");
    println!("  struct 本身存储在栈上（或堆上，如果在 Box 里）");
    println!("  struct 的 String/Vec 字段在堆上分配数据");
    println!("  struct drop 时递归 drop 所有字段 → 自动释放堆数据");
    println!("  不需要手动析构，零内存泄漏");

    println!("\n1、把 String 赋值给 struct 字段时发生 move");
    let name = String::from("Alice");
    let email = String::from("alice@example.com");
    let city = String::from("Hangzhou");
    let user = User { name, email, city, login_count: 0 };
    // name、email、city 都 move 进了 user，原变量不再有效
    // println!("name = {name}");  // ❌ 编译错误：value borrowed here after move
    println!("user = {:?}", user);
    println!("  name/email/city 已经 move 进 user，原变量失效");
    println!("  现在 user 是这些字符串数据的唯一 owner");

    println!("\n2、struct 离开作用域时，自动释放所有字段");
    {
        let temp_user = User::new("Bob", "bob@example.com", "Beijing");
        println!("  temp_user.name = {}", temp_user.name);
        // temp_user 离开 {} 作用域 → drop(temp_user)
        // → 自动 drop temp_user.name / email / city（释放堆内存）
    } // ← temp_user 在这里 drop
    println!("  temp_user 已离开作用域，其 String 字段也被自动释放");

    println!("\n3、可以独立借用 struct 的不同字段（编译器分开跟踪）");
    let mut u = User::new("Carol", "carol@example.com", "Shanghai");
    let name_ref = &u.name;    // 借用 name 字段
    let city_ref = &u.city;    // 借用 city 字段（与 name 独立）
    println!("  name = {name_ref}, city = {city_ref}");
    // 两个不可变借用结束后，可以可变借用另一个字段
    u.login_count += 1; // ✅ 修改 u32 字段（Copy 类型，不影响上面的借用）
    println!("  login_count = {}", u.login_count);
    println!("  小结：编译器分别跟踪每个字段的借用状态，字段级别的借用检查");

    println!("\n4、不能同时对同一字段有可变和不可变借用");
    let name_borrow = u.name(); // &self 方法，借用整个 u
    println!("  u.name() = {name_borrow}");
    // name_borrow 最后一次使用在上面，借用已结束
    u.greet(); // 现在可以再次借用 u
    println!("  小结：方法的 &self 借用同样适用 NLL（最后使用后即结束）");

    println!("\n5、把 String push 进 Vec<String> 时发生 move");
    let rust = String::from("Rust");
    let go = String::from("Go");
    let python = String::from("Python");
    let mut languages: Vec<String> = Vec::new();
    languages.push(rust);   // rust  move 进 Vec
    languages.push(go);     // go    move 进 Vec
    languages.push(python); // python move 进 Vec
    // println!("rust = {rust}"); // ❌ rust 已经 move 进 Vec
    println!("  languages = {:?}", languages);
    println!("  Vec 是这些 String 的新 owner；Vec drop 时 String 一并释放");

    println!("\n6、Vec 索引返回引用，不能 move 出元素");
    // let first = languages[0]; // ❌ 编译错误：cannot move out of index of `Vec<String>`
    // 原因：移走元素会在 Vec 中留下"空洞"，破坏 Vec 的内部完整性
    let first_ref = &languages[0]; // ✅ 借用，不 move
    println!("  &languages[0] = {first_ref}");
    println!("  小结：languages[0] 是引用，不是 move；要独立副本用 .clone()");

    println!("\n7、如果还想继续使用原值，在 push 前 clone");
    let topic = String::from("ownership");
    let mut topics: Vec<String> = Vec::new();
    topics.push(topic.clone()); // clone 进去，topic 本身不变
    println!("  topic 仍然可用: {topic}");
    println!("  topics = {:?}", topics);
    println!("  注意：clone 有堆分配开销，仅在确实需要两份数据时使用");

    println!("\n8、[进阶] 部分 move（partial move）");
    let mut full_user = User::new("Dave", "dave@example.com", "Shenzhen");
    let extracted_name = full_user.name; // ← move 出 name 字段
    // println!("{:?}", full_user); // ❌ 整体不能用了（name 已 move）
    // full_user.greet();           // ❌ greet 需要借用 name，但 name 已 move
    println!("  extracted_name = {extracted_name}");
    // 但还可以访问未被 move 的字段：
    full_user.login_count += 1;
    println!("  full_user.city = {}（未被 move 的字段仍然可用）", full_user.city);
    println!("  小结：部分 move 后，struct 不能被整体使用，但未 move 的字段仍然独立有效");

    println!("\n9、结论：所有权规则对 struct/Vec 完全一致");
    println!("  String、Vec、struct — 凡是拥有堆资源的类型，都遵循同一套规则：");
    println!("  → 赋值/传参 = move（转移所有权）");
    println!("  → 只读 = &T 借用");
    println!("  → 修改 = &mut T 借用");
    println!("  → 需要副本 = .clone()（有堆分配开销，谨慎使用）");
}
