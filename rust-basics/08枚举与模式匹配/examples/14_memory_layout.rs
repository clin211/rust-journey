//! 14. 枚举的内存布局：discriminant、最大变体、Niche / NPO
//!
//! 运行：cargo run --example 14_memory_layout
//!
//! 本例覆盖：
//! - 枚举一定有"判别式"（discriminant），用来区分当前是哪个变体
//! - enum 的总大小 = max(变体大小) + tag + 对齐 padding
//! - Niche / Null Pointer Optimization：Option<&T> 居然和 &T 一样大！
//! - #[repr(u8)] / #[repr(i32)] 可以指定判别式宽度
//! - 与 Result<(), E> 的"零成本错误处理"

#![allow(dead_code, unused_variables)]

use std::mem::{size_of, align_of};

// ============================================================================
// 1. discriminant：每个 enum 实例都自带一个"我是谁"的标签
// ============================================================================
//
// 每个 enum 实例运行时都需要某种方式表达"我现在是哪个变体"。
// Rust 给每个变体分配一个整数判别式（discriminant），缺省从 0 开始递增：
//
//   enum Direction { Up = 0, Down = 1, Left = 2, Right = 3 }
//
// 编译器生成的内存布局（默认）大致是：
//
//   Direction:        [tag: u8 ]    (1 字节，因为变体数量 <= 256)
//
//   多变体 enum + payload:
//   ┌────┬───────────────────────────────────────┐
//   │tag │  payload (按"最大变体"对齐的存储空间)   │
//   └────┴───────────────────────────────────────┘
//
// 当 enum 没有数据时 size 通常等于 "够装下 tag 的最小整数"。

#[derive(Debug, Clone, Copy)]
enum Direction { Up, Down, Left, Right }

#[derive(Debug)]
enum Mixed {
    A,
    B(u8),
    C(u32, u32),
    D { name: String },
}

// ============================================================================
// 2. 显式指定 discriminant 与 #[repr]
// ============================================================================
//
// 不带数据的枚举可以像 C 一样指定数值，方便和外部协议 / FFI 对齐：

// HTTP 状态码会到 500+，所以用 u16 当判别式宽度
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
    Unknown = 0,
}

// 可以用 `as` 转成数值
fn http_code(s: HttpStatus) -> u16 {
    s as u16                       // 注意：这里是 u16，因为 #[repr(u16)]
}

// ============================================================================
// 3. enum 大小取决于"最大变体"
// ============================================================================
//
// 一个 enum 不管当前是哪种变体，运行时都要预留"最大变体"的空间。
// 这也是为什么递归 enum 必须用 Box / Rc 打破循环（11 章已讲）。

#[derive(Debug)]
enum SmallVariant {
    OnlyByte(u8),                 // 1 字节
}

#[derive(Debug)]
enum SkewedVariants {
    Tiny,                         // 0 字节
    Small(u8),                    // 1 字节
    HugeOne(u64, u64, u64, u64),  // 32 字节 → 整个 enum 至少 ~40 字节
}

// ============================================================================
// 4. Niche optimization：Option<NonZero> 与 Option<&T> 不再多一字节
// ============================================================================
//
// 如果 inner 类型有"绝不会出现的位模式"（niche），编译器会把 `None` 编码成那个位模式，
// 不再额外占 tag 字节。最经典的例子：
//
//   - 引用 &T / &mut T  绝不能为 0
//   - Box<T> / Vec<T> / String 内部指针也不为 0
//   - NonZeroU32 / NonZeroI64 等专门标了"不为 0"
//
// 这让 Option<&T> 的大小和 &T 完全一样：
//
//   size_of::<&T>()         == 8     (64-bit)
//   size_of::<Option<&T>>() == 8     ← 和裸指针一样大！0 当 None
//
// Rust 的"零成本抽象" 在 enum 上的最小实例就是这个。

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,      // Option<Box<...>> 不会比 Box<...> 大，0 表示 None
}

// ============================================================================
// 5. Result<(), E> 也是零成本
// ============================================================================
//
// 当 T = ()（unit）时，Result<(), E> 就退化成"E + 一位 Ok/Err 区分"，
// 大小通常和 E 相同或仅多 1 字节，绝对不会比手写 "C 风格 errno" 慢。

#[derive(Debug)]
enum MyErr { Code(i32) }

// ============================================================================
// 6. align_of：对齐要求
// ============================================================================
//
// enum 的对齐 = 它持有的最大对齐字段的对齐。
// 这通常会让"小变体 + 大变体"组合的 enum 出现 padding：
//
//   enum X { Tiny, Big(u64) }
//   ── 整体对齐 = 8（u64 要求 8 字节对齐）
//   ── 整体 size = 16（tag 占 1，padding 7，u64 占 8）

#[derive(Debug)]
enum Padded {
    A,
    B(u64),
}

// ============================================================================
// 7. 实测：把所有相关 size_of 集中打印
// ============================================================================

fn pretty_size<T>(name: &str) {
    println!("  {:30} size={:>3} B   align={} B", name, size_of::<T>(), align_of::<T>());
}

fn main() {
    println!("===== 1. 不带数据的小枚举 =====");
    pretty_size::<Direction>("Direction");
    println!("    Direction::Up   as u8 = {}", Direction::Up as u8);
    println!("    Direction::Down as u8 = {}", Direction::Down as u8);
    println!("    Direction::Right as u8 = {}", Direction::Right as u8);

    println!("\n===== 2. 显式 #[repr(u16)] + 显式 discriminant =====");
    pretty_size::<HttpStatus>("HttpStatus (#[repr(u16)])");
    println!("    http_code(NotFound)     = {}", http_code(HttpStatus::NotFound));
    println!("    http_code(Ok)           = {}", http_code(HttpStatus::Ok));

    println!("\n===== 3. 带数据的枚举：大小 = max(变体) + tag =====");
    pretty_size::<Mixed>("Mixed");
    pretty_size::<SmallVariant>("SmallVariant");
    pretty_size::<SkewedVariants>("SkewedVariants");
    println!("    SkewedVariants::Tiny 也要预留 HugeOne 大小的空间");

    println!("\n===== 4. Niche / NPO 实例 =====");
    pretty_size::<&i32>("&i32");
    pretty_size::<Option<&i32>>("Option<&i32>");
    pretty_size::<Box<i32>>("Box<i32>");
    pretty_size::<Option<Box<i32>>>("Option<Box<i32>>");
    pretty_size::<Vec<u8>>("Vec<u8>");
    pretty_size::<Option<Vec<u8>>>("Option<Vec<u8>>");
    pretty_size::<Option<bool>>("Option<bool>");
    pretty_size::<Option<u8>>("Option<u8>");

    println!("\n  ⚡ 注意:");
    println!("  · Option<&i32> 与 &i32 一样大       ─ 0 当 None，不需要额外 tag");
    println!("  · Option<Box<T>>、Option<Vec<T>>、Option<String> 同理");
    println!("  · Option<u8> 必须额外存 tag（u8 的 0 是合法值，没有 niche）");
    println!("  · Option<bool> 命中 niche（bool 只有 0/1 两个合法位模式）");

    println!("\n===== 5. Result<(), MyErr> 实测 =====");
    pretty_size::<()>("()");
    pretty_size::<MyErr>("MyErr");
    pretty_size::<Result<(), MyErr>>("Result<(), MyErr>");

    println!("\n===== 6. 对齐与 padding =====");
    pretty_size::<Padded>("Padded { A, B(u64) }");
    println!("    align=8 → tag(1) + padding(7) + u64(8) = 16 字节");

    println!("\n===== 7. 链表节点 Niche =====");
    pretty_size::<Node>("Node { i32, Option<Box<Node>> }");
    println!("    next 的 Option 不再多占字节，Box 内部指针的 0 当 None");

    println!("\n===== 要点回顾 =====");
    println!("· 每个 enum 实例都隐含一个 discriminant tag");
    println!("· enum 的 size = max(变体 payload) + tag + padding");
    println!("· Niche optimization：内部类型有非法位模式时，None 借用之，省下 tag");
    println!("· #[repr(u8)] / #[repr(i32)] 让你掌控 tag 宽度（FFI / 协议必备）");
    println!("· Rust 的 Option / Result 在大多数情况下做到了 0 运行时开销");
}
