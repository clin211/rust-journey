//! 03. 为枚举实现方法、关联函数、派生宏
//!
//! 运行：cargo run --example 03_enum_methods
//!
//! 本例覆盖：
//! - `impl` 块挂方法（&self / &mut self / self）
//! - 关联函数：构造器、parse-like 方法
//! - 常用派生：Debug / Clone / Copy / PartialEq / Eq / Hash / Default
//! - `Self` 类型别名 + 多个 impl 块

#![allow(dead_code)]
// 注：CNY/USD/EUR 是国际通用的货币代码缩写, 保持业界惯例
#![allow(clippy::upper_case_acronyms)]

use std::fmt;

// ============================================================================
// 1. 基础：在 enum 上挂方法
// ============================================================================
//
// 和 struct 一样，enum 也可以有 impl 块，方法接收者可选 &self / &mut self / self。
// 不同变体在方法体里靠 match 区分。

// 注意：含有 f64 的 enum 只能 derive PartialEq，不能 derive Eq —— f64 满足不了
// "完全相等" 的语义（NaN != NaN）。要进 HashMap 的 key 请见下面 Currency 那个例子。
#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Circle(f64),                     // 半径
    Square(f64),                     // 边长
    Rectangle { w: f64, h: f64 },
}

impl Shape {
    /// &self：只读，最常见。计算面积
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Square(s) => s * s,
            Shape::Rectangle { w, h } => w * h,
        }
    }

    /// &self：判定
    fn is_round(&self) -> bool {
        matches!(self, Shape::Circle(_))
    }

    /// 关联函数（构造器）：长得像 String::from / Vec::new
    fn unit_circle() -> Self {
        Shape::Circle(1.0)
    }

    /// 关联函数：从 (w, h) 元组构造
    fn from_size((w, h): (f64, f64)) -> Self {
        if (w - h).abs() < f64::EPSILON {
            Shape::Square(w)
        } else {
            Shape::Rectangle { w, h }
        }
    }

    /// self：消费自身、返回新值（"扩展类"接口）
    fn scale(self, k: f64) -> Self {
        match self {
            Shape::Circle(r) => Shape::Circle(r * k),
            Shape::Square(s) => Shape::Square(s * k),
            Shape::Rectangle { w, h } => Shape::Rectangle { w: w * k, h: h * k },
        }
    }
}

// ============================================================================
// 2. 多个 impl 块按主题分组
// ============================================================================
//
// 和结构体一样，可以为同一个 enum 写多个 impl 块。常用来：
// - 把"业务方法"和"trait 实现"拆开
// - 给不同 trait 各占一个 impl 块

impl Shape {
    /// 周长（按主题分类，单独占一个 impl 块）
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Square(s) => 4.0 * s,
            Shape::Rectangle { w, h } => 2.0 * (w + h),
        }
    }
}

// trait 实现单独成块：把 Shape 按"自己的格式"打印
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Circle(r) => write!(f, "Circle(r={r})"),
            Shape::Square(s) => write!(f, "Square({s}x{s})"),
            Shape::Rectangle { w, h } => write!(f, "Rect({w}x{h})"),
        }
    }
}

// ============================================================================
// 3. Default：给 enum 一个"默认变体"
// ============================================================================
//
// `#[derive(Default)]` 直接派生 Default 时，必须用 `#[default]` 标注哪个变体是默认的。
// 这是 Rust 1.62+ 的能力。

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum LogLevel {
    Trace,
    Debug,
    #[default]                           // ← 默认变体
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// 关联函数：尽力把字符串解析为 LogLevel；失败返回 None
    fn parse(s: &str) -> Option<Self> {
        Some(match s.to_ascii_lowercase().as_str() {
            "trace" => Self::Trace,
            "debug" => Self::Debug,
            "info" => Self::Info,
            "warn" | "warning" => Self::Warn,
            "error" => Self::Error,
            _ => return None,
        })
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

// ============================================================================
// 4. PartialEq / Eq / Hash：让 enum 能当 HashMap 的 key
// ============================================================================
//
// - PartialEq:  支持 ==
// - Eq:         "完全相等"标记 (要求所有数据字段都 Eq)
// - Hash:       要能作为 HashMap/HashSet 的 key，必须同时实现
//
// 派生这三个之后，enum 就能像 String / i32 一样进哈希表。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Currency {
    CNY,
    USD,
    EUR,
    Custom(String),       // String 也是 PartialEq + Eq + Hash
}

// ============================================================================
// 5. &mut self：在 enum 自身做就地修改
// ============================================================================
//
// 当 enum 实例需要"原地切换变体或修改数据"时，用 &mut self。
// 注意：在 match 里改变 self 自身的变体时，常用 std::mem::replace / take。

#[derive(Debug)]
enum Counter {
    Off,
    On { value: u32 },
}

impl Counter {
    fn turn_on(&mut self) {
        // 不管原来是哪个变体，统一切到 On{value:0}
        *self = Counter::On { value: 0 };
    }

    fn inc(&mut self) {
        match self {
            Counter::Off => { /* 关机时忽略 */ }
            Counter::On { value } => *value += 1,
        }
    }

    fn turn_off(&mut self) {
        *self = Counter::Off;
    }
}

fn main() {
    println!("===== 1. Shape 上的方法 =====");
    let shapes = [
        Shape::Circle(2.0),
        Shape::Square(3.0),
        Shape::Rectangle { w: 4.0, h: 5.0 },
    ];
    for s in &shapes {
        println!(
            "{s:<14} 面积={:.2}  周长={:.2}  圆形={}",
            s.area(),
            s.perimeter(),
            s.is_round()
        );
    }

    let big = Shape::unit_circle().scale(10.0);
    println!("放大后的单位圆: {big}, 面积={:.2}", big.area());

    println!("\n===== 2. 关联函数构造 =====");
    let s1 = Shape::from_size((4.0, 4.0));
    let s2 = Shape::from_size((4.0, 5.0));
    println!("from_size((4,4)) = {s1}");
    println!("from_size((4,5)) = {s2}");

    println!("\n===== 3. LogLevel 默认值 + 解析 =====");
    let default_lv = LogLevel::default();
    println!("默认日志等级: {default_lv:?}");
    for input in ["debug", "WARNING", "fatal", "info"] {
        println!("  parse({input:?}) = {:?}", LogLevel::parse(input));
    }
    println!("Info.as_str() = {}", LogLevel::Info.as_str());

    println!("\n===== 4. Currency 作为 HashMap key =====");
    use std::collections::HashMap;
    let mut rates: HashMap<Currency, f64> = HashMap::new();
    rates.insert(Currency::CNY, 1.0);
    rates.insert(Currency::USD, 7.2);
    rates.insert(Currency::EUR, 7.8);
    rates.insert(Currency::Custom("BTC".into()), 500_000.0);

    for (c, r) in &rates {
        println!("  1 {c:?} = {r} CNY");
    }

    println!("\n===== 5. Counter 状态切换 =====");
    let mut c = Counter::Off;
    println!("初始: {c:?}");
    c.turn_on();
    println!("开机: {c:?}");
    c.inc();
    c.inc();
    c.inc();
    println!("3 次自增: {c:?}");
    c.turn_off();
    println!("关机: {c:?}");

    println!("\n===== 要点回顾 =====");
    println!("· enum 也支持 impl 块，方法接收者三种全都能用");
    println!("· 关联函数用 :: 调用，是常见的 'parse / from_xxx / new' 入口");
    println!("· #[derive(Default)] 配合 #[default] 给枚举默认变体");
    println!("· #[derive(PartialEq, Eq, Hash)] 让枚举能进 HashMap");
}
