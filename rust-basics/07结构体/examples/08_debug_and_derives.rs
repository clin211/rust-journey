#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// Debug 派生与常用派生宏（Derives）
//
// Rust 不像 JavaScript 那样可以直接 println!("{:?}", obj)，
// 结构体默认「不能被格式化、不能复制、不能比较、不能哈希」——
// 这些能力都需要你显式「派生（derive）」对应的 trait。
//
//   #[derive(Debug)]              → 支持 {:?} / {:#?} 打印
//   #[derive(Clone)]              → 支持显式 .clone() 深拷贝
//   #[derive(Copy)]               → 按位复制，要求所有字段都 Copy
//   #[derive(PartialEq)]          → 支持 == 和 !=
//   #[derive(Eq)]                 → 标记「完全相等」（要求 PartialEq）
//   #[derive(Hash)]               → 可作为 HashMap / HashSet 的 key
//   #[derive(PartialOrd, Ord)]    → 支持大小比较（<, <=, >, >=）
//   #[derive(Default)]            → 提供 Self::default()
//
// 常见搭配：
//   `#[derive(Debug, Clone, PartialEq)]` 几乎是所有值对象类型的标配
//   如果所有字段都是 Copy，可以加上 Copy；再想做 HashMap key 再加 Eq + Hash
//
// 本示例把这些派生的「使用方式、要求、限制」都演示一遍，
// 并介绍几个日常最好用的工具：`{:?}` / `{:#?}` / `dbg!` 宏。
// ─────────────────────────────────────────────────────────────────────────────

// ── 一个信息量足够的结构体：一次性演示多种派生 ──────────────────────────────
// Debug：{:?} / {:#?} 打印
// Clone：支持显式深拷贝
// PartialEq, Eq：== / != 和完全相等（HashMap key 需要 Eq）
// Hash：可作为 HashMap 的 key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// ── 所有字段都是 Copy 类型 → 可以派生 Copy ──────────────────────────────────
// 这类「值对象」常见于几何 / 数学 / 颜色等场景
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// ── Order：带嵌套结构体的派生 ────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
struct Item {
    name: String,
    price: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Order {
    order_id: u64,
    items: Vec<Item>,                        // 包含 Vec，Vec<Item> 也是 Clone
    total: f64,
}

// ── Config：用 Default 提供无参构造 ─────────────────────────────────────────
#[derive(Debug, Default, Clone)]
struct Config {
    verbose: bool,                           // default: false
    timeout_ms: u64,                         // default: 0
    tags: Vec<String>,                       // default: []
    host: String,                            // default: ""
}

fn main() {
    println!("{}", "=== Debug 派生与常用派生 ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、Debug：能被 {{:?}} 和 {{:#?}} 打印");
    // ─────────────────────────────────────────

    // 如果没有 #[derive(Debug)]，println!("{:?}", user) 会直接编译错误：
    //   `User` doesn't implement `Debug`
    // Debug 是 Rust 里最常派生的 trait，99% 的结构体都应该派生它
    let user = User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };

    // {:?} 是紧凑模式：所有字段挤在一行
    println!("  {{:?}}  = {:?}", user);

    // {:#?} 是「pretty」模式：每个字段一行，嵌套缩进
    println!("  {{:#?}} = {:#?}", user);

    println!("  两种格式符差别：{{:?}} 紧凑、{{:#?}} 多行缩进更适合长结构体");
    println!("小结：Debug 是「开发/调试」用的格式化，几乎所有结构体都该派生");

    // ─────────────────────────────────────────
    println!("\n2、Display vs Debug：两种打印的分工");
    // ─────────────────────────────────────────

    // 重要区别：
    //   Display ({})  → 面向最终用户的输出，不能派生，必须手动实现
    //   Debug   ({:?}) → 面向开发者的调试输出，推荐派生

    // println!("{}", user);                 // ❌ User 没实现 Display
    println!("  User 没有 Display，{{}} 打印不出来，只能用 {{:?}}");

    // 要支持 {}，需要手动实现 Display（这会在专门的章节讲）
    // 初学阶段：开发调试用 Debug 就够了
    println!("  规则：调试日志用 {{:?}}，用户界面输出用 {{}}");
    println!("  注意：Display 必须手写 impl，Debug 可以用 #[derive]");

    println!("小结：Debug 面向开发者，Display 面向用户；两者用途互补，不能混为一谈");

    // ─────────────────────────────────────────
    println!("\n3、Clone：显式深拷贝");
    // ─────────────────────────────────────────

    let u1 = User {
        id: 1,
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
    };

    // 没有 #[derive(Clone)] 时，u1.clone() 无法调用
    // 派生后，.clone() 对每个字段「逐字段 clone」，然后打包成新的 User
    let u2 = u1.clone();                     // u1 仍然有效，u2 是独立副本

    println!("  u1 = {:?}", u1);
    println!("  u2 = {:?}（独立副本）", u2);

    // 修改 u2 不会影响 u1
    let mut u3 = u1.clone();
    u3.email = String::from("CHANGED");
    println!("  修改 u3.email 后：");
    println!("    u1 = {:?}", u1);
    println!("    u3 = {:?}", u3);

    println!("  Clone 是「显式的深拷贝」：你要显式写 .clone() 才会发生");
    println!("  代价：字段里有 String / Vec 时，会真正分配新堆内存（有成本）");
    println!("小结：Clone 用于在不能 move 的场景下生成独立副本（谨慎使用）");

    // ─────────────────────────────────────────
    println!("\n4、Copy：按位复制，赋值 / 传参不 move");
    // ─────────────────────────────────────────

    // Copy 要求：所有字段都实现 Copy
    // Point 的 x / y 都是 f64（Copy），所以 Point 可以 #[derive(Copy)]
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1;                             // 不是 move，是按位复制
    let p3 = p1;                             // 可以随意多次复制

    println!("  p1 = {:?}, p2 = {:?}, p3 = {:?}", p1, p2, p3);
    println!("  p1 仍然可用！Copy 类型赋值不会让原变量失效");

    // ⚠️ User 不能加 Copy，因为 String 字段本身不是 Copy
    // #[derive(Copy)]
    // struct User { ... name: String }     // ❌ 编译错误

    println!("  ⚠️ User 不能派生 Copy —— 字段里有 String，不是 Copy 类型");
    println!("  规则：只有当「所有字段都是 Copy」时，结构体才能派生 Copy");
    println!("小结：Copy 适合「纯值、小、无堆分配」的类型，如坐标/颜色/单位等");

    // ─────────────────────────────────────────
    println!("\n5、PartialEq / Eq：== 和 !=");
    // ─────────────────────────────────────────

    let a = User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("a@x.com"),
    };
    let b = User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("a@x.com"),
    };
    let c = User {
        id: 2,
        name: String::from("Bob"),
        email: String::from("b@x.com"),
    };

    println!("  a == b → {}", a == b);       // true：字段全部相等
    println!("  a == c → {}", a == c);       // false：id 和 name 都不同
    println!("  a != c → {}", a != c);       // true

    // PartialEq 与 Eq 的区别：
    //   PartialEq：允许「浮点 NaN != NaN」这种不完全相等
    //   Eq       ：标记「完全相等」（NaN 不满足，所以 f64 不能 derive(Eq)）
    //
    // 经验：普通业务结构体可同时 derive(PartialEq, Eq)，
    //      只要没有浮点字段就没问题

    println!("  PartialEq 允许「部分相等」，Eq 标记「完全相等」");
    println!("  当字段里有 f64/f32 时，只能 derive PartialEq，不能 derive Eq");
    println!("小结：日常业务结构体常见组合是 PartialEq + Eq + Hash，便于进 HashMap/HashSet");

    // ─────────────────────────────────────────
    println!("\n6、Hash：作为 HashMap 的 key");
    // ─────────────────────────────────────────

    use std::collections::HashMap;

    // User 派生了 Hash + Eq，可以直接当 HashMap 的 key
    let mut scores: HashMap<User, i32> = HashMap::new();
    scores.insert(a.clone(), 95);
    scores.insert(c.clone(), 88);

    // 用「等值」查找：只要字段都相同，就能查到
    if let Some(score) = scores.get(&b) {    // b 的字段和 a 相同
        println!("  b 的分数 = {score} （注意：b 和 a 字段相同，哈希命中）");
    }
    if let Some(score) = scores.get(&c) {
        println!("  c 的分数 = {score}");
    }

    println!("  派生 Hash 要求同时派生 Eq（不能只派生 Hash）");
    println!("  经验：要做 Map key，永远同时派生 Eq + Hash");
    println!("小结：Eq + Hash 让结构体能当哈希表 key，这是业务代码里非常常用的能力");

    // ─────────────────────────────────────────
    println!("\n7、PartialOrd / Ord：排序");
    // ─────────────────────────────────────────

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Grade {
        score: u32,  // 排序的「第一关键字」
        name: String, // 分数相同时，按 name 字典序
    }

    // 派生的 Ord 是「字段从上到下逐个比较」的
    // 所以结构体里字段顺序影响排序
    let mut grades = vec![
        Grade { score: 85, name: "Alice".into() },
        Grade { score: 92, name: "Bob".into() },
        Grade { score: 85, name: "Carol".into() },
    ];

    grades.sort(); // 按 score 升序，score 相同按 name 升序
    println!("  sort 之后: {:#?}", grades);

    println!("  派生的顺序 = 「字段声明顺序」的字典序比较");
    println!("  要自定义排序规则，用 Vec::sort_by(|a, b| ...)");
    println!("小结：简单排序直接 derive(Ord, PartialOrd)，复杂排序用 sort_by");

    // ─────────────────────────────────────────
    println!("\n8、Default：零配置构造");
    // ─────────────────────────────────────────

    // 派生 Default 要求：所有字段都实现 Default
    // Rust 标准库的大部分基本类型都实现了 Default：
    //   bool / 整数 → 0 / false
    //   String      → 空字符串 ""
    //   Vec<T>      → 空 Vec
    //   Option<T>   → None
    let cfg_default = Config::default();
    println!("  Config::default() = {:?}", cfg_default);

    // 常见搭配：只覆盖你关心的字段，其他用默认值
    let cfg_custom = Config {
        verbose: true,
        timeout_ms: 5000,
        ..Default::default()                 // 其他字段默认值（tags / host）
    };
    println!("  部分覆盖 Config = {:?}", cfg_custom);

    println!("  #[derive(Default)] 让结构体拥有一个「无参默认构造器」");
    println!("  搭配 ..Default::default() 可以实现类似「命名参数」的使用方式");
    println!("小结：Default 是 Rust 无参构造的惯例，建议「值对象 + 可默认字段」都派生");

    // ─────────────────────────────────────────
    println!("\n9、dbg! 宏：打印表达式及其结果");
    // ─────────────────────────────────────────

    // dbg! 宏会：
    //   · 用 Debug 格式打印值
    //   · 同时打印文件名、行号、表达式原文
    //   · 返回被打印的值（所以可以嵌入到表达式里）

    let p = Point { x: 3.0, y: 4.0 };

    // 直接包裹表达式：dbg!(表达式)
    let len_sq = dbg!(p.x * p.x + p.y * p.y);
    println!("  长度平方 = {len_sq}");

    // 也可以打印任意变量
    let u = User {
        id: 99,
        name: "dbg".into(),
        email: "d@x.com".into(),
    };
    let _u = dbg!(u);                        // 注意：dbg! 会 move！

    println!("  dbg! 的特点：");
    println!("    · 打印到 stderr（不是 stdout）");
    println!("    · 同时输出文件名、行号、表达式文本");
    println!("    · 返回被打印的值，可以放在任意表达式里");
    println!("    · ⚠️ dbg!(x) 会 move x，如果之后还要用请 dbg!(&x) 打印引用");

    println!("小结：dbg! 是 Rust 开发中最常用的「临时调试输出」工具");

    // ─────────────────────────────────────────
    println!("\n10、嵌套结构体与 Vec 字段：派生自动递归");
    // ─────────────────────────────────────────

    let order = Order {
        order_id: 1001,
        items: vec![
            Item { name: "Book".into(), price: 29.9 },
            Item { name: "Pen".into(), price: 5.5 },
        ],
        total: 35.4,
    };

    // {:#?} 递归漂亮打印，嵌套 Vec / 嵌套 struct 都处理得很好
    println!("  order = {:#?}", order);

    // Clone 也会递归：先克隆 Vec，再对 Vec 里的每个 Item clone
    let order_clone = order.clone();
    assert_eq!(order, order_clone);
    println!("  order == order.clone() → {}", order == order_clone);

    println!("  #[derive(Debug)] / Clone / PartialEq 都会「沿着字段自动递归」");
    println!("  只要每个字段都满足对应 trait，外层结构体就可以 derive");
    println!("小结：组合字段（嵌套 / Vec / 选项等）都能自动享受 derive 带来的能力");

    // ─────────────────────────────────────────
    println!("\n【总结】常用派生速查表");
    // ─────────────────────────────────────────
    println!("  {:<14}  作用                              | 常见要求",            "派生");
    println!("  {:<14}  -------------------------------- | ----------------",  "---");
    println!("  {:<14}  {{:?}} / {{:#?}} 打印               | 字段也需要 Debug",   "Debug");
    println!("  {:<14}  .clone() 显式深拷贝               | 字段也需要 Clone",    "Clone");
    println!("  {:<14}  赋值/传参时按位复制               | 所有字段都 Copy",     "Copy");
    println!("  {:<14}  == / !=                           | 字段也需要 PartialEq","PartialEq");
    println!("  {:<14}  标记「完全相等」（HashMap 必需）  | 字段也需要 Eq",       "Eq");
    println!("  {:<14}  哈希（HashMap / HashSet 的 key）  | 配合 Eq 使用",        "Hash");
    println!("  {:<14}  排序比较                          | 字段也需要 Ord",      "Ord");
    println!("  {:<14}  Self::default() 无参构造         | 字段也需要 Default",  "Default");
    println!();
    println!("  推荐组合：");
    println!("    · 普通值对象 ：#[derive(Debug, Clone, PartialEq)]");
    println!("    · HashMap key：#[derive(Debug, Clone, PartialEq, Eq, Hash)]");
    println!("    · 纯值 Copy  ：#[derive(Debug, Clone, Copy, PartialEq)]");
    println!("    · 可默认     ：追加 Default");
}
