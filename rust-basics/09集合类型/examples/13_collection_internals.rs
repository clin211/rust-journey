//! 13. 集合内部结构：Vec / String / HashMap / BTreeMap 的内存与扩容
//!
//! 运行：cargo run --example 13_collection_internals
//!
//! 本例覆盖：
//! - Vec / String 的胖指针 (ptr, len, cap)
//! - 容量增长策略：从小到大 push 看 cap 怎么变
//! - with_capacity 的真实价值
//! - HashMap / HashSet 的"装填因子" 与 reallocation
//! - BTreeMap 的 B-Tree 性质 (最少 6 节点 / 最多 11 节点的 B-树)
//! - 选型对比表：访问模式 vs 容器

#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::mem::size_of;

// ============================================================================
// 1. 栈上大小：所有"动态容器"都是胖指针
// ============================================================================

fn stack_sizes() {
    println!("  size_of::<Vec<i32>>()       = {} B (ptr+len+cap)", size_of::<Vec<i32>>());
    println!("  size_of::<String>()         = {} B (= Vec<u8>)", size_of::<String>());
    println!("  size_of::<HashMap<u64, u64>>() = {} B", size_of::<HashMap<u64, u64>>());
    println!("  size_of::<BTreeMap<u64, u64>>() = {} B", size_of::<BTreeMap<u64, u64>>());
    println!();
    println!("  → 容器在栈上都是固定大小, 真正的数据存堆上");
    println!("  → Vec / String 都是 24 B (3 个 usize: ptr/len/cap)");
}

// ============================================================================
// 2. Vec 容量增长曲线
// ============================================================================
//
// Rust 标准库的 Vec 扩容策略 (实现细节, 不是稳定保证):
//
//   首次 push: cap 变成 4 (或者你 with_capacity 给的)
//   之后扩容: cap 翻倍
//
// 这能让 push 摊还 O(1) (n 次 push 总共拷贝 ≈ 2n 个元素).
// 但每次扩容都要分配新堆 + memcpy, 所以在"已知规模"时强烈推荐 with_capacity.

fn vec_growth() {
    println!("  --- 不预分配 ---");
    let mut v: Vec<i32> = Vec::new();
    let mut prev = 0;
    for i in 1..=20 {
        v.push(i);
        if v.capacity() != prev {
            println!("    push #{i:2}: len={}, cap={}", v.len(), v.capacity());
            prev = v.capacity();
        }
    }

    println!("  --- with_capacity(20) ---");
    let mut v: Vec<i32> = Vec::with_capacity(20);
    for i in 1..=20 {
        v.push(i);
        if v.capacity() != 20 {
            println!("    push #{i}: capacity 变了 → {}", v.capacity());
        }
    }
    println!("    所有 push 都不需要 realloc, len={}, cap={}", v.len(), v.capacity());
}

// ============================================================================
// 3. shrink_to_fit / shrink_to
// ============================================================================
//
// 已经分配但用不上时, 可以缩容把多余内存还给 allocator。
// 这通常发生在: "构建期大量 push, 之后只读" 的场景。

fn shrink_demo() {
    let mut v: Vec<u64> = (0..1000).collect();
    v.truncate(10);
    println!("  truncate(10) 后: len={}, cap={}", v.len(), v.capacity());

    v.shrink_to_fit();
    println!("  shrink_to_fit:    len={}, cap={}", v.len(), v.capacity());

    let mut v: Vec<u64> = (0..1000).collect();
    v.truncate(10);
    v.shrink_to(50);                     // 至少保留 50 槽位
    println!("  shrink_to(50):    len={}, cap={}", v.len(), v.capacity());
}

// ============================================================================
// 4. HashMap 的扩容
// ============================================================================
//
// HashMap 维护"装填因子" (load factor): 元素数 / 桶数。
// 默认装填因子上限大约 0.875, 超过就 resize 把桶数翻倍。
// 同样的: 已知规模时用 with_capacity 避免反复 resize。

fn hashmap_growth() {
    let mut m: HashMap<i32, i32> = HashMap::new();
    println!("  初始: cap = {}", m.capacity());
    let mut prev = m.capacity();
    for i in 0..200 {
        m.insert(i, i);
        if m.capacity() != prev {
            println!("    after insert #{}: cap = {}", i + 1, m.capacity());
            prev = m.capacity();
        }
    }
    println!("  → 装填因子触发后, cap 翻倍重哈希");
}

// ============================================================================
// 5. 用一个简单 benchmark 看 with_capacity 收益
// ============================================================================

fn with_capacity_perf() {
    use std::time::Instant;
    let n = 1_000_000;

    // 不预分配
    let t = Instant::now();
    let mut v: Vec<i32> = Vec::new();
    for i in 0..n { v.push(i); }
    let t1 = t.elapsed();

    // with_capacity
    let t = Instant::now();
    let mut v: Vec<i32> = Vec::with_capacity(n as usize);
    for i in 0..n { v.push(i); }
    let t2 = t.elapsed();

    println!("  push {n} 次:");
    println!("    Vec::new()          = {t1:?}");
    println!("    with_capacity({n})  = {t2:?}");

    // HashMap 同理
    let t = Instant::now();
    let mut m: HashMap<i32, i32> = HashMap::new();
    for i in 0..(n / 10) { m.insert(i, i); }
    let t1 = t.elapsed();

    let t = Instant::now();
    let mut m: HashMap<i32, i32> = HashMap::with_capacity((n / 10) as usize);
    for i in 0..(n / 10) { m.insert(i, i); }
    let t2 = t.elapsed();

    println!("  insert {} 次 (HashMap):", n / 10);
    println!("    HashMap::new()                  = {t1:?}");
    println!("    HashMap::with_capacity({})  = {t2:?}", n / 10);
}

// ============================================================================
// 6. 选型对比表
// ============================================================================

fn selection_chart() {
    println!("  ┌─────────────────────────────┬──────────────────┬─────────────────────────┐");
    println!("  │ 访问模式                     │ 容器             │ 关键操作复杂度           │");
    println!("  ├─────────────────────────────┼──────────────────┼─────────────────────────┤");
    println!("  │ 顺序访问、随机索引             │ Vec<T>           │ 索引 O(1), push 摊还 O(1) │");
    println!("  │ 双端 push/pop                │ VecDeque<T>      │ 前后端均 O(1)            │");
    println!("  │ 优先队列, 取最值              │ BinaryHeap<T>    │ push/pop O(log n)        │");
    println!("  │ 键值查找 (无序、最快)         │ HashMap<K, V>    │ get/insert O(1) 平均     │");
    println!("  │ 键值查找 (有序、范围)         │ BTreeMap<K, V>   │ get/insert O(log n)      │");
    println!("  │ 元素去重                     │ HashSet<T>       │ contains O(1) 平均       │");
    println!("  │ 元素去重 + 有序               │ BTreeSet<T>      │ contains O(log n)        │");
    println!("  │ 文本                         │ String           │ 同 Vec<u8>               │");
    println!("  │ 不可变只读视图                │ &[T] / &str      │ 零分配, 借用              │");
    println!("  └─────────────────────────────┴──────────────────┴─────────────────────────┘");
}

// ============================================================================
// 7. 一个常见的"性能小坑" 演示
// ============================================================================
//
// 在循环里反复 String::new() + push_str 会反复分配, 不如用一个 String 在外面 reuse:

fn buffer_reuse_demo() {
    use std::time::Instant;

    let n = 100_000;
    let words = vec!["hello", "world", "rust", "is", "fast"];

    // 反复 new
    let t = Instant::now();
    let mut total = 0usize;
    for _ in 0..n {
        let mut s = String::new();
        for w in &words { s.push_str(w); }
        total += s.len();
    }
    let t1 = t.elapsed();

    // 复用 buffer
    let t = Instant::now();
    let mut s = String::new();
    let mut total2 = 0usize;
    for _ in 0..n {
        s.clear();
        for w in &words { s.push_str(w); }
        total2 += s.len();
    }
    let t2 = t.elapsed();

    println!("  反复 new String: {t1:?} (total={total})");
    println!("  复用 buffer:     {t2:?} (total={total2})");
}

fn main() {
    println!("===== 1. 栈上大小 =====");
    stack_sizes();

    println!("\n===== 2. Vec 容量增长 =====");
    vec_growth();

    println!("\n===== 3. shrink_to_fit / shrink_to =====");
    shrink_demo();

    println!("\n===== 4. HashMap 容量增长 =====");
    hashmap_growth();

    println!("\n===== 5. with_capacity 的性能收益 =====");
    with_capacity_perf();

    println!("\n===== 6. 容器选型 =====");
    selection_chart();

    println!("\n===== 7. buffer 复用 =====");
    buffer_reuse_demo();

    println!("\n===== 要点回顾 =====");
    println!("· 容器都是栈上胖指针 + 堆上数据");
    println!("· Vec / String 容量翻倍增长; HashMap 装填因子超阈值时 rehash");
    println!("· 已知规模强烈推荐 with_capacity");
    println!("· 不再增长的 Vec / String 可以 shrink_to_fit 释放多余内存");
    println!("· 紧密循环里 String / Vec 应该 clear() 复用, 而不是 new()");
}
