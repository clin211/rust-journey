//! 15. 进阶模式匹配：可反驳性、嵌套深度、ref mut、let else 综合
//!
//! 运行：cargo run --example 15_advanced_patterns
//!
//! 本例覆盖：
//! - 可反驳模式 vs 不可反驳模式
//! - 模式可以出现在哪些位置
//! - 深嵌套解构（enum-in-struct-in-enum-in-...）
//! - 在 match 臂里同时绑定 + 借用
//! - matches! 宏
//! - 匹配字符串切片（精确、前缀、范围）
//! - 模式匹配编译期能拦截哪些错误（漏写变体、重复模式 unreachable）

#![allow(dead_code, unused_variables)]
// 注：first_two_leaves 与 first_two_leaves_v2 故意分别用嵌套 if let 与单层 if let
// 做对比展示, clippy::collapsible_if 在这里被有意保留。
#![allow(clippy::collapsible_if, clippy::collapsible_match)]

// ============================================================================
// 1. 可反驳性（refutable）vs 不可反驳性（irrefutable）
// ============================================================================
//
//   不可反驳模式（irrefutable）：永远能匹配上，不会失败
//     - `let x = 5;` 中的 `x`
//     - `let (a, b) = pair;` 中的 `(a, b)`
//     - 函数参数模式
//
//   可反驳模式（refutable）：可能匹配失败
//     - `if let Some(x) = opt { ... }` 中的 `Some(x)`
//     - match 的某些臂、while let 的模式
//
// 规则：
//   - let / 函数参数 / for      → 只接受不可反驳
//   - if let / while let         → 只接受可反驳
//   - match                      → 全收（每个臂都是模式）
//   - let else                   → 接受可反驳（必须有 else 分支）

fn refutability_demo() {
    let opt: Option<i32> = Some(7);

    // ❌ let Some(x) = opt;
    //    error: refutable pattern in local binding: `None` not covered
    //    解法：用 if let / let else / match

    // ✅ if let
    if let Some(x) = opt {
        println!("  if let: 拿到 {x}");
    }

    // ✅ let else
    let Some(x) = opt else {
        unreachable!()                    // 这里 opt 是 Some，永不触发
    };
    println!("  let else: x = {x}");

    // 不可反驳 let（永远成功）
    let pair = (10, "hi");
    let (n, s) = pair;
    println!("  let (n, s) = pair → n={n}, s={s}");
}

// ============================================================================
// 2. 模式出现的所有位置（一览）
// ============================================================================

fn where_patterns_appear() {
    // 1) let
    let (a, b) = (1, 2);

    // 2) 函数参数
    fn print_pair((x, y): (i32, i32)) {
        println!("  函数参数解构: ({x}, {y})");
    }
    print_pair((3, 4));

    // 3) for 循环（迭代器产出 (key, value) 等元组时常用）
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    for (k, v) in &map {
        println!("  for 解构: {k} = {v}");
    }

    // 4) match
    let x = 5;
    match x {
        1 => println!("  match: one"),
        2..=10 => println!("  match: 2..=10 hit"),
        _ => println!("  match: other"),
    }

    // 5) if let
    if let Some(v) = Some(42) {
        println!("  if let: {v}");
    }

    // 6) while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("  while let pop: {top}");
    }
    println!();

    // 7) let else
    fn safe_div(a: i32, b: i32) -> Option<i32> { if b == 0 { None } else { Some(a / b) } }
    let Some(q) = safe_div(10, 2) else { return };
    println!("  let else 取值: q = {q}");
}

// ============================================================================
// 3. 深嵌套解构：一行匹配多层
// ============================================================================
//
// 真实业务里 enum-in-struct-in-enum 的嵌套非常常见。
// 一行模式就能把多层信息一次取出。

#[derive(Debug)]
struct Profile {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Debug)]
struct Address {
    country: String,
    city: String,
}

#[derive(Debug)]
enum Account {
    Anon,
    Member { user_id: u64, profile: Profile },
    Admin(Profile),
}

fn city_of(a: &Account) -> Option<&str> {
    match a {
        Account::Anon => None,
        Account::Member {
            profile: Profile {
                address: Address { city, .. },
                ..
            },
            ..
        } => Some(city.as_str()),
        Account::Admin(Profile {
            address: Address { city, .. },
            ..
        }) => Some(city.as_str()),
    }
}

// ============================================================================
// 4. 在臂里同时借用 + 绑定（field shorthand）
// ============================================================================
//
// 当 match 的对象是 `&T` 时，编译器会自动让臂里绑定的变量也变成 `&T` 的引用——
// 这意味着 match 不会"消耗"原值。

#[derive(Debug)]
enum Item {
    Single(String),
    Many(Vec<String>),
}

fn describe(item: &Item) -> String {
    match item {
        Item::Single(s) => format!("一个: {s}"),    // s 是 &String
        Item::Many(v) => format!("一组: {v:?}"),    // v 是 &Vec<String>
    }
    // 函数返回后 item 仍然属于调用方，没有任何 move
}

// ============================================================================
// 5. matches! 宏：把"是不是某变体"压成布尔
// ============================================================================
//
// 当你只关心"是不是这种情况"，不关心数据时，用 matches! 比写整行 match 更短：

#[derive(Debug)]
enum Status {
    Active,
    Inactive { since: u64 },
    Banned { reason: String },
}

fn is_active(s: &Status) -> bool {
    matches!(s, Status::Active)
}

fn is_banned_recently(s: &Status) -> bool {
    // 还能带守卫
    matches!(s, Status::Banned { reason } if reason.contains("recent"))
}

// ============================================================================
// 6. 字符串模式匹配：常见错误与正确做法
// ============================================================================

fn classify_command(s: &str) -> &'static str {
    // String 不能直接作为 match 模式，要 .as_str() 或 match &s[..]
    match s {
        "help" | "?" => "帮助",
        "quit" | "exit" => "退出",
        // 范围模式不支持字符串，但可以在守卫里用 starts_with / ends_with
        x if x.starts_with("set ") => "设置命令",
        x if x.starts_with("get ") => "查询命令",
        _ => "未知命令",
    }
}

// ============================================================================
// 7. 嵌套深度匹配 + 守卫：把多层条件合并
// ============================================================================

#[derive(Debug)]
enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>),
}

fn first_two_leaves(t: &Tree) -> Option<(i32, i32)> {
    match t {
        // 直接匹配三层结构 + 取出两个叶子值
        Tree::Node(l, r) => match (l.as_ref(), r.as_ref()) {
            (Tree::Leaf(a), Tree::Leaf(b)) => Some((*a, *b)),
            _ => None,
        },
        _ => None,
    }
}

// 一行写法（嵌套 match 在模式里直接展开）
fn first_two_leaves_v2(t: &Tree) -> Option<(i32, i32)> {
    if let Tree::Node(l, r) = t {
        if let (Tree::Leaf(a), Tree::Leaf(b)) = (l.as_ref(), r.as_ref()) {
            return Some((*a, *b));
        }
    }
    None
}

// ============================================================================
// 8. ref mut：同时绑定 + 拿到可变借用
// ============================================================================
//
// 现代代码里几乎都改用 `match &mut value { Variant(x) => ... }` 写法，
// 但 ref mut 还能在某些场景见到。

#[derive(Debug)]
enum Counter {
    On(u32),
    Off,
}

fn double_if_on(c: &mut Counter) {
    match c {
        Counter::On(n) => *n *= 2,            // 现代写法：自动得到 &mut u32
        Counter::Off => {}
    }
}

// ============================================================================
// 9. 编译期保护：穷尽性 + unreachable 检测
// ============================================================================
//
// 编译器会主动告诉你：
//   - 模式覆盖不全（漏处理变体）
//   - 模式永远到不了（被前面 catch 走了）
//
// 这些警告值得开 #[deny(unreachable_patterns)] 强制做。

fn coverage_demo(s: Status) -> &'static str {
    // 故意写一个"unreachable"模式让编译器感受到。注释掉的那一行如果取消，
    // 编译器会发出 `unreachable_patterns` 警告。
    match s {
        Status::Active => "active",
        Status::Inactive { .. } => "inactive",
        Status::Banned { .. } => "banned",
        // _ => "never reached, please remove",   // ← 取消注释会触发 warning
    }
}

fn main() {
    println!("===== 1. 可反驳 vs 不可反驳 =====");
    refutability_demo();

    println!("\n===== 2. 模式出现的位置 =====");
    where_patterns_appear();

    println!("\n===== 3. 深嵌套解构 =====");
    let alice = Account::Member {
        user_id: 1,
        profile: Profile {
            name: "alice".into(),
            age: 30,
            address: Address {
                country: "CN".into(),
                city: "Shanghai".into(),
            },
        },
    };
    let admin = Account::Admin(Profile {
        name: "carol".into(),
        age: 35,
        address: Address {
            country: "US".into(),
            city: "NYC".into(),
        },
    });
    println!("  city_of(alice) = {:?}", city_of(&alice));
    println!("  city_of(admin) = {:?}", city_of(&admin));
    println!("  city_of(anon)  = {:?}", city_of(&Account::Anon));

    println!("\n===== 4. 借用式 match（不消耗原值）=====");
    let items = [
        Item::Single("apple".into()),
        Item::Many(vec!["a".into(), "b".into()]),
    ];
    for it in &items {
        println!("  {}", describe(it));
    }
    // 注意：items 仍然完整可用
    println!("  items 数量: {}", items.len());

    println!("\n===== 5. matches! 宏 =====");
    let users = [
        Status::Active,
        Status::Inactive { since: 1729000000 },
        Status::Banned { reason: "recent spam".into() },
        Status::Banned { reason: "old issue".into() },
    ];
    for u in &users {
        println!(
            "  {:?}\n     is_active={}, recent_ban={}",
            u,
            is_active(u),
            is_banned_recently(u)
        );
    }

    println!("\n===== 6. 字符串命令分类 =====");
    for s in ["help", "quit", "set lang zh", "get user", "what"] {
        println!("  '{s}' → {}", classify_command(s));
    }

    println!("\n===== 7. 嵌套深度 + 树结构 =====");
    let t = Tree::Node(Box::new(Tree::Leaf(10)), Box::new(Tree::Leaf(20)));
    println!("  first_two_leaves(...)    = {:?}", first_two_leaves(&t));
    println!("  first_two_leaves_v2(...) = {:?}", first_two_leaves_v2(&t));

    println!("\n===== 8. &mut 自动绑定 =====");
    let mut c = Counter::On(5);
    println!("  before = {c:?}");
    double_if_on(&mut c);
    println!("  after  = {c:?}");

    println!("\n===== 9. 穷尽性检查 =====");
    println!("  coverage_demo(Active) = {}", coverage_demo(Status::Active));
    println!("  coverage_demo(Banned) = {}", coverage_demo(Status::Banned { reason: "x".into() }));

    println!("\n===== 要点回顾 =====");
    println!("· 不可反驳 = let / 函数参数；可反驳 = if let / while let");
    println!("· let else (1.65+) 让'必须能 match' 早返回写得很清爽");
    println!("· 模式可以一直嵌套，跨多层结构一次拆开");
    println!("· matches! 把'是不是某变体'变成一句布尔，简洁高效");
    println!("· 编译器会同时检查穷尽性 + unreachable，让模式书写极其安全");
}
