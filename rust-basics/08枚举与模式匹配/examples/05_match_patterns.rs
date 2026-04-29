//! 05. 模式语法全集：字面量 / 范围 / | / .. / 守卫 / @ / ref
//!
//! 运行：cargo run --example 05_match_patterns
//!
//! 本例覆盖：
//! - 字面量模式（i32 / char / bool / str）
//! - 多模式 `|`
//! - 范围 `..=` 和 `..`（仅整数 / char 区间合法）
//! - 通配 `_` 与忽略 `..`
//! - 命名变量绑定（注意 shadowing）
//! - 守卫 `if` 子句
//! - `@` 绑定（同时匹配 + 命名）
//! - `ref` / `ref mut`（少见但需要认识）

#![allow(dead_code, unused_variables, unreachable_patterns)]
// allow(unreachable_patterns) 用于展示 shadow_trap 中的陷阱：
// 我们故意写了一臂会捕获所有值，让后面的 `_ => ...` 永远到不了。
// 编译器会给这种写法发出警告 —— 这正是"模式书写"安全网的体现，
// 真实业务里你应该谨慎处理这条警告，绝不要 allow 掉。

// ============================================================================
// 1. 字面量模式
// ============================================================================
//
// 最朴素：用具体的字面量当作模式。

fn classify_int(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        -1 => "minus one",
        _ => "other",
    }
}

fn classify_char(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",       // | 多模式
        '0'..='9' => "digit",                          // 范围
        ' ' | '\t' | '\n' => "whitespace",
        _ => "other",
    }
}

// 字符串字面量也行（注意：必须是 &str / 类型一致）
fn translate_yes_no(s: &str) -> &'static str {
    match s {
        "yes" | "y" | "true" | "1" => "肯定",
        "no" | "n" | "false" | "0" => "否定",
        _ => "未知",
    }
}

// ============================================================================
// 2. 范围 `..=` 与 `..`
// ============================================================================
//
//   m..=n   闭区间 [m, n]    ← 模式里只能用闭区间和半开区间，标准用法
//   m..n    半开区间 [m, n)  ← Rust 1.66+ 模式里可用，老版本只能用 ..=
//
// 范围模式只能用在整数和 char 上（浮点不允许，因为 NaN 等导致排序模糊）。

fn grade(score: u32) -> &'static str {
    match score {
        0..=59 => "F",
        60..=69 => "D",
        70..=79 => "C",
        80..=89 => "B",
        90..=100 => "A",
        _ => "无效成绩",
    }
}

// ============================================================================
// 3. `_` 通配 与 `..` 忽略
// ============================================================================
//
//   `_`      占位一个值，不绑定                ← 完全不要这一项
//   `..`     在元组/结构体里"省略其余字段"
//
// 这两个看着像，其实工作层级不同。

fn first_two((a, b, ..): (i32, i32, i32, i32, i32)) -> (i32, i32) {
    (a, b)                                            // 用 .. 跳过后面 3 个字段
}

#[derive(Debug)]
struct Point3 { x: i32, y: i32, z: i32 }

fn xy_only(p: Point3) -> (i32, i32) {
    let Point3 { x, y, .. } = p;                       // 结构体里 .. 跳过其余字段
    (x, y)
}

// ============================================================================
// 4. 命名变量绑定（小心 shadowing）
// ============================================================================
//
// 在模式里写一个标识符 `x` 时，它会被解释成"绑定一个新变量"，而不是匹配外部变量 x。
// 这是初学者最容易踩的坑：

fn shadow_trap() {
    let x = 5;
    let y = 10;

    let result = match x {
        1 => "one",
        // ❌ 你也许想说"如果 x 等于 y，则 ..."；但这里 `y` 是个新绑定！
        // 它会匹配任何值，并把那个值绑定为本臂内的 y，覆盖外面的 y=10。
        y => "match anything (and bind to y)",     // ← 永远命中
        _ => "other",                               // 永远到不了这里
    };
    println!("[shadow_trap] x={x}, result={result}");
    // 注意：这里的 y=10 没被改变，只是 match 内部的 y 被 shadow 了。

    // 想"匹配某个变量值" → 用守卫 if，见下一节。
}

// ============================================================================
// 5. 守卫 (guard) `if` 子句
// ============================================================================
//
// 模式后面可以加 `if 条件`，只有模式 + 条件都成立时才进入此臂。
// 守卫常用来：
// - 匹配两个变量的关系：`x if x == y`
// - 加一个条件细分：`Some(n) if n > 0`

fn match_with_guard(pair: (i32, i32)) -> &'static str {
    match pair {
        (0, _) | (_, 0) => "至少一个零",
        (a, b) if a == b => "相等",
        (a, b) if a.abs() == b.abs() => "绝对值相等",
        _ => "其它",
    }
}

#[derive(Debug)]
enum LogEntry {
    Info(String),
    Warn(String),
    Error { code: i32, msg: String },
}

fn level(entry: &LogEntry) -> &'static str {
    match entry {
        LogEntry::Info(_) => "info",
        LogEntry::Warn(_) => "warn",
        LogEntry::Error { code, .. } if *code >= 500 => "fatal",  // 守卫 + .. 忽略
        LogEntry::Error { .. } => "error",
    }
}

// ============================================================================
// 6. `@` 绑定：模式匹配 + 同时记下整个值
// ============================================================================
//
// 当你既想"匹配某个范围"，又想"把那个具体值取出来用"时，用 `名字 @ 模式`。
// 例如：年龄在 13..=19 是青少年，并且我想要那个具体年龄。

fn classify_age(age: u32) -> String {
    match age {
        n @ 0..=12 => format!("孩子，年龄 {n}"),
        n @ 13..=19 => format!("青少年，年龄 {n}"),
        n @ 20..=64 => format!("成年人，年龄 {n}"),
        n => format!("长者，年龄 {n}"),
    }
}

// 嵌套结构体 + @
#[derive(Debug)]
struct Person { id: u32, name: String }

fn describe_person(p: &Person) -> String {
    match p {
        Person { id: id @ 1..=99, name } => format!("内部用户 #{id} {name}"),
        Person { id: id @ 100..=999, name } => format!("付费用户 #{id} {name}"),
        Person { id, name } => format!("游客 #{id} {name}"),
    }
}

// ============================================================================
// 7. `ref` 与 `ref mut`
// ============================================================================
//
// 现代 Rust 里 `ref` 用得越来越少，因为 `&` 模式 + 自动借用基本能覆盖。
// 但你仍可能在老代码或某些复杂场景里见到它，所以要认识。
//
//   语义对照：
//   match value { x => ... }         // x 是 value 的所有权（move）
//   match value { ref x => ... }     // x 是 &value（借用）
//   match value { ref mut x => ... } // x 是 &mut value（可变借用）
//
//   match &value { x => ... }        // x 是 &value（更现代的写法，常见）
//   match &mut value { x => ... }    // x 是 &mut value
//
// 现代代码里"用 & 写法 + 不写 ref"是惯例。

#[derive(Debug)]
enum Shape {
    Square(String),     // 故意带个 String，演示 move vs borrow
    Circle(String),
}

fn description_take(s: Shape) -> String {
    // 这里会消耗 s
    match s {
        Shape::Square(name) => format!("square {name}"),
        Shape::Circle(name) => format!("circle {name}"),
    }
}

fn description_borrow(s: &Shape) -> String {
    // 借用版本：函数返回后 s 仍然属于调用方
    match s {
        Shape::Square(name) => format!("square {name}"),
        Shape::Circle(name) => format!("circle {name}"),
    }
}

fn description_via_ref(s: Shape) -> (String, Shape) {
    // 用 ref 演示老写法：借用 name 而不消耗 s
    match s {
        Shape::Square(ref name) => (format!("square {name}"), Shape::Square(name.clone())),
        Shape::Circle(ref name) => (format!("circle {name}"), Shape::Circle(name.clone())),
    }
}

// ============================================================================
// 8. 综合：组合多种模式
// ============================================================================

#[derive(Debug)]
enum Cmd {
    Help,
    Quit,
    Set { key: String, value: String },
    Get(String),
    History(u32),
}

fn dispatch(cmd: &Cmd) -> String {
    match cmd {
        Cmd::Help | Cmd::Quit => "立即处理".into(),
        Cmd::Set { key, .. } if key.starts_with('_') => format!("私有 key: {key}"),
        Cmd::Set { key, value } => format!("设置 {key} = {value}"),
        Cmd::Get(key) => format!("读取 {key}"),
        Cmd::History(n @ 1..=10) => format!("查看最近 {n} 条历史 (有效)"),
        Cmd::History(n) => format!("历史条数 {n} 超出 [1, 10] 范围"),
    }
}

fn main() {
    println!("===== 1. 字面量模式 =====");
    for n in [-1, 0, 1, 7] {
        println!("  classify_int({n}) = {}", classify_int(n));
    }
    for c in ['a', 'b', '5', ' ', '%'] {
        println!("  classify_char({c:?}) = {}", classify_char(c));
    }
    for s in ["yes", "n", "maybe"] {
        println!("  translate({s:?}) = {}", translate_yes_no(s));
    }

    println!("\n===== 2. 范围 ..= =====");
    for s in [55, 65, 78, 88, 100, 105] {
        println!("  {s} 分 -> {}", grade(s));
    }

    println!("\n===== 3. _ 与 .. =====");
    let tup = (10, 20, 30, 40, 50);
    println!("  first_two({tup:?}) = {:?}", first_two(tup));
    let p = Point3 { x: 1, y: 2, z: 3 };
    println!("  xy_only(Point3) = {:?}", xy_only(p));

    println!("\n===== 4. 变量绑定 / shadowing 陷阱 =====");
    shadow_trap();

    println!("\n===== 5. 守卫 (guard) =====");
    for pair in [(0, 5), (3, 3), (3, -3), (1, 2)] {
        println!("  {:?} -> {}", pair, match_with_guard(pair));
    }
    let logs = [
        LogEntry::Info("启动完成".into()),
        LogEntry::Warn("内存使用 90%".into()),
        LogEntry::Error { code: 404, msg: "未找到".into() },
        LogEntry::Error { code: 503, msg: "服务挂了".into() },
    ];
    for e in &logs {
        println!("  {:?} -> level = {}", e, level(e));
    }

    println!("\n===== 6. `@` 绑定 =====");
    for age in [5, 16, 30, 70] {
        println!("  {}", classify_age(age));
    }
    let people = [
        Person { id: 7, name: "alice".into() },
        Person { id: 200, name: "bob".into() },
        Person { id: 99999, name: "tourist".into() },
    ];
    for p in &people {
        println!("  {}", describe_person(p));
    }

    println!("\n===== 7. ref / borrow 对比 =====");
    let s1 = Shape::Square("box".into());
    println!("  借用: {}", description_borrow(&s1));    // 不消耗 s1
    println!("  借用: {}", description_borrow(&s1));    // 仍可再用
    let consumed = description_take(s1);                 // 消耗 s1
    println!("  消耗: {consumed}");

    println!("\n===== 8. 综合 dispatch =====");
    let cmds = [
        Cmd::Help,
        Cmd::Set { key: "lang".into(), value: "zh".into() },
        Cmd::Set { key: "_internal".into(), value: "secret".into() },
        Cmd::Get("lang".into()),
        Cmd::History(5),
        Cmd::History(50),
    ];
    for c in &cmds {
        println!("  {:?}\n     => {}", c, dispatch(c));
    }

    println!("\n===== 要点回顾 =====");
    println!("· 字面量 / 多模式 | / 范围 ..= 是基础三件套");
    println!("· 模式里写名字 = 绑定新变量 (会 shadow，注意陷阱)");
    println!("· 守卫 if 用来引入运行期条件 + 跨字段比较");
    println!("· @ 让你既匹配模式，又拿到具体的值");
    println!("· `&` + 自动借用是现代写法，`ref` 用得越来越少");
}
