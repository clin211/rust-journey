#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 元组结构体（Tuple Structs） & Newtype 模式
//
// 元组结构体长得像「命名的元组」：它有名字，但字段没有名字，只有位置。
//
//   语法：struct Color(u8, u8, u8);
//   访问：c.0, c.1, c.2
//
// 相比普通结构体：
//   · 更紧凑：不需要给每个字段起名
//   · 但可读性略差：c.0 不如 c.red 直观
//   · 每个元组结构体是独立类型，即使字段类型完全相同，也不能互相赋值
//
// 最重要的应用：Newtype 模式
//   用一个只有「一个字段」的元组结构体包装一个已有类型，
//   形成一个新类型，用来：
//     · 在类型系统里把「概念上不同但底层类型相同」的数据区分开
//       （例：Meters vs Kilograms，都是 f64 但不能互相混用）
//     · 为外部类型实现自己的 trait（绕过"孤儿规则"）
//     · 给已有类型加上额外的语义 / 约束
// ─────────────────────────────────────────────────────────────────────────────

// ── 经典元组结构体 ───────────────────────────────────────────────────────────
// RGB 颜色：3 个 u8，分别代表红、绿、蓝分量
struct Color(u8, u8, u8);

// 3D 空间中的点：每个维度一个 i32 分量
struct Point3D(i32, i32, i32);

// ── 反面教材：字段类型相同，但类型不同，不能混用 ──────────────────────────────
// 这个演示很重要：即使 Color 和 Point3D 的字段都是 3 个数字，
// Rust 在类型系统层面把它们视为完全不同的类型
//
// fn take_color(c: Color) { /* ... */ }
// take_color(Point3D(1, 2, 3));  // ❌ 编译错误：expected Color, found Point3D

// ── Newtype 模式 1：单位安全 ─────────────────────────────────────────────────
// 用 Newtype 包装 f64，把「米」和「千米」变成两个不同的类型
// 好处：函数参数类型不会搞错，单位在类型系统里就表达清楚了
struct Meters(f64);
struct Kilometers(f64);

// 两个函数签名完全不同，编译器在调用点就能拦截单位错误
fn run_meters(d: Meters) {
    println!("  跑了 {:.1} 米", d.0);
}

fn run_kilometers(d: Kilometers) {
    println!("  跑了 {:.2} 千米", d.0);
}

// ── Newtype 模式 2：语义封装 ─────────────────────────────────────────────────
// 用 UserId 包装 u64，让「用户 ID」在类型层面与普通 u64 区分开
// 避免把订单 ID / 帖子 ID 等其他 u64 误传进来
struct UserId(u64);

fn load_user(id: UserId) -> String {
    format!("loaded user #{}", id.0)
}

// ── Newtype 模式 3：包装外部类型，实现自定义方法/trait ────────────────────────
// 直接对 Vec<T> 加方法是不行的（孤儿规则），但可以用 Newtype 套一层
// 在 TaggedList 上挂任何我们想要的方法
struct TaggedList(Vec<String>);

impl TaggedList {
    fn new() -> Self {
        TaggedList(Vec::new())
    }

    fn add(&mut self, tag: &str) {
        self.0.push(tag.to_string()); // 注意 .0 才是内部的 Vec
    }

    fn count(&self) -> usize {
        self.0.len()
    }

    fn join(&self, sep: &str) -> String {
        self.0.join(sep)
    }
}

fn main() {
    println!("{}", "=== 元组结构体 & Newtype 模式 ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、基础元组结构体：Color 和 Point3D");
    // ─────────────────────────────────────────

    // 构造时像函数调用，按位置传参
    let black = Color(0, 0, 0);              // 纯黑
    let red = Color(255, 0, 0);              // 纯红
    let origin = Point3D(0, 0, 0);

    // 通过 .0 / .1 / .2 按位置访问字段
    println!("  black = ({}, {}, {})", black.0, black.1, black.2);
    println!("  red   = ({}, {}, {})", red.0, red.1, red.2);
    println!("  origin = ({}, {}, {})", origin.0, origin.1, origin.2);

    println!("  构造语法很像调用函数，字段访问用 .0 .1 .2");
    println!("小结：元组结构体 = 有名字的元组，字段按位置存取");

    // ─────────────────────────────────────────
    println!("\n2、结构体名称本身就是类型：Color ≠ Point3D");
    // ─────────────────────────────────────────

    // 即使两个元组结构体字段数和类型都一样，也无法互相赋值
    // let c: Color = Point3D(1, 2, 3); // ❌ mismatched types

    // Color 和 Point3D 的字段类型其实不同（u8 vs i32），但更重要的是：
    // 即使字段类型完全一致，它们在 Rust 里仍然是两种完全不同的类型
    struct A(i32, i32);
    struct B(i32, i32);

    let _a = A(1, 2);
    let _b = B(1, 2);
    // let a2: A = B(1, 2);  // ❌ 即使字段相同，A 和 B 是不同类型
    println!("  A 和 B 字段完全相同，但 Rust 视为两种类型，不能互相赋值");

    println!("  这正是元组结构体的价值所在：命名 = 类型，语义不同就是不同类型");
    println!("小结：元组结构体利用「类型系统」防止语义不同但形状相同的数据混用");

    // ─────────────────────────────────────────
    println!("\n3、Newtype 模式 1：让单位进入类型系统");
    // ─────────────────────────────────────────

    let d1 = Meters(5000.0);                 // 5 km == 5000 m
    let d2 = Kilometers(5.0);                // 5 km

    run_meters(d1);                          // ✅ 参数类型匹配
    run_kilometers(d2);                      // ✅ 参数类型匹配

    // ❌ 如果参数传错单位，编译期就拦截
    // run_meters(Kilometers(5.0));          // expected Meters, found Kilometers
    // run_kilometers(Meters(5000.0));       // expected Kilometers, found Meters
    println!("  Newtype 让编译器在参数单位不匹配时直接报错，运行时 0 开销");

    // 需要跨单位换算时，就提供明确的转换函数
    fn to_kilometers(m: Meters) -> Kilometers {
        Kilometers(m.0 / 1000.0)
    }

    let km = to_kilometers(Meters(8000.0));
    println!("  8000 m 转换为 {:.2} km", km.0);

    println!("小结：把「单位」编进类型，在编译期杜绝算术单位混淆");

    // ─────────────────────────────────────────
    println!("\n4、Newtype 模式 2：语义 ID");
    // ─────────────────────────────────────────

    let uid = UserId(42);
    println!("  {}", load_user(uid));

    // ❌ 普通 u64 不能自动当作 UserId
    // load_user(42);                        // expected UserId, found integer
    println!("  普通 u64 不能误传给 load_user（必须显式用 UserId 包装）");
    println!("  这种用法在业务代码里对「别搞错 ID」非常有用");

    println!("小结：Newtype 让 ID 和其它数字一样强类型，杜绝参数混淆");

    // ─────────────────────────────────────────
    println!("\n5、Newtype 模式 3：包装外部类型 + 加方法");
    // ─────────────────────────────────────────

    let mut tags = TaggedList::new();
    tags.add("rust");
    tags.add("ownership");
    tags.add("structs");

    println!("  tag 数量 = {}", tags.count());
    println!("  joined   = {}", tags.join(", "));

    // 若想访问内部的 Vec，就用 tuple 字段 .0
    let first = &tags.0[0];
    println!("  第一个 tag = {first}");

    println!("  为 Vec<String> 直接挂方法会违反「孤儿规则」，Newtype 套一层就 ok");
    println!("小结：Newtype 是 Rust 里「扩展已有类型」的惯用方式");

    // ─────────────────────────────────────────
    println!("\n6、解构元组结构体");
    // ─────────────────────────────────────────

    let color = Color(128, 64, 200);

    // 像解构元组一样解构元组结构体
    let Color(r, g, b) = color;              // r=128, g=64, b=200
    println!("  解构后 r={r}, g={g}, b={b}");

    // 甚至可以在函数参数里直接解构
    fn print_rgb(Color(r, g, b): Color) {
        println!("  解构参数: R={r}, G={g}, B={b}");
    }
    print_rgb(Color(10, 20, 30));

    println!("  也可以用 _ 忽略其中一部分字段");
    let Color(only_r, _, _) = Color(99, 100, 101);
    println!("  只取 R 分量 = {only_r}");

    println!("小结：元组结构体支持元组式解构，参数/let/match 里都能用");

    // ─────────────────────────────────────────
    println!("\n7、元组结构体 vs 普通结构体：什么时候该用哪个？");
    // ─────────────────────────────────────────

    println!("  用元组结构体的场景：");
    println!("    · 字段非常少（通常 1-3 个），字段含义特别直观");
    println!("    · Newtype 模式：一个字段包装外部类型，赋予新语义");
    println!("    · 颜色 RGB、坐标 (x, y)、固定形状的几何体等");

    println!("\n  用普通结构体的场景：");
    println!("    · 字段较多，用 .0 .1 .2 读代码容易乱");
    println!("    · 需要按字段「命名」传达含义的业务数据");
    println!("    · 需要字段初始化简写或更新语法时");

    println!("\n  经验法则：");
    println!("    优先用具名字段结构体；只有在 Newtype 或字段少到「序号即含义」时用元组");
    println!("小结：Newtype 是元组结构体最有价值的用法，其余场景优先具名字段");

    // ─────────────────────────────────────────
    println!("\n【总结】元组结构体与 Newtype");
    // ─────────────────────────────────────────
    println!("  · 语法：struct Color(u8, u8, u8); 实例访问 c.0 / c.1 / c.2");
    println!("  · 类型：每个元组结构体都是独立类型，字段相同也不能互换");
    println!("  · Newtype：单字段元组结构体，用来包装已有类型或赋予新语义");
    println!("  · 场景：单位安全、ID 类型安全、绕过孤儿规则、字段少且含义清晰时");
    println!("  · 解构：像元组一样用 `let Color(r, g, b) = c;`");
    println!("  · 选用：字段多或需要具名传达语义时，还是优先用普通结构体");
}
