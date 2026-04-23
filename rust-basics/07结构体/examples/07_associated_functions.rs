#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 关联函数（Associated Functions）
//
// 关联函数是「挂在类型上、但第一个参数不是 self」的函数。
// 它们属于类型本身，而不是某个实例。
//
//   调用语法：Type::func(args)
//   常见用途：
//     · 构造器（constructor）：返回一个新实例
//     · 工具函数：与类型相关，但不需要现有实例作为输入
//     · 常量 / 静态值：返回某个特殊的实例（如 zero、unit、default）
//
// 典型的构造器命名惯例：
//
//   fn new(...) -> Self          最常见的默认构造器
//   fn from_xxx(src) -> Self     从某个类型转换过来（"from_str", "from_tuple" 等）
//   fn with_xxx(...) -> Self     带自定义配置创建实例（"with_capacity" 等）
//   fn default() -> Self         约定俗成的默认值（derive(Default) 会自动产生）
//   fn zero() / fn origin()      返回「零值 / 原点」等特殊实例
//
// 标准库里到处都是这种模式：
//   · String::from("x")、String::new()、String::with_capacity(n)
//   · Vec::new()、Vec::with_capacity(n)、Vec::from([1, 2, 3])
//   · Box::new(value)
//
// 本示例用 Rectangle 和 Point 详细演示各种关联函数的惯用写法。
// ─────────────────────────────────────────────────────────────────────────────

// ── Rectangle：多种构造器 ────────────────────────────────────────────────────
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 默认构造器，最常见的 new()
    // 返回类型写 Self，Self 是「当前 impl 块对应类型」的别名
    // 相当于写 `-> Rectangle`，但在泛型或多 impl 块里更稳定
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // 命名构造器：构造一个正方形（边长相同）
    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }

    // 命名构造器：从「一个元组」构造矩形
    fn from_tuple(size: (u32, u32)) -> Self {
        let (width, height) = size;
        Rectangle { width, height }
    }

    // 返回「单位矩形」的静态实例工厂
    // 这种命名一般是 unit / zero / origin 等，语义明确
    fn unit() -> Self {
        Rectangle { width: 1, height: 1 }
    }

    // 实例方法（用来和关联函数做对比）：注意它有 &self 参数
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// ── Point：关联常量 + 构造器 ─────────────────────────────────────────────────
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // 关联常量：绑定到类型（而不是实例）的常量，常用作「特殊值」
    // 调用：Point::ORIGIN
    const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

    // 基础构造器
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // 从极坐标 (r, θ) 构造一个点
    fn from_polar(r: f64, theta: f64) -> Self {
        Point {
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }

    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// ── Config：链式 with_xxx 风格的构造器 ───────────────────────────────────────
// 这是一个简化版的 builder：不返回中间 builder 对象，直接在 Config 上返回 Self
// 优点：写法紧凑；缺点：不能强制「必填字段」
struct Config {
    host: String,
    port: u16,
    timeout_ms: u64,
}

impl Config {
    fn new(host: &str) -> Self {
        Config {
            host: host.to_string(),
            port: 8080,                      // 给一个默认端口
            timeout_ms: 3000,                // 给一个默认超时
        }
    }

    // &mut self 形式返回 Self（消费自身并返回新版本）
    // 也可以用 `self` 形式的 fluent API，看项目偏好
    fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self                                 // 返回自己，继续链式调用
    }

    fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    fn describe(&self) -> String {
        format!(
            "host={}, port={}, timeout={}ms",
            self.host, self.port, self.timeout_ms
        )
    }
}

// ── 演示 Self 的不同用法 ────────────────────────────────────────────────────
struct Counter {
    value: i64,
    step: i64,
}

impl Counter {
    // 返回 Self：意思是「当前这个类型」，不必写 Counter
    fn new() -> Self {
        Self { value: 0, step: 1 }           // 构造时也可以用 Self { ... }
    }

    fn with_step(step: i64) -> Self {
        Self { value: 0, step }
    }

    fn reset(&mut self) {
        self.value = 0;
    }
}

fn main() {
    println!("{}", "=== 关联函数（Associated Functions） ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、最常见的构造器：new");
    // ─────────────────────────────────────────

    // 调用语法：Type::func(args)，用两个冒号连接
    let r = Rectangle::new(10, 4);

    println!("  Rectangle::new(10, 4) → {}x{}", r.width, r.height);
    println!("  area = {}", r.area());

    println!("  new 是 Rust 惯用命名，90% 的类型都有这么一个默认构造器");
    println!("小结：关联函数用 :: 调用，类型名 + :: + 函数名 + 参数");

    // ─────────────────────────────────────────
    println!("\n2、命名构造器：square / from_tuple");
    // ─────────────────────────────────────────

    let sq = Rectangle::square(5);
    let ft = Rectangle::from_tuple((7, 2));

    println!("  Rectangle::square(5)       → {}x{}", sq.width, sq.height);
    println!("  Rectangle::from_tuple((7,2)) → {}x{}", ft.width, ft.height);

    println!("  命名构造器用来「更清晰地表达意图」：");
    println!("    - square(side) 比 new(side, side) 更有语义");
    println!("    - from_tuple 比在 new 里搞多种参数重载更清晰");

    println!("小结：构造器可以有多个，命名带动了可读性和意图表达");

    // ─────────────────────────────────────────
    println!("\n3、类型工厂：unit / zero / origin 风格");
    // ─────────────────────────────────────────

    let u = Rectangle::unit();
    println!("  Rectangle::unit() → {}x{}, area = {}",
        u.width, u.height, u.area());

    // Point 的「原点」用关联常量（更合适）
    let o = Point::ORIGIN;                   // 直接取常量，零开销
    println!("  Point::ORIGIN = ({:.1}, {:.1})", o.x, o.y);

    println!("  关联常量（const）和返回固定值的关联函数都可以表达「特殊实例」");
    println!("    - 常量：const ORIGIN: Point = Point {{ x: 0.0, y: 0.0 }};");
    println!("    - 函数：fn origin() -> Self {{ ... }}");

    println!("小结：关联常量更适合「编译期就能算出来」的特殊实例");

    // ─────────────────────────────────────────
    println!("\n4、从极坐标构造 Point：from_polar");
    // ─────────────────────────────────────────

    // 绕原点转一圈，打印 4 个点
    let radius = 2.0;
    let pi = std::f64::consts::PI;

    for i in 0..4 {
        let angle = i as f64 * pi / 2.0;     // 0, π/2, π, 3π/2
        let p = Point::from_polar(radius, angle);
        let d = p.distance_to(&Point::ORIGIN);
        println!("  polar(r=2, θ={:>4.2}π): p=({:>5.2}, {:>5.2}), dist=(dist={:.2})",
            angle / pi, p.x, p.y, d);
    }

    println!("  from_polar / from_str / from_tuple 等都是「从外部形式构造 Self」的命名");
    println!("小结：from_xxx 是非常标准的 Rust 构造器命名");

    // ─────────────────────────────────────────
    println!("\n5、链式构造器 with_xxx：像 builder 但更轻量");
    // ─────────────────────────────────────────

    // Config::new 只要 host，剩下字段有默认值
    // 然后用 with_port / with_timeout 链式地覆盖
    let cfg = Config::new("localhost")
        .with_port(9090)
        .with_timeout(5000);

    println!("  cfg = {}", cfg.describe());

    // 没有 with_xxx 调用时，默认值生效
    let cfg_default = Config::new("example.com");
    println!("  默认: {}", cfg_default.describe());

    println!("  with_xxx 链式调用的优点：可读性强，不用一大串位置参数");
    println!("  对比：Config::new(\"localhost\", 9090, 5000) 看多了会分不清谁是谁");

    println!("小结：字段多、需要可选参数时，优先考虑 with_xxx 链式调用");

    // ─────────────────────────────────────────
    println!("\n6、Self 是什么？");
    // ─────────────────────────────────────────

    // Self（大写 S）是当前 impl 块对应类型的「别名」
    // - 在方法签名里：fn foo() -> Self 等价于 fn foo() -> Counter
    // - 在方法体里：  Self { ... }      等价于 Counter { ... }
    //
    // 使用 Self 的好处：
    //   · 代码对类型名变化更鲁棒（改类型名时不用处处改）
    //   · 在泛型 impl 中更通用（impl<T> Foo<T> 里 Self 自动带 T）
    //   · 与 trait 方法里的 Self 完全一致，有利于抽象

    let mut c1 = Counter::new();
    c1.value = 10;
    println!("  Counter::new()       → value={}, step={}", c1.value, c1.step);

    let c2 = Counter::with_step(5);
    println!("  Counter::with_step(5) → value={}, step={}", c2.value, c2.step);

    c1.reset();
    println!("  c1.reset() 后 value = {}", c1.value);

    println!("  在 impl Counter 里，Self 就是 Counter 的别名；");
    println!("  在 impl<T> Box<T> 里，Self 就是 Box<T> 的别名。");
    println!("小结：Self 是当前 impl 块的「当前类型」，非常适合做构造器返回类型");

    // ─────────────────────────────────────────
    println!("\n7、关联函数 vs 方法：怎么区分");
    // ─────────────────────────────────────────

    // 关联函数：没有 self 参数（或者不叫 self）
    //   · 通过 Type::func() 调用
    //   · 常用于构造器、工具函数

    // 方法：第一个参数是 self / &self / &mut self
    //   · 通过 x.func() 调用（也可以用 Type::func(&x) 完全限定调用）
    //   · 操作「现有实例」

    // 对比示例：
    let r1 = Rectangle::new(3, 4);           // 关联函数：创建实例
    let a1 = r1.area();                       // 方法：用现有实例计算

    let a2 = Rectangle::area(&r1);            // 方法的「完全限定调用」
    assert_eq!(a1, a2);

    println!("  r1.area()         → 方法调用语法（推荐写法）");
    println!("  Rectangle::area(&r1) → 完全限定调用（等价，用来区分同名方法时有用）");
    println!("  关联函数没有 self，只能写 Rectangle::new(3, 4)，不能写 (什么).new()");

    println!("小结：是否有 self → 决定它是方法还是关联函数");

    // ─────────────────────────────────────────
    println!("\n8、Default trait：标准库里的「无参构造器」");
    // ─────────────────────────────────────────

    // 很多类型实现了 Default trait，提供统一的默认值构造
    // #[derive(Default)] 会自动给字段全部为 Default 的结构体派生
    #[derive(Default, Debug)]
    struct Settings {
        volume: u8,    // u8 的 default() 是 0
        muted: bool,   // bool 的 default() 是 false
        name: String,  // String 的 default() 是 ""
    }

    let s1: Settings = Default::default();   // 显式调用 Default::default()
    let s2 = Settings::default();            // 更常见的写法
    println!("  Settings::default() = {:?}", s1);
    println!("  s1 == s2？→ 字段都是默认值，内容一致: {:?}", s2);

    // 常常和 ..Default::default() 配合，只覆盖部分字段
    let s3 = Settings {
        volume: 80,
        ..Default::default()
    };
    println!("  部分覆盖 Settings {{ volume: 80, ..default }} = {:?}", s3);

    println!("  Default::default() 是一种跨类型的「无参构造器」，常配合 .. 语法使用");
    println!("小结：当结构体字段普遍有合理默认值时，派生 Default 比写 new() 更省事");

    // ─────────────────────────────────────────
    println!("\n【总结】关联函数要点");
    // ─────────────────────────────────────────
    println!("  · 语法：在 impl 块里，第一个参数不是 self");
    println!("  · 调用：Type::func(args)，用 :: 而不是 .");
    println!("  · 用途：构造器、工具函数、工厂、特殊值（零/单位等）");
    println!("  · 命名惯例：new / from_xxx / with_xxx / default / zero / unit ...");
    println!("  · 返回 Self：用 Self 代替具体类型名，更通用也更鲁棒");
    println!("  · 关联常量：const NAME: Type = ...; 也能挂在类型上");
    println!("  · Default trait：派生 Default 可自动获得 Type::default()");
}
