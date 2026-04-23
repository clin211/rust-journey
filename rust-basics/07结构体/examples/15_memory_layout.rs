#![allow(dead_code)]

use colored::*;
use std::marker::PhantomData;
use std::mem::{align_of, size_of};

// ─────────────────────────────────────────────────────────────────────────────
// 内存布局与零成本抽象（Memory Layout & Zero-Cost Abstractions）
//
// 这是本章「布道 Rust」的杀手锏。
//
// 很多人听说 Rust 是「零成本抽象」时的第一反应是：
//   "这不就是个口号吗？任何抽象都有开销吧。"
//
// 本示例用 `std::mem::size_of` 和 `align_of` 给你看硬证据：
//
//   1. 基础类型的大小和对齐
//   2. 结构体的大小不是字段总和 —— 有 padding（内存对齐）
//   3. 字段声明顺序影响 size（实测：换顺序能省 33% 内存）
//   4. 单元结构体和 `()` 是零大小类型（ZST）
//   5. Newtype 是真的零开销：size_of::<Meters>() == size_of::<f64>()
//   6. Option<&T> / Option<Box<T>> 的 null pointer optimization
//   7. `#[repr(C)]` / `#[repr(packed)]` / `#[repr(transparent)]` 如何控制布局
//   8. PhantomData 不占空间，但让编译器记住类型信息
//   9. enum 的 discriminant 大小（简介）
//
// 看完这些你会明白：
//   · Rust 对内存有极强的控制力，并不逊色于 C
//   · 抽象（泛型、Newtype、PhantomData）不仅不慢，有时候比「手写」还优
//   · 类型系统和内存布局是紧密耦合的 —— 这是 Rust 在嵌入式 / 系统编程
//     / FFI 场景下能替代 C 的根本原因
// ─────────────────────────────────────────────────────────────────────────────

// ── 1. 基础数值类型大小：所有 Rust 程序员的常识 ─────────────────────────────
// 这些类型的大小在语言规范里是确定的（除了 usize/isize 依赖平台）

// ── 2. 结构体：字段顺序影响布局 ─────────────────────────────────────────────
// 字段声明顺序不同，Rust 可能给出不同的 size（由于内存对齐）
// 注意 Rust 默认（不带 repr）会做字段重排优化，但下面例子会演示控制方式

// 默认 repr 下的"糟糕"顺序（u8, u64, u8）
// 如果没有重排，大概是 1+7(padding)+8+1+7(padding) = 24 字节
// Rust 默认可能会重排成 8+1+1+padding = 16，因此无保证
#[derive(Debug)]
struct BadOrder {
    a: u8,
    b: u64,
    c: u8,
}

// 紧凑顺序：大的放前面 → 字段重排对此影响较小
#[derive(Debug)]
struct GoodOrder {
    b: u64,
    a: u8,
    c: u8,
}

// 加上 #[repr(C)] 之后，字段顺序严格按声明，布局完全可控、可预测
// 这主要用于 FFI 或数据序列化场景
#[repr(C)]
#[derive(Debug)]
struct CBadOrder {
    a: u8,
    b: u64,
    c: u8,
}

#[repr(C)]
#[derive(Debug)]
struct CGoodOrder {
    b: u64,
    a: u8,
    c: u8,
}

// ── 3. 零大小类型（ZST）── ───────────────────────────────────────────────────
// 单元结构体、空元组、PhantomData 都是 ZST，大小为 0 字节
// 它们在集合里「几乎免费」，也是 Rust 类型系统的高阶用法基础
struct UnitMarker;

#[derive(Debug)]
struct EmptyStruct {}

#[derive(Debug)]
struct OnlyPhantom<T> {
    _marker: PhantomData<T>,
}

// ── 4. Newtype 零成本证明 ───────────────────────────────────────────────────
// Newtype 是「只有一个字段的元组结构体」，通常用来给类型换个身份
// #[repr(transparent)] 明确告诉编译器：Newtype 的布局和内部字段完全一致
// 这保证了：
//   · size_of::<Meters>() == size_of::<f64>()
//   · 内存里 Meters 和 f64 二进制表示一样
//   · FFI 场景可以安全互换（Meters ↔ f64）
#[repr(transparent)]
struct Meters(f64);

// 不加 repr 的 Newtype，Rust 也几乎一定给出相同大小（编译器优化）
// 但没有形式保证，repr(transparent) 才能让它成为「契约」
struct Kilometers(f64);

// 包装 bool 的 Newtype —— 大小也完全等于 bool
#[repr(transparent)]
struct Flag(bool);

// ── 5. 各种 repr 属性 ────────────────────────────────────────────────────────

// 默认 repr（无标记）：
//   · Rust 会做字段重排（field reordering）优化 size
//   · 不保证顺序、不保证与 C 的兼容
//   · 只在「纯 Rust 内部使用」时用

// #[repr(C)]：
//   · 字段严格按声明顺序，和 C 的 struct 布局完全一致
//   · FFI（与 C 代码交换数据）、固定格式序列化的首选
#[repr(C)]
struct FfiPoint {
    x: f64,
    y: f64,
}

// #[repr(packed)]：
//   · 去掉所有 padding，字段紧贴在一起
//   · 代价：可能导致非对齐访问（不同 CPU 可能性能差甚至 crash）
//   · 用于：协议包头、旧格式兼容、极端空间受限场景
#[repr(packed)]
struct PackedHeader {
    version: u8,
    flags: u8,
    length: u32,
}

// #[repr(transparent)]：
//   · 只对「单字段结构体」有效
//   · 保证该结构体的布局与唯一字段完全一致
//   · Newtype 模式的正式契约形式
#[repr(transparent)]
struct UserId(u64);

// ── 6. Option 优化：null pointer optimization ───────────────────────────────
// 你会以为 Option<T> 比 T 多一个字节（tag）
// 但 Rust 对「有天然无效值」的类型（&T / Box<T> / Rc<T> / NonZero...）做了优化：
//   · Option<&T> 的 None 就用「空指针 0x0」表示
//   · 因此 size_of::<Option<&T>>() == size_of::<&T>()！
//
// 这是 Rust 类型系统和编译器优化合力的经典案例

// ── 7. enum 的大小 ───────────────────────────────────────────────────────────
// enum 的 size = max(variants) + discriminant
// 但由于内存对齐，最终大小会比单纯相加更大
#[derive(Debug)]
enum Small {
    A,
    B,
    C,
}

#[derive(Debug)]
enum WithData {
    Int(i32),
    Flag(bool),
    Empty,
}

fn main() {
    println!("{}", "=== 内存布局与零成本抽象 ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、基础数值类型：大小与对齐");
    // ─────────────────────────────────────────

    println!("  {:<15} {:<12} {}", "类型", "size", "align");
    println!("  {:-<15} {:-<12} {}", "", "", "");
    println!("  {:<15} {:<12} {}", "bool", size_of::<bool>(), align_of::<bool>());
    println!("  {:<15} {:<12} {}", "char", size_of::<char>(), align_of::<char>());
    println!("  {:<15} {:<12} {}", "u8", size_of::<u8>(), align_of::<u8>());
    println!("  {:<15} {:<12} {}", "u32", size_of::<u32>(), align_of::<u32>());
    println!("  {:<15} {:<12} {}", "u64", size_of::<u64>(), align_of::<u64>());
    println!("  {:<15} {:<12} {}", "f32", size_of::<f32>(), align_of::<f32>());
    println!("  {:<15} {:<12} {}", "f64", size_of::<f64>(), align_of::<f64>());
    println!("  {:<15} {:<12} {}", "usize", size_of::<usize>(), align_of::<usize>());
    println!("  {:<15} {:<12} {}", "&str", size_of::<&str>(), align_of::<&str>());
    println!("  {:<15} {:<12} {}", "String", size_of::<String>(), align_of::<String>());
    println!("  {:<15} {:<12} {}", "Vec<i32>", size_of::<Vec<i32>>(), align_of::<Vec<i32>>());

    println!("  观察:");
    println!("    · char 是 4 字节（Unicode scalar value），不是 1");
    println!("    · &str 是 16 字节「胖指针」：指针 8 + 长度 8");
    println!("    · String / Vec 都是 24 字节（指针 + 长度 + 容量）");
    println!("    · usize 是平台相关的（64 位系统上 8 字节）");
    println!("小结：Rust 的类型大小可以精确查询，且与 C 基本对齐");

    // ─────────────────────────────────────────
    println!("\n2、结构体字段顺序的影响");
    // ─────────────────────────────────────────

    // 默认 repr，Rust 可能重排字段，结果可能优于你预期
    let bad = size_of::<BadOrder>();
    let good = size_of::<GoodOrder>();

    // #[repr(C)] 严格按声明顺序，差异最明显
    let c_bad = size_of::<CBadOrder>();
    let c_good = size_of::<CGoodOrder>();

    println!("  默认 repr:");
    println!("    struct {{ u8, u64, u8 }} → {} 字节 (Rust 可能重排)", bad);
    println!("    struct {{ u64, u8, u8 }} → {} 字节", good);
    println!();
    println!("  #[repr(C)]（严格按声明顺序）:");
    println!("    struct {{ u8, u64, u8 }} → {} 字节 (看到 padding 了吗？)", c_bad);
    println!("    struct {{ u64, u8, u8 }} → {} 字节", c_good);
    println!();
    println!("  布局可视化（repr(C) 下 u8, u64, u8 的情况）：");
    println!("    | a(1) | padding(7) | b(8) | c(1) | padding(7) | = 24 字节");
    println!();
    println!("    换成 u64, u8, u8 后：");
    println!("    | b(8) | a(1) | c(1) | padding(6) | = 16 字节（省 33%）");

    println!("  教训：FFI / 嵌入式场景下，字段顺序对 size 影响巨大");
    println!("  最佳实践：大字段放前、同类型归组，能显著降低 size");
    println!("小结：Rust 默认会优化布局，但 #[repr(C)] 下字段顺序非常关键");

    // ─────────────────────────────────────────
    println!("\n3、零大小类型（ZST）：真的 0 字节");
    // ─────────────────────────────────────────

    println!("  单元结构体   UnitMarker:        {} 字节", size_of::<UnitMarker>());
    println!("  空结构体     EmptyStruct:       {} 字节", size_of::<EmptyStruct>());
    println!("  空元组       ():                {} 字节", size_of::<()>());
    println!("  空数组       [u8; 0]:           {} 字节", size_of::<[u8; 0]>());
    println!("  PhantomData  OnlyPhantom<u64>:  {} 字节", size_of::<OnlyPhantom<u64>>());

    println!();
    println!("  ZST 的实用价值:");
    println!("    · 类型状态机：Article<Draft> / Article<Published>（状态不占空间）");
    println!("    · trait 挂载：struct SimpleLogger; impl Logger for SimpleLogger {{}}");
    println!("    · 集合标记：HashSet<()> 作为「纯 key」去重集合");
    println!("    · 编译期配置：struct Config<ProductionMode>;");
    println!();
    println!("  Vec<()> 也有实际用处：只需要「元素数量」，不需要值");
    let mut empties: Vec<()> = Vec::new();
    empties.push(());
    empties.push(());
    empties.push(());
    println!("    Vec<()> 有 {} 个元素，但整个 Vec 只占 {} 字节（仅元数据）",
        empties.len(), size_of::<Vec<()>>());

    println!("小结：ZST 让「类型层面的存在」和「运行时内存」解耦，是 Rust 抽象的关键工具");

    // ─────────────────────────────────────────
    println!("\n4、Newtype 零开销证明");
    // ─────────────────────────────────────────

    println!("  size_of::<f64>()          = {}", size_of::<f64>());
    println!("  size_of::<Meters>()       = {}   (#[repr(transparent)])", size_of::<Meters>());
    println!("  size_of::<Kilometers>()   = {}   (无 repr 标注，Rust 也优化到一样)",
        size_of::<Kilometers>());
    println!();
    println!("  size_of::<bool>()         = {}", size_of::<bool>());
    println!("  size_of::<Flag>()         = {}   (Flag = bool)", size_of::<Flag>());
    println!();
    println!("  size_of::<u64>()          = {}", size_of::<u64>());
    println!("  size_of::<UserId>()       = {}   (UserId = u64)", size_of::<UserId>());
    println!();
    println!("  换句话说 —— Newtype 是「类型层面的包装」，不是「数据层面的包装」");
    println!("  编译优化后，Meters 和 f64 在汇编层面完全一样");
    println!();
    println!("  validate(Meters(5000.0)) 和 validate(5000.0) 的机器码完全相同");
    println!("  唯一的区别：前者在编译期被类型系统保护，后者没有");

    println!("小结：Newtype 是 Rust「安全包装 + 零开销」最经典的体现");

    // ─────────────────────────────────────────
    println!("\n5、Option 的 null pointer optimization");
    // ─────────────────────────────────────────

    println!("  size_of::<&i32>()             = {}", size_of::<&i32>());
    println!("  size_of::<Option<&i32>>()     = {}   ← 和 &i32 完全一样！", size_of::<Option<&i32>>());
    println!();
    println!("  size_of::<Box<i32>>()         = {}", size_of::<Box<i32>>());
    println!("  size_of::<Option<Box<i32>>>() = {}   ← Box 的 None 也用 0 表示", size_of::<Option<Box<i32>>>());
    println!();
    println!("  size_of::<u64>()              = {}", size_of::<u64>());
    println!("  size_of::<Option<u64>>()      = {}   ← u64 没有「天然无效值」, 所以多 8 字节（tag + padding）",
        size_of::<Option<u64>>());
    println!();
    println!("  size_of::<std::num::NonZeroU64>()       = {}", size_of::<std::num::NonZeroU64>());
    println!("  size_of::<Option<std::num::NonZeroU64>>() = {}   ← NonZero 类型满足 null pointer optimization",
        size_of::<Option<std::num::NonZeroU64>>());

    println!();
    println!("  重点：");
    println!("    · Option<&T> 和 &T 同大小 —— 因为 &T 不会是 null，0 就是 None 的标记");
    println!("    · Option<Box<T>> 也同理 —— Box 内部也是非零指针");
    println!("    · 想让自定义类型也享受这种优化？用 NonZero / NonNull 系列类型");
    println!();
    println!("  这个优化对链表、树、图等数据结构至关重要");
    println!("  C/C++ 里 null 指针约定要你手动管理，Rust 用类型系统自动保证");

    println!("小结：Option 经过编译器优化，比你想象的更接近「零开销」");

    // ─────────────────────────────────────────
    println!("\n6、repr(packed)：去掉 padding");
    // ─────────────────────────────────────────

    // packed 会去掉所有 padding，size = 字段大小总和
    let p = size_of::<PackedHeader>();
    let normal = size_of::<FfiPoint>();

    println!("  FfiPoint (#[repr(C)])         = {} 字节", normal);
    println!("  PackedHeader (#[repr(packed)]) = {} 字节 (u8 + u8 + u32 = 6, 没有 padding)", p);
    println!();
    println!("  packed 的代价:");
    println!("    · 字段访问可能变慢（非对齐读写）");
    println!("    · 某些架构（ARM 老版）直接崩溃");
    println!("    · 不能直接取字段引用（可能未对齐）");
    println!();
    println!("  典型用途：");
    println!("    · 解析二进制协议（网络包头、文件格式）");
    println!("    · 与严格定义的 C 结构体兼容");
    println!("    · 空间极度敏感的嵌入式场景");

    println!("小结：packed 是「用性能换空间」的极端工具，日常代码不用考虑");

    // ─────────────────────────────────────────
    println!("\n7、enum 的 size：tag + 最大 variant");
    // ─────────────────────────────────────────

    println!("  enum Small {{ A, B, C }} (无 data)       → {} 字节", size_of::<Small>());
    println!("  enum WithData {{ Int(i32), Flag(bool), Empty }} → {} 字节",
        size_of::<WithData>());
    println!();
    println!("  计算规则：");
    println!("    · enum 的 size = max(各 variant 的 size) + discriminant tag");
    println!("    · 再考虑对齐，可能有 padding");
    println!("    · Small 只有 3 个无 data 的 variant，discriminant 1 字节就够");
    println!("    · WithData 最大 variant 是 Int(i32)（4 字节），tag 会 padding 到对齐");

    println!();
    println!("  尖锐的对比：");
    println!("    C 的 union 不记录当前是哪个 variant（需要程序员手动 tag）");
    println!("    Rust 的 enum 自带 tag，配合 match 编译期穷举检查");
    println!("    空间代价：多 1~8 字节 tag；收获：类型安全、模式匹配");

    println!("小结：Rust enum 内存布局是「有 tag 的 union」，比 C 安全但略大");

    // ─────────────────────────────────────────
    println!("\n8、PhantomData：类型信息不占空间");
    // ─────────────────────────────────────────

    // OnlyPhantom<T> 只有一个 PhantomData<T> 字段
    // 它在运行时完全不存在，但编译器把 T 当作这个结构体的「虚拟字段」
    // 这让类型状态机、生命周期追踪、marker trait 等高级特性成为可能

    println!("  OnlyPhantom<u64>  大小 = {} 字节", size_of::<OnlyPhantom<u64>>());
    println!("  OnlyPhantom<Vec<String>> 大小 = {} 字节",
        size_of::<OnlyPhantom<Vec<String>>>());
    println!();
    println!("  不管 T 多大，PhantomData<T> 都是 0 字节");
    println!("  它的作用是:");
    println!("    · 让编译器「记得」这个结构体和 T 有关系（用于变量界定）");
    println!("    · 让借用检查正确跟踪 T 的生命周期");
    println!("    · 实现 Type State 模式（如 Article<Draft> / Article<Published>）");
    println!();
    println!("  这是 Rust 类型系统最优雅的地方之一：");
    println!("    「类型层面的信息」和「运行时的字节」完全解耦");

    println!("小结：PhantomData 让你在零运行时代价下使用类型系统的全部威力");

    // ─────────────────────────────────────────
    println!("\n9、布道重点：Rust 的「零成本抽象」究竟是什么");
    // ─────────────────────────────────────────

    println!("  Rust 创始人 Bjarne Stroustrup（C++ 之父）定义的「零成本抽象」：");
    println!("    1. 你不使用的，你不必为它付出代价");
    println!("    2. 使用的东西，你手写不会更快");
    println!();
    println!("  Rust 在结构体层面做到了这两条：");
    println!("    · Newtype 不占额外空间（#[repr(transparent)]）");
    println!("    · 泛型单态化后等价于「手写多份代码」");
    println!("    · PhantomData 编译后完全消失");
    println!("    · Option<&T> / Option<Box<T>> 和原始指针同大小");
    println!("    · 方法调用 = 普通函数调用（无 vtable，除非用 dyn Trait）");
    println!();
    println!("  对比 Java / Go / JavaScript：");
    println!("    · Java 泛型有类型擦除 + 装箱（Integer vs int 差距巨大）");
    println!("    · Go 空接口 interface{{}} 有类型标签 + 指针间接");
    println!("    · JavaScript 对象全是哈希表（属性查找运行时开销）");
    println!();
    println!("  Rust 同时提供了：");
    println!("    · 高级语言的表达力（trait、泛型、枚举、闭包、迭代器）");
    println!("    · 系统语言的控制力（确定大小、无 GC、精确内存布局）");
    println!();
    println!("  这就是 Rust 能同时进入嵌入式、操作系统、游戏引擎、Web 后端、");
    println!("  编译器、数据库、区块链等全部领域的根本原因。");

    println!("小结：抽象不慢，选错语言才慢 —— Rust 证明了两全其美是可能的");

    // ─────────────────────────────────────────
    println!("\n【总结】内存布局要点速查");
    // ─────────────────────────────────────────
    println!("  · size_of::<T>() / align_of::<T>() 任意类型都能查");
    println!("  · 结构体 size 可能大于字段总和（padding）");
    println!("  · 字段声明顺序 → 布局 → size（大字段放前能省空间）");
    println!("  · 默认 repr 会重排字段优化 size；#[repr(C)] 保证顺序");
    println!("  · 单元结构体 / () / PhantomData / [T; 0] 都是 ZST（0 字节）");
    println!("  · Newtype 零开销（#[repr(transparent)] 是形式保证）");
    println!("  · Option<&T> / Option<Box<T>> 和原类型同大小（null pointer opt）");
    println!("  · NonZero / NonNull 系列类型可享受同样优化");
    println!("  · enum size = max(variant) + tag + 对齐 padding");
    println!();
    println!("  各 repr 总览：");
    println!("    {:<25} {}", "默认（无标注）", "Rust 自己决定布局，可重排字段");
    println!("    {:<25} {}", "#[repr(C)]", "严格按声明顺序，兼容 C，FFI 首选");
    println!("    {:<25} {}", "#[repr(packed)]", "去掉所有 padding，可能导致非对齐访问");
    println!("    {:<25} {}", "#[repr(transparent)]", "单字段结构体与内部字段同布局");
    println!("    {:<25} {}", "#[repr(u8)] / #[repr(i32)]", "控制 enum 的 discriminant 类型");
    println!();
    println!("  推荐学习路径：");
    println!("    · 《Rustonomicon》—— 内存布局、UB、unsafe 的权威指南");
    println!("    · Rust Reference 第 6 章 Type Layout");
    println!("    · cargo-show-asm / cargo-bloat 观察编译结果");
    println!();
    println!("  一句话总结：");
    println!("    Rust 让你在不牺牲安全性和表达力的前提下，获得 C 级别的内存控制权");
}
