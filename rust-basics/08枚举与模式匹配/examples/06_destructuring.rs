//! 06. 解构：enum / struct / tuple / 数组 / 嵌套
//!
//! 运行：cargo run --example 06_destructuring
//!
//! 本例覆盖：
//! - let 解构（模式 = 值）
//! - 元组、数组、结构体、枚举的解构
//! - 嵌套解构
//! - 函数参数的解构
//! - 部分忽略：`_` 和 `..`

#![allow(dead_code, unused_variables)]

// ============================================================================
// 1. let 解构：模式不是 match 专属
// ============================================================================
//
// 你写 `let x = 5;` 这种平凡的赋值时，左边其实就是一个**不可反驳模式**。
// "不可反驳"意思是：必然能匹配上，所以编译器允许在 let 里用。
//
// 这就是为什么你能写 `let (a, b) = (1, 2);`——元组在这里不过是个模式。

fn let_demo() {
    let x = 42;                                    // 名称模式
    let (a, b) = (1, 2);                           // 元组模式
    let [head, mid, tail] = [10, 20, 30];          // 数组模式（长度必须确定）
    println!("  let 解构: x={x}, (a,b)=({a},{b}), arr=[{head},{mid},{tail}]");
}

// ============================================================================
// 2. 函数参数也能解构
// ============================================================================
//
// 函数签名里的形参也是模式，所以能直接解构元组或结构体。
// 这能让函数体短小很多。

#[derive(Debug)]
struct Point { x: f64, y: f64 }

fn distance_to_origin(Point { x, y }: &Point) -> f64 {
    (x * x + y * y).sqrt()
}

fn add_pair((a, b): (i32, i32)) -> i32 {
    a + b
}

// ============================================================================
// 3. 解构枚举
// ============================================================================
//
// match 时已经在做解构。这里把"三种变体形态"的解构语法集中放到一起。

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

fn handle(msg: &Message) -> String {
    // 注意臂的"顺序"很关键：上面更宽松的模式会先吃掉数据，
    // 例如把 `Message::Echo(s) if s.is_empty() => ...`  写在 `Echo(s) =>` 之后，
    // 编译器会发出 unreachable_patterns 警告 —— 把更具体的臂往前放。
    match msg {
        Message::Quit => "quit".into(),
        Message::Echo(s) if s.is_empty() => "empty echo".into(), // ← 更具体的放前
        Message::Echo(s) => format!("echo {s}"),
        Message::Move { x, y } => format!("move {x},{y}"),
        Message::ChangeColor(r, g, b) => format!("color {r},{g},{b}"),
    }
}

// ============================================================================
// 4. 嵌套解构
// ============================================================================
//
// 实际项目里，最常见的就是"枚举里嵌着结构体、结构体里又有枚举"。
// 这种嵌套用一行模式就能拆开，省掉很多 if/else。

#[derive(Debug)]
struct UserInfo {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum Auth {
    Anonymous,
    Login(UserInfo),
    Banned { user: UserInfo, reason: String },
}

fn greeting(auth: &Auth) -> String {
    match auth {
        Auth::Anonymous => "游客你好".into(),
        Auth::Login(UserInfo { name, age }) => format!("你好 {name}，年龄 {age}"),
        Auth::Banned {
            user: UserInfo { name, .. }, // 嵌套：把 user.name 拆出来
            reason,
        } => format!("用户 {name} 已被封禁: {reason}"),
    }
}

// ============================================================================
// 5. 元组里嵌套枚举：双轴分类
// ============================================================================
//
// 一个 (Day, Weather) 元组的 match 是个常见技巧：用一对值同时分类两个维度。

#[derive(Debug, Clone, Copy)]
enum Day { Workday, Weekend }
#[derive(Debug, Clone, Copy)]
enum Weather { Sunny, Rainy }

fn plan(day: Day, weather: Weather) -> &'static str {
    match (day, weather) {
        (Day::Workday, Weather::Sunny) => "上班，阳光打卡",
        (Day::Workday, Weather::Rainy) => "上班，记得带伞",
        (Day::Weekend, Weather::Sunny) => "户外野餐",
        (Day::Weekend, Weather::Rainy) => "宅家看剧",
    }
}

// ============================================================================
// 6. 数组与切片解构
// ============================================================================
//
// 数组长度在编译期就能确定，可以直接 `let [a, b, c] = arr;`。
// 切片长度运行期才知道，所以需要在 match 里写"开头/结尾/中间"模式。

fn array_examples() {
    let arr = [10, 20, 30];
    let [a, b, c] = arr;
    println!("  let [a,b,c] = {arr:?} -> {a},{b},{c}");

    let big = [1, 2, 3, 4, 5];
    match big {
        [first, .., last] => println!("  数组首尾: {first} ... {last}"),
    }

    // 切片：字符串 split 后的 Vec 切片，按长度分类
    let words: Vec<&str> = "rust is fun".split_whitespace().collect();
    match words.as_slice() {
        [] => println!("  空字符串"),
        [only] => println!("  只有一个词: {only}"),
        [first, second] => println!("  两个词: {first}, {second}"),
        [first, .., last] => println!("  多个词: 首={first}, 尾={last}"),
    }
}

// ============================================================================
// 7. 部分忽略：_ vs ..
// ============================================================================
//
//   `_`  恰好占一个位置，不绑定值
//   `..` 在元组/结构体里"省略其余字段"（一次性吃掉多个）

fn ignore_demo() {
    // 元组里：_ 跳过单个，.. 跳过余下
    let tup = (1, 2, 3, 4, 5);
    let (first, _, _, _, fifth) = tup;
    let (a, .., e) = tup;
    println!("  跳单个: first={first}, fifth={fifth}");
    println!("  跳余下: a={a}, e={e}");

    // 结构体里：.. 跳过未提到的字段
    #[derive(Debug)]
    struct Order { id: u64, sku: String, qty: u32, price: f64 }
    let o = Order { id: 1, sku: "A001".into(), qty: 3, price: 9.9 };
    let Order { id, qty, .. } = o;
    println!("  Order 部分解构: id={id}, qty={qty}");
}

// ============================================================================
// 8. let 模式 + Option：常见 idiom
// ============================================================================
//
// `let Some(x) = opt` 这种写法是**可反驳模式**（可能匹配失败），
// 不能直接用在 let 里——会编译错误。要用 `if let` / `let else`。

fn maybe_double(x: Option<i32>) -> Option<i32> {
    // 错误示范（编译失败）：
    // let Some(v) = x;     // ❌ refutable pattern in local binding

    // 正确写法 1：if let
    if let Some(v) = x {
        return Some(v * 2);
    }
    None
}

fn must_double(x: Option<i32>) -> i32 {
    // 正确写法 2：let else（Rust 1.65+）
    let Some(v) = x else {
        return -1;
    };
    v * 2
}

fn main() {
    println!("===== 1. let 解构 =====");
    let_demo();

    println!("\n===== 2. 函数参数解构 =====");
    let p = Point { x: 3.0, y: 4.0 };
    println!("  Point({},{}) 到原点距离 = {}", p.x, p.y, distance_to_origin(&p));
    println!("  add_pair((10,20)) = {}", add_pair((10, 20)));

    println!("\n===== 3. 解构枚举 =====");
    let msgs = [
        Message::Quit,
        Message::Echo("hi".into()),
        Message::Move { x: 1, y: 2 },
        Message::ChangeColor(0xff, 0x00, 0x66),
    ];
    for m in &msgs {
        println!("  {} ", handle(m));
    }

    println!("\n===== 4. 嵌套解构 =====");
    let auths = [
        Auth::Anonymous,
        Auth::Login(UserInfo { name: "alice".into(), age: 30 }),
        Auth::Banned {
            user: UserInfo { name: "bob".into(), age: 25 },
            reason: "刷帖".into(),
        },
    ];
    for a in &auths {
        println!("  {}", greeting(a));
    }

    println!("\n===== 5. (Day, Weather) 双维度 =====");
    for d in [Day::Workday, Day::Weekend] {
        for w in [Weather::Sunny, Weather::Rainy] {
            println!("  {:?} + {:?} -> {}", d, w, plan(d, w));
        }
    }

    println!("\n===== 6. 数组与切片 =====");
    array_examples();

    println!("\n===== 7. _ 与 .. 区别 =====");
    ignore_demo();

    println!("\n===== 8. Option + let 的两种正确写法 =====");
    println!("  maybe_double(Some(7)) = {:?}", maybe_double(Some(7)));
    println!("  maybe_double(None)    = {:?}", maybe_double(None));
    println!("  must_double(Some(7))  = {}", must_double(Some(7)));
    println!("  must_double(None)     = {}", must_double(None));

    println!("\n===== 要点回顾 =====");
    println!("· 模式不是 match 专属 -- let、函数参数、for、if let 全都接受模式");
    println!("· 嵌套解构能把多层结构一次拆开，少写很多代码");
    println!("· `_` 是恰好一个位置；`..` 是在元组/结构体里省略多个");
    println!("· 可反驳模式用 if let 或 let else，不能直接用在 let 里");
}
