#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 第七章：结构体（Structs）
//
//   结构体（struct）是 Rust 组织数据的核心手段。它让你把一组相关字段
//   打包成一个自定义类型，并配合 impl 块为这个类型挂上方法与关联函数，
//   形成清晰、紧凑的业务语义。
//
//   本章的三个核心主题：
//     1. 定义数据：三种结构体形态（具名字段 / 元组 / 单元）
//     2. 创建实例：字段初始化、简写、更新语法
//     3. 绑定行为：impl 块、方法接收者、关联函数、常用 derive
//
//   结构体在 Rust 里的独特之处：
//     · 字段默认私有，靠 pub 精准控制可见性（内聚 + 封装）
//     · 没有继承，但靠组合 + trait 可以实现更清晰的抽象
//     · 所有权自然贯通：结构体拥有其字段，字段离开结构体会一起 drop
//     · 部分 move 支持：字段所有权可以被单独拿走，剩余字段仍可用
//
//   本章示例文件（建议按顺序学习）：
//     01_struct_basics          → 定义、实例化、字段访问、可变性
//     02_field_init_shorthand   → 字段初始化简写
//     03_struct_update_syntax   → 结构体更新语法 ..base 与 move 规则
//     04_tuple_structs          → 元组结构体与 Newtype 模式
//     05_unit_structs           → 单元结构体的用途（零大小类型）
//     06_methods                → impl 块、&self / &mut self / self
//     07_associated_functions   → 关联函数（构造器模式、Self 类型）
//     08_debug_and_derives      → #[derive(Debug)]、常用派生、dbg! 宏
//     09_ownership_in_structs   → 结构体所有权、部分 move、String vs &str
//     10_rectangle              → 综合练习：Rectangle + 单元测试（cargo test）
//     11_generics_in_structs    → 泛型结构体、特化 impl、trait bound、单态化
//     12_lifetimes_in_structs   → 结构体生命周期参数、省略规则、&'static
//     13_trait_impl_showcase    → impl Display / From / Add / Iterator（trait 实现秀）
//     14_builder_pattern        → 构建器模式：轻量、独立、Type-State 三层实现
//     15_memory_layout          → 内存布局、零成本抽象、repr 属性、Option 优化
// ─────────────────────────────────────────────────────────────────────────

fn main() {
    println!("{}", "=== 结构体 (Structs) ===".green().bold());

    println!("\n本章涵盖 Rust 结构体从定义到使用的完整链路。");
    println!("先建立「数据 + 行为」的直觉，再延伸到所有权与派生。");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── 示例列表 ──".cyan().bold());

    println!("\n  {}  结构体基础",
        "01_struct_basics".yellow().bold());
    println!("     cargo run --example 01_struct_basics");
    println!("     主题：具名字段 struct 的定义、实例化、字段访问、可变性");

    println!("\n  {}  字段初始化简写",
        "02_field_init_shorthand".yellow().bold());
    println!("     cargo run --example 02_field_init_shorthand");
    println!("     主题：同名变量直接作为字段值、常见函数构造模式");

    println!("\n  {}  结构体更新语法",
        "03_struct_update_syntax".yellow().bold());
    println!("     cargo run --example 03_struct_update_syntax");
    println!("     主题：..base 语法、字段级 move 规则、何时会让原实例失效");

    println!("\n  {}  元组结构体与 Newtype 模式",
        "04_tuple_structs".yellow().bold());
    println!("     cargo run --example 04_tuple_structs");
    println!("     主题：struct Point(i32, i32)、Newtype 做类型安全封装");

    println!("\n  {}  单元结构体",
        "05_unit_structs".yellow().bold());
    println!("     cargo run --example 05_unit_structs");
    println!("     主题：struct AlwaysEqual;、零大小类型、trait 挂载点");

    println!("\n  {}  方法（impl 块）",
        "06_methods".yellow().bold());
    println!("     cargo run --example 06_methods");
    println!("     主题：&self / &mut self / self 的选择、自动引用/解引用");

    println!("\n  {}  关联函数",
        "07_associated_functions".yellow().bold());
    println!("     cargo run --example 07_associated_functions");
    println!("     主题：new() 构造器、多种命名构造器、Self 类型别名");

    println!("\n  {}  Debug 派生与 dbg!",
        "08_debug_and_derives".yellow().bold());
    println!("     cargo run --example 08_debug_and_derives");
    println!("     主题：#[derive(Debug)]、{{:?}} vs {{:#?}}、常用派生宏");

    println!("\n  {}  结构体所有权",
        "09_ownership_in_structs".yellow().bold());
    println!("     cargo run --example 09_ownership_in_structs");
    println!("     主题：字段 move、部分 move、String vs &str 的设计取舍");

    println!("\n  {}  综合练习：Rectangle",
        "10_rectangle".yellow().bold());
    println!("     cargo run --example 10_rectangle");
    println!("     主题：area / can_hold / square / Debug 打印 / 单元测试（cargo test）");

    println!("\n  {}  泛型结构体",
        "11_generics_in_structs".yellow().bold());
    println!("     cargo run --example 11_generics_in_structs");
    println!("     主题：struct Point<T>、特化 impl、trait bound、单态化的零成本");

    println!("\n  {}  结构体生命周期",
        "12_lifetimes_in_structs".yellow().bold());
    println!("     cargo run --example 12_lifetimes_in_structs");
    println!("     主题：struct Foo<'a>、省略规则、多个独立生命周期、&'static 字段");

    println!("\n  {}  trait 实现秀",
        "13_trait_impl_showcase".yellow().bold());
    println!("     cargo run --example 13_trait_impl_showcase");
    println!("     主题：impl Display / From / Add / AddAssign / Iterator 一把梭");

    println!("\n  {}  构建器模式",
        "14_builder_pattern".yellow().bold());
    println!("     cargo run --example 14_builder_pattern");
    println!("     主题：轻量 builder、独立 Builder + Result、Type-State 编译期校验");

    println!("\n  {}  内存布局与零成本抽象",
        "15_memory_layout".yellow().bold());
    println!("     cargo run --example 15_memory_layout");
    println!("     主题：size_of / align_of、字段 padding、repr(C/packed/transparent)");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── 结构体三形态速览 ──".cyan().bold());

    // 1) 具名字段结构体（最常用）
    #[derive(Debug)]
    struct User {
        username: String,
        age: u32,
        active: bool,
    }

    let alice = User {
        username: String::from("Alice"),
        age: 30,
        active: true,
    };
    println!("\n  具名字段结构体 (classic struct):");
    println!("    {:?}", alice);
    println!("    访问: alice.age = {}", alice.age);

    // 2) 元组结构体（适合少量字段，字段名多余）
    #[derive(Debug)]
    struct Point(i32, i32);

    let origin = Point(0, 0);
    println!("\n  元组结构体 (tuple struct):");
    println!("    {:?}", origin);
    println!("    访问: origin.0 = {}, origin.1 = {}", origin.0, origin.1);

    // 3) 单元结构体（零大小、无字段）
    #[derive(Debug)]
    struct Marker;

    let m = Marker;
    println!("\n  单元结构体 (unit struct):");
    println!("    {:?}   （没有字段，仅作为类型标记）", m);

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── 方法接收者速查 ──".cyan().bold());

    println!("\n  {:<15}  只读访问字段 / 计算派生值 / 返回内部引用",  "&self");
    println!("  {:<15}  可变访问字段 / 修改状态",                     "&mut self");
    println!("  {:<15}  消费整个实例 / 转换为其他类型",                "self");
    println!("  {:<15}  关联函数 / 构造器 / 常量工具",                 "无 self");

    println!("\n  命名惯例：");
    println!("    · 构造器通常叫 new()，返回 Self");
    println!("    · 其他命名构造器：from()、with_xxx()、default()");
    println!("    · 消费实例、转换为其他类型的方法常以 into_ 开头");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── 常用派生宏（derive） ──".cyan().bold());

    println!("\n  {:<14}  用 {{:?}} 和 {{:#?}} 打印（必配）",         "Debug");
    println!("  {:<14}  让结构体实例可以按位复制（要求字段都 Copy）", "Copy");
    println!("  {:<14}  让结构体实例可以显式深拷贝",                   "Clone");
    println!("  {:<14}  支持 == 和 != 比较",                           "PartialEq");
    println!("  {:<14}  用作 HashMap key 时必备",                      "Eq + Hash");
    println!("  {:<14}  支持排序比较",                                 "PartialOrd / Ord");
    println!("  {:<14}  提供默认值 Self::default()",                   "Default");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "提示：使用上方 cargo run --example <名称> 命令运行各示例。".bright_black());
}
