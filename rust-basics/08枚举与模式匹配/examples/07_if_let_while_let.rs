//! 07. if let / let else / while let：match 的轻量化简写
//!
//! 运行：cargo run --example 07_if_let_while_let
//!
//! 本例覆盖：
//! - if let：只关心一个变体
//! - if let ... else：再加一个 fallback
//! - if let 链（Rust 1.88+）
//! - let else：早返回 + 把值"摊"出来
//! - while let：循环消费

#![allow(dead_code, unused_variables)]
// 注：clippy 会建议把 `while let Some(x) = iter.next()` 改成 `for x in iter`,
// 但本文件的主题就是演示 while let 本身, 故意保留这种写法。
#![allow(clippy::while_let_on_iterator)]

// ============================================================================
// 1. if let：当你只在乎一个变体
// ============================================================================
//
// match 必须穷尽，但有时你只想"如果匹配上某种情况，就做点事，否则什么都不做"。
// 写完整的 match 会很啰嗦：
//
//   match opt {
//       Some(x) => println!("{x}"),
//       None => {}                      // 啥都不干，纯凑数
//   }
//
// 这种场景就该用 `if let`：

fn print_if_some(opt: Option<i32>) {
    if let Some(x) = opt {
        println!("  有值: {x}");
    }
    // 没有 else，没匹配上就直接跳过
}

// ============================================================================
// 2. if let ... else：模式匹配 + fallback
// ============================================================================
//
// 想在"没匹配上"时也走一段逻辑？加 else：

fn describe_opt(opt: Option<&str>) -> String {
    if let Some(name) = opt {
        format!("欢迎 {name}!")
    } else {
        "请先登录".to_string()
    }
}

// ============================================================================
// 3. if let 链：现代 Rust 的语法糖
// ============================================================================
//
// Rust 1.88+ 支持把多个 if let 用 `&&` 串起来——多个模式匹配 + 任意布尔条件混着写。
// 早期版本里这种写法要靠嵌套 if let 或 match，现在直接一行搞定。

#[derive(Debug)]
struct User {
    name: String,
    role: Role,
}

#[derive(Debug)]
enum Role {
    Guest,
    Member { vip: bool },
    Admin,
}

fn welcome_vip(user: Option<&User>) {
    // 三个条件同时成立才欢迎：
    //   user 是 Some
    //   user.role 是 Member
    //   member.vip == true
    if let Some(u) = user
        && let Role::Member { vip } = &u.role
        && *vip
    {
        println!("  尊贵的 VIP {}！", u.name);
    } else {
        println!("  普通通道，欢迎光临");
    }
}

// ============================================================================
// 4. let else（Rust 1.65+）：早返回 + 把值取出来
// ============================================================================
//
// `let else` 用来在"必须匹配上某个模式，否则就立刻退出"的场景里去掉嵌套：
//
// 老写法（嵌套缩进）：
//   match parse(s) {
//       Some(x) => {
//           // 主逻辑被嵌了一层
//           do_something(x);
//       }
//       None => return Err("parse failed"),
//   }
//
// 新写法（平直清爽）：
//   let Some(x) = parse(s) else { return Err("parse failed"); };
//   do_something(x);              // 主逻辑回到顶层缩进
//
// `else` 块必须发散（return / break / continue / panic / unreachable!），
// 否则编译失败——这条规则保证了"x 在后面一定是合法的"。

fn parse_age(s: &str) -> Option<u8> {
    s.parse().ok()
}

fn validate(input: &str) -> Result<u8, String> {
    let Some(age) = parse_age(input) else {
        return Err(format!("'{input}' 不是合法的 u8"));
    };

    if age < 18 {
        return Err("未满 18 岁不能继续".into());
    }

    Ok(age)
}

// 没有 let else 的写法，对比一下
fn validate_old(input: &str) -> Result<u8, String> {
    match parse_age(input) {
        Some(age) if age >= 18 => Ok(age),
        Some(_) => Err("未满 18 岁不能继续".into()),
        None => Err(format!("'{input}' 不是合法的 u8")),
    }
}

// ============================================================================
// 5. while let：循环消费
// ============================================================================
//
// while let 的形式是：
//   while let 模式 = 表达式 { 循环体 }
// 只要"表达式 match 上模式"就继续，否则跳出。
//
// 最常见的用法：从栈/队列里反复 pop，直到为空。

fn drain_stack() {
    let mut stack = vec![1, 2, 3, 4, 5];

    // pop() 返回 Option<T>：还有就 Some(x)，空了就 None
    while let Some(top) = stack.pop() {
        println!("  pop: {top}, 剩 {:?}", stack);
    }
    println!("  栈已清空");
}

// 配合 iter.next() 也很自然
fn show_lines() {
    let text = "alpha\nbeta\n\ngamma";
    let mut iter = text.lines();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            println!("  [空行]");
        } else {
            println!("  line: {line}");
        }
    }
}

// 多变体场景：消费一个事件队列，遇到 Quit 就停下
#[derive(Debug)]
enum Event {
    Click,
    Scroll(i32),
    Key(char),
    Quit,
}

fn run_event_loop(mut queue: Vec<Event>) {
    queue.reverse();                    // 让最早的事件在尾部，便于 pop 当作"队列"
    while let Some(ev) = queue.pop() {
        match ev {
            Event::Quit => {
                println!("  收到 Quit，退出事件循环");
                return;
            }
            other => println!("  处理: {other:?}"),
        }
    }
}

// ============================================================================
// 6. while let 与可变借用
// ============================================================================
//
// 一个常见的 idiom：处理可变集合时的"窥视 + 弹出"。

fn pop_while_positive(v: &mut Vec<i32>) {
    while let Some(&last) = v.last() {       // last() 返回 Option<&T>
        if last <= 0 { break; }
        v.pop();
    }
}

// ============================================================================
// 7. if let 与 match 的取舍
// ============================================================================
//
// 这两个能力高度重叠，怎么选？经验法则：
//
//   - 只关心 1 个变体（其它一概忽略）          → if let
//   - 只关心 1 个变体，但失败时也要做事         → if let / else
//   - 多个变体都要分别处理                     → match
//   - 有复杂的守卫、嵌套                       → match
//   - 想顺便确保"不会漏掉变体"                 → match (利用穷尽性检查)
//
// 当你"为了避免一行 match" 而写 if let，但其实关心多个分支时，
// 反而会丢掉穷尽性检查 —— 别贪图短而牺牲安全。

fn main() {
    println!("===== 1. if let（单变体）=====");
    print_if_some(Some(42));
    print_if_some(None);                       // 啥都不打印

    println!("\n===== 2. if let / else =====");
    println!("  {}", describe_opt(Some("alice")));
    println!("  {}", describe_opt(None));

    println!("\n===== 3. if let 链 =====");
    let admin = User { name: "carol".into(), role: Role::Admin };
    let vip = User { name: "alice".into(), role: Role::Member { vip: true } };
    let normal = User { name: "bob".into(), role: Role::Member { vip: false } };
    welcome_vip(Some(&admin));
    welcome_vip(Some(&vip));
    welcome_vip(Some(&normal));
    welcome_vip(None);

    println!("\n===== 4. let else =====");
    for input in ["19", "abc", "5"] {
        match validate(input) {
            Ok(age) => println!("  validate({input:?}) = Ok({age})"),
            Err(e) => println!("  validate({input:?}) = Err({e})"),
        }
    }
    // 与老写法语义等价
    println!("  (老写法等价: {:?})", validate_old("19"));

    println!("\n===== 5. while let（栈 pop）=====");
    drain_stack();

    println!("\n===== 6. while let（行迭代）=====");
    show_lines();

    println!("\n===== 7. while let（事件循环）=====");
    run_event_loop(vec![
        Event::Click,
        Event::Scroll(-3),
        Event::Key('A'),
        Event::Quit,
        Event::Click,                          // 这个永远到不了
    ]);

    println!("\n===== 8. while let + 可变借用 =====");
    let mut v = vec![3, 5, 7, -1, 2];
    pop_while_positive(&mut v);
    println!("  剩余: {:?}", v);

    println!("\n===== 要点回顾 =====");
    println!("· if let         单变体，忽略其它");
    println!("· if let / else  单变体 + fallback");
    println!("· if let 链 (1.88+) 多个模式 + 条件用 && 串起来");
    println!("· let else       早返回，让主逻辑保持顶层缩进");
    println!("· while let      循环消费集合，pop 到空为止");
}
