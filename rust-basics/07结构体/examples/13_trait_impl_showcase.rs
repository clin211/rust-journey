#![allow(dead_code)]

use colored::*;
use std::fmt;
use std::ops::{Add, AddAssign, Mul};

// ─────────────────────────────────────────────────────────────────────────────
// trait 实现秀（Trait Impl Showcase）
//
// 前面的章节里，我们用 #[derive] 自动得到了 Debug / Clone / PartialEq 等能力。
// 但 Rust 真正的威力在于：**你可以为自己的结构体手动实现任何 trait**。
//
// 这一章用几个最常见、最有表现力的 trait 演示「把类型当一等公民」：
//
//   1. impl Display    → 让结构体可以 println!("{}", x) 面向用户输出
//   2. impl From<T>    → 自动获得 Into<T>，实现优雅的类型转换
//   3. impl Add/Mul    → 运算符重载，让 p1 + p2 / m * 2 变合法
//   4. impl AddAssign  → += 运算符
//   5. impl Default    → 手写 Default（vs derive，什么时候该手写）
//   6. impl Iterator   → 让结构体能被 for 循环遍历（预告）
//
// 核心哲学：
//   · Rust 的 trait 系统是「开放扩展」的：任何人都可以给任何类型实现 trait
//     （只要满足孤儿规则：类型或 trait 至少有一个是你自己的）
//   · 手写 impl 给了你完全控制：自定义显示格式、自定义相等、自定义转换
//   · 这让你的自定义类型和标准库类型「地位平等」——
//     你的 Point 和 std::collections::HashMap 在语言层面是同等公民
//
// 这些东西不新奇，但合起来就是 Rust「类型一等公民」文化的体现。
// 布道 Rust 时，这一章是最容易让听众「哇」出来的部分。
// ─────────────────────────────────────────────────────────────────────────────

// ── 1. Display：面向用户的格式化输出 ─────────────────────────────────────────
// Debug 是给开发者看的（{:?}），Display 是给用户看的（{}）
// Debug 可以派生，Display 必须手写（为什么？因为"面向用户"没有通用规则）
//
// impl Display 要求实现 fmt 方法，把内容写到 Formatter 里
// 最常用的做法是 write!(f, "...") / writeln!(f, "...")，就像 println! 但写入 f
#[derive(Debug)]
struct Money {
    amount: i64,                                 // 以「分」为单位，避免浮点精度
    currency: &'static str,                      // "CNY" / "USD" / "EUR"
}

impl Money {
    fn new(amount: i64, currency: &'static str) -> Self {
        Money { amount, currency }
    }
}

// Display：带符号和小数点的人类可读格式
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let major = self.amount / 100;
        let minor = (self.amount % 100).abs();
        write!(f, "{}{}.{:02}", self.currency, major, minor)
    }
}

// ── 2. From<T>：实现一个，自动获得 Into ─────────────────────────────────────
// impl From<(i64, &'static str)> for Money 的意思是：
//   从一个 (i64, &'static str) 元组可以构造出 Money
// 有了这个，你就自动获得：
//   · Money::from((1000, "CNY"))
//   · (1000, "CNY").into()   ← Rust 自动帮你转
//
// From / Into 是 Rust 里最常见的「隐式转换」入口
// 推荐实现 From，Into 是免费送的反向能力
impl From<(i64, &'static str)> for Money {
    fn from(tuple: (i64, &'static str)) -> Self {
        Money {
            amount: tuple.0,
            currency: tuple.1,
        }
    }
}

// 也可以从单纯的 i64 构造（默认币种 CNY）
impl From<i64> for Money {
    fn from(amount: i64) -> Self {
        Money {
            amount,
            currency: "CNY",
        }
    }
}

// ── 3. Add / AddAssign：给 Point 实现运算符重载 ─────────────────────────────
// Rust 的运算符本质上都是某个 trait 的方法：
//   ·    + → std::ops::Add           有 a + b
//   ·    - → std::ops::Sub           有 a - b
//   ·    * → std::ops::Mul           有 a * b
//   ·    / → std::ops::Div           有 a / b
//   ·   += → std::ops::AddAssign     有 a += b
//   ·   [] → std::ops::Index         有 a[i]
//
// 为自己的结构体实现这些 trait，你就能自定义运算符行为
// 标准库里的 Vec2、Matrix 等类型全都是这样做的
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
}

// Add<Vec2> for Vec2：Vec2 + Vec2 → Vec2
// 类型参数 Output 决定了 + 的返回类型
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Mul<f64> for Vec2：Vec2 * f64 → Vec2（标量乘法）
// 注意 Mul 的左右两边类型不同：self 是 Vec2，rhs 是 f64
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// AddAssign：让 v += delta 合法
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// ── 4. Default：什么时候手写，什么时候派生 ──────────────────────────────────
// 能派生就派生：#[derive(Default)] 给所有字段调用 T::default()
// 需要手写的场景：
//   · 字段的默认值不是类型的 Default，而是业务意义上的「合理默认」
//   · 希望默认值经过计算（例如时间戳、UUID）
//   · 不希望暴露「所有字段全零」的构造能力
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout_ms: u64,
    max_connections: u32,
}

// 手写 Default：给出「业务合理」的默认值，而不是字段类型的默认值
// 对比派生，这里的 port = 8080、timeout_ms = 3000 更有语义
impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "127.0.0.1".into(),
            port: 8080,
            timeout_ms: 3000,
            max_connections: 1024,
        }
    }
}

// ── 5. Iterator：让结构体能被 for 循环 ──────────────────────────────────────
// Iterator 是 Rust 标准库最核心的 trait 之一
// 只要实现了 next() 方法，你的类型就能用在 for 循环、.map()、.filter()、.collect() 里
//
// 这里做一个斐波那契数列生成器，每次 next() 返回下一个 Fibonacci 数
struct Fibonacci {
    current: u64,
    next: u64,
    count: u32,
    limit: u32,
}

impl Fibonacci {
    fn new(limit: u32) -> Self {
        Fibonacci {
            current: 0,
            next: 1,
            count: 0,
            limit,
        }
    }
}

impl Iterator for Fibonacci {
    // 关联类型：next() 返回 Option<Self::Item>
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.count >= self.limit {
            return None;
        }
        let out = self.current;
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        self.count += 1;
        Some(out)
    }
}

fn main() {
    println!("{}", "=== trait 实现秀 ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、Display vs Debug：面向用户 vs 面向开发者");
    // ─────────────────────────────────────────

    let m = Money::new(12345, "CNY");

    // Debug：自动派生，给开发者看（结构清晰但不美观）
    println!("  Debug   →  {:?}", m);

    // Display：手写实现，给用户看（格式化后干净整洁）
    println!("  Display →  {}", m);

    // 更多 Display 用法
    let salary = Money::new(2500000, "USD");
    let expense = Money::new(-8900, "EUR");
    println!("  薪资 = {}, 支出 = {}", salary, expense);

    println!("  Debug 可以派生（#[derive(Debug)]）；Display 必须手写");
    println!("  规则：面向开发者的数据结构打印用 Debug，面向用户的输出用 Display");
    println!("小结：手写 Display 让你的类型拥有「漂亮、语义化、可控」的展示能力");

    // ─────────────────────────────────────────
    println!("\n2、From<T>：优雅的类型转换");
    // ─────────────────────────────────────────

    // 方法 1：显式 Money::from(...)
    let m1 = Money::from((50000, "USD"));
    println!("  Money::from((50000, \"USD\")) = {}", m1);

    // 方法 2：用 .into()（因为 From 自动给你 Into）
    // 注意：.into() 需要 Rust 能推断出目标类型
    let m2: Money = (12800, "CNY").into();
    println!("  (12800, \"CNY\").into() = {}", m2);

    // 方法 3：从 i64（使用默认币种 CNY）
    let m3: Money = 999.into();
    println!("  999.into() = {}", m3);

    println!("  实现 From<T> 后，Rust 自动给你 Into<T>（免费的反向能力）");
    println!("  函数参数里写 <T: Into<Money>>，调用方就能直接传 (5000, \"USD\")");

    // 函数签名里用 Into：调用方可以传任何「能转成 Money」的类型
    fn print_price(p: impl Into<Money>) {
        let money = p.into();
        println!("  价格: {}", money);
    }

    print_price(1000);                             // i64 → Money
    print_price((5000, "USD"));                    // (i64, &str) → Money
    print_price(Money::new(8888, "EUR"));          // 直接传 Money

    println!("  一次实现 From，调用方获得多种写法 —— 这就是 Rust 的表达力");
    println!("小结：From<T> 是 Rust 里「隐式转换」的正规入口，标准库到处都在用");

    // ─────────────────────────────────────────
    println!("\n3、Add / Mul / AddAssign：运算符重载");
    // ─────────────────────────────────────────

    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);

    let sum = a + b;                               // 触发 Add::add
    println!("  a + b   = {:?}", sum);

    let scaled = a * 3.0;                          // 触发 Mul::mul
    println!("  a * 3.0 = {:?}", scaled);

    let mut acc = Vec2::new(0.0, 0.0);
    acc += a;                                      // 触发 AddAssign::add_assign
    acc += b;
    println!("  acc 累加 a 和 b 后 = {:?}", acc);

    // 链式运算符：(a + b) * 2.0
    let chain = (a + b) * 2.0;
    println!("  (a + b) * 2.0 = {:?}", chain);

    println!("  Rust 的 + - * / += 都是 trait 方法的语法糖");
    println!("  给自己的类型实现对应 trait，你的类型就有了和内建类型一样的语法");
    println!("  数学库（nalgebra、glam）/ 游戏引擎（bevy）全靠这一点写得像公式");
    println!("小结：运算符重载 = trait 实现，让自定义类型拥有「语法级的优雅」");

    // ─────────────────────────────────────────
    println!("\n4、Default：手写 vs 派生");
    // ─────────────────────────────────────────

    // 用手写的 Default：字段都有「业务合理」的默认值
    let cfg = ServerConfig::default();
    println!("  ServerConfig::default() = {:?}", cfg);

    // 配合 ..Default::default() 使用：只覆盖需要定制的字段
    let dev_cfg = ServerConfig {
        port: 9090,                                // 开发用 9090
        timeout_ms: 10000,                         // 长超时便于调试
        ..Default::default()                       // 其他字段用默认
    };
    println!("  定制后的 dev_cfg = {:?}", dev_cfg);

    println!("  为什么手写 Default？");
    println!("    · 派生会调用每个字段的 ::default()，导致 port=0、host=\"\" 等无意义值");
    println!("    · 手写可以给出「业务合理」的默认（8080、3000ms 等），更贴近真实使用");
    println!("    · 配合 ..Default::default() 就有了「命名参数」体验");

    println!("  什么时候派生？字段的类型默认值正好就是你想要的默认时（如 Counter {{ count: 0 }}）");
    println!("小结：Default 是「无参构造」的 Rust 惯例；字段有业务默认值时手写更好");

    // ─────────────────────────────────────────
    println!("\n5、Iterator：让结构体可以被 for 循环");
    // ─────────────────────────────────────────

    // 实现 Iterator 之后，结构体可以用在 for 循环里
    let fibs: Vec<u64> = Fibonacci::new(10).collect();
    println!("  前 10 个 Fibonacci = {:?}", fibs);

    // 甚至可以链式用迭代器适配器
    let even_fibs: Vec<u64> = Fibonacci::new(20)
        .filter(|n| n % 2 == 0)                    // 保留偶数
        .take(5)                                   // 取前 5 个
        .collect();
    println!("  前 20 个 Fibonacci 里的偶数（取前 5 个）= {:?}", even_fibs);

    // for 循环遍历
    print!("  for 循环遍历 Fibonacci::new(8)：");
    for n in Fibonacci::new(8) {
        print!(" {}", n);
    }
    println!();

    println!("  一旦你的类型实现 Iterator，就能和 std 的所有迭代器适配器互动");
    println!("  .map() / .filter() / .fold() / .collect() 全都自动可用");
    println!("小结：Iterator 是 Rust 最重要的 trait 之一，实现 next() 就解锁全部能力");

    // ─────────────────────────────────────────
    println!("\n6、综合：自定义类型 = 一等公民");
    // ─────────────────────────────────────────

    // 现在 Money 和 Vec2 都可以像标准库类型那样优雅地使用：
    //   · {} 打印、{:?} 调试
    //   · from/into 转换
    //   · + / * / += 运算
    //   · for 循环
    //   · 进集合、当 key...（配合 PartialEq/Eq/Hash 派生）
    //
    // 这就是 Rust「trait 开放扩展」的威力：
    // 你写的类型和 String、Vec<T>、HashMap<K, V> 在语言层面是平等的

    let totals: Vec<Money> = vec![
        Money::from(1000),
        Money::from(2500),
        Money::from((5000, "USD")),
    ];
    println!("  所有金额:");
    for m in &totals {
        println!("    - {}", m);
    }

    // Vec2 集合也一样自然
    let moves: Vec<Vec2> = vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(-0.5, 0.5),
    ];
    let total_move = moves.iter().copied().fold(Vec2::new(0.0, 0.0), |acc, v| acc + v);
    println!("  向量累加结果 = {:?}", total_move);

    println!("  自定义类型 + trait 实现 = 和标准库同等地位");
    println!("  这让业务代码可以用「数学/符号化」的语言写出来，读起来像公式");
    println!("小结：Rust 的 trait 系统让你的类型天然融入生态，不只是「数据容器」");

    // ─────────────────────────────────────────
    println!("\n【总结】trait 实现的核心要点");
    // ─────────────────────────────────────────
    println!("  · impl Display       → 自定义面向用户的格式化");
    println!("  · impl From<T>       → 优雅的类型转换入口，免费获得 Into");
    println!("  · impl Add/Mul/...   → 运算符重载，语法级表达力");
    println!("  · impl AddAssign     → += -= 等复合赋值");
    println!("  · impl Default       → 无参构造；业务默认优于字段默认时手写");
    println!("  · impl Iterator      → 让类型融入迭代器生态（map/filter/collect）");
    println!();
    println!("  实现 trait 的孤儿规则：");
    println!("    trait 或 类型 至少有一个是你自己的");
    println!("    想给 Vec<String> 加方法？用 Newtype 包一层（见 04_tuple_structs）");
    println!();
    println!("  Rust 文化：");
    println!("    一个好的库，往往是「精心设计的结构体 + 精心实现的 trait」的组合");
    println!("    从 Serde 到 Tokio，再到 Bevy，无一例外");
    println!();
    println!("  进阶方向：");
    println!("    · 自定义 Deref / DerefMut —— 让类型像「智能指针」一样工作");
    println!("    · 关联类型（type Item）—— trait 里带类型成员");
    println!("    · trait 对象（dyn Trait）—— 动态分发");
    println!("    · blanket impl —— 给所有实现了 A 的类型自动实现 B");
}
