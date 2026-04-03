use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 方法接收者与 API 设计：
//
//   方法接收者（receiver）是 self 的不同形式，本质是所有权规则的直接延伸：
//
//     &self        → 不可变借用：只读，不消费实例
//     &mut self    → 可变借用：可修改字段，不消费实例
//     self         → 接管所有权：消费实例，调用后不可再用
//
//   选择原则：
//     能用 &self 就用 &self（最宽松，调用方限制最少）
//     需要修改用 &mut self
//     需要消费（析构、转换、builder 模式）才用 self
//
//   API 设计惯例：
//     into_xxx(self)     → 消费并转换（into_string、into_bytes 等）
//     to_xxx(&self)      → 只读转换，通常带 clone（to_string、to_owned 等）
//     as_xxx(&self)      → 零拷贝视图（as_str、as_bytes 等）
//     with_xxx(self)     → builder 模式的链式调用
// ─────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    login_count: u32,
    active: bool,
}

impl User {
    // 关联函数（无 self）：构造器，惯例命名 new
    fn new(name: &str, email: &str) -> User {
        User {
            name: name.to_string(),
            email: email.to_string(),
            login_count: 0,
            active: true,
        }
    }

    // &self：只读，最通用的接收者
    // 返回字段引用：生命周期与 &self 绑定（编译器自动推断）
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn is_active(&self) -> bool {
        self.active // bool 是 Copy，直接返回值
    }

    fn summary(&self) -> String {
        // 返回新 String（不是借用）：无生命周期问题
        format!("{}（{}）登录 {} 次", self.name, self.email, self.login_count)
    }

    // &mut self：需要修改字段
    fn login(&mut self) {
        self.login_count += 1;
    }

    fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    // self：消费实例，提取内部数据
    // 命名惯例：into_ 前缀表示消费转换
    fn into_name(self) -> String {
        self.name // 提取 name，其余字段在这里 drop
    }

    fn into_summary_parts(self) -> (String, String, u32) {
        (self.name, self.email, self.login_count)
        // self 在函数结束时 drop（但 name/email 已经 move 出去了）
    }
}

// Builder 模式：用 self 实现链式调用
// 每个 with_xxx 方法消费旧 Builder，返回修改后的新 Builder
#[derive(Debug)]
struct RequestBuilder {
    url: String,
    method: String,
    timeout_secs: u64,
    headers: Vec<(String, String)>,
}

impl RequestBuilder {
    fn new(url: &str) -> Self {
        RequestBuilder {
            url: url.to_string(),
            method: String::from("GET"),
            timeout_secs: 30,
            headers: Vec::new(),
        }
    }

    // self → Self：消费旧 Builder，返回修改后的新 Builder（链式调用）
    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self // 把修改后的 self 返回出去，所有权转移给调用方
    }

    fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    // 最终构建：消费 Builder，"产出"结果（这里简化为打印）
    fn send(self) {
        println!("  发送请求: {} {}", self.method, self.url);
        println!("  超时: {}s，headers: {:?}", self.timeout_secs, self.headers);
        // self 在这里 drop，Builder 使命完成
    }
}

fn main() {
    println!("{}", "=== 方法接收者与 API 设计 ===".green().bold());

    println!("\n【接收者类型与所有权规则对应关系】");
    println!("  &self     → 不可变借用（规则：可多个并存）");
    println!("  &mut self → 可变借用（规则：排他，同时只能一个）");
    println!("  self      → 接管所有权（调用后实例不再有效）");
    println!("  方法接收者是所有权规则在面向对象风格中的自然体现");

    println!("\n1、&self：只读方法，最宽松");
    let mut user = User::new("Alice", "alice@example.com");
    // 可以同时调用多个 &self 方法（多个不可变借用）
    println!("  name = {}", user.name());
    println!("  email = {}", user.email());
    println!("  active = {}", user.is_active());
    println!("  summary = {}", user.summary());
    println!("  小结：&self 方法调用完后，user 仍然可用；可连续调用多次");

    println!("\n2、&mut self：修改方法，需要 mut 绑定");
    user.login();      // 每次调用 user 的可变借用开始，结束后才能再借
    user.login();
    user.rename("Alice Chen");
    println!("  修改后: {:?}", user);
    println!("  小结：&mut self 方法对 user 的可变借用是独占的，方法结束后恢复");

    println!("\n3、❌ 不要为只读方法写 &mut self");
    println!("  fn name(&mut self) -> &str {{ ... }}");
    println!("  → 调用 name() 就需要 mut 变量，并排斥其他不可变借用");
    println!("  → 接收者越严格，调用方限制越多：最小权限原则");
    println!("  经验法则：接收者类型选最宽松的（&self > &mut self > self）");

    println!("\n4、self：消费方法（into_ 命名惯例）");
    let user2 = User::new("Bob", "bob@example.com");
    let name = user2.into_name(); // user2 被消费，name 得到所有权
    // println!("{:?}", user2); // ❌ 编译错误：value used after move
    println!("  into_name 提取到: {name}（user2 已被消费）");

    let user3 = User::new("Carol", "carol@example.com");
    let (name, email, count) = user3.into_summary_parts();
    println!("  into_summary_parts: name={name}, email={email}, count={count}");
    println!("  小结：self 方法用于需要解构或转换实例的场景，命名用 into_");

    println!("\n5、Builder 模式：self 实现优雅的链式调用");
    // 每个 with_xxx 调用：消费旧 Builder → 返回新 Builder → 传给下一个方法
    RequestBuilder::new("https://api.example.com/data")
        .method("POST")
        .timeout(60)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .send(); // 最终消费 Builder，发送请求
    println!("  Builder 模式的所有权流：");
    println!("  new() → method() → timeout() → header() → send()");
    println!("  每步都是 self → Self，所有权沿链条流动，最终由 send() 消费");

    println!("\n6、to_xxx vs into_xxx vs as_xxx 命名惯例");
    let s = String::from("hello");
    // as_xxx：零拷贝，返回引用（&self）
    let bytes: &[u8] = s.as_bytes();    // 返回 &[u8]，没有分配
    let str_ref: &str = s.as_str();     // 返回 &str，没有分配
    println!("  as_bytes() = {:?}（零拷贝视图）", &bytes[..3]);
    println!("  as_str() = {str_ref}（零拷贝视图）");

    // to_xxx：有拷贝/分配，通常是 &self
    let owned: String = str_ref.to_string(); // 从 &str 创建新 String（分配堆内存）
    println!("  to_string() = {owned}（有分配）");

    // into_xxx：消费自身并转换（self）
    let bytes_owned: Vec<u8> = s.into_bytes(); // s 被消费，转成 Vec<u8>
    println!("  into_bytes() = {:?}（消费 s，零拷贝转换）", &bytes_owned[..3]);
    // println!("{s}"); // ❌ s 已被 into_bytes 消费

    println!("\n7、API 设计总结");
    println!("  ┌─────────────┬──────────────────┬─────────────────────────┐");
    println!("  │ 接收者      │ 调用者需要       │ 适用场景                │");
    println!("  ├─────────────┼──────────────────┼─────────────────────────┤");
    println!("  │ &self       │ 任意 user        │ 读取、查询、格式化      │");
    println!("  │ &mut self   │ mut user         │ 修改字段、追加数据      │");
    println!("  │ self        │ user（调用后失效）│ 消费、转换、builder     │");
    println!("  └─────────────┴──────────────────┴─────────────────────────┘");
    println!("  最小权限原则：方法能用 &self 就不要用 &mut self");
    println!("  最小权限原则：能用 &mut self 就不要用 self");
}
