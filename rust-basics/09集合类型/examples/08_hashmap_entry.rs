//! 08. HashMap 的 entry API：or_insert / or_insert_with / and_modify
//!
//! 运行：cargo run --example 08_hashmap_entry
//!
//! `entry` 是工程里最高频的 HashMap 写法。
//! 它把"查 + 改 / 不存在就插入"合并成一次哈希查询, 既高效又简洁。
//!
//! 本例覆盖：
//! - or_insert：不存在时插入默认值, 然后返回 &mut V
//! - or_insert_with：默认值需要懒求值时
//! - or_default：用 V::default()
//! - and_modify：存在时先改一下
//! - 三大经典模式：计数器 / 分组 / 缓存
//! - "在不存在时只查不改"的边角情况

#![allow(dead_code)]
// 注：or_insert_with(Vec::new) 是教学场景的清晰写法（让初学者一眼看到默认值是什么），
// 比 or_default() 在初学阶段更直观, 故保留 unwrap_or_default lint
#![allow(clippy::unwrap_or_default)]

use std::collections::HashMap;

// ============================================================================
// 1. 入门：等价写法对比
// ============================================================================
//
// 给定一段词, 统计每个词出现次数。
//
// **没有 entry**：要写一堆分支判断:

fn count_naive(words: &[&str]) -> HashMap<String, i32> {
    let mut m = HashMap::new();
    for w in words {
        if m.contains_key(*w) {
            *m.get_mut(*w).unwrap() += 1;        // 这里查了第二次!
        } else {
            m.insert(w.to_string(), 1);          // 又查了第三次!
        }
    }
    m
}

// **用 entry**：一次哈希查询:

fn count_entry(words: &[&str]) -> HashMap<String, i32> {
    let mut m = HashMap::new();
    for w in words {
        *m.entry(w.to_string()).or_insert(0) += 1;
    }
    m
}

// ============================================================================
// 2. or_insert / or_insert_with / or_default
// ============================================================================

fn or_insert_demo() {
    let mut m: HashMap<&str, i32> = HashMap::new();

    // or_insert: 不存在时插入给定值, 然后返回 &mut V
    let v = m.entry("a").or_insert(10);
    *v += 1;
    let v = m.entry("a").or_insert(99);          // 已存在 → 99 不会被插入
    *v += 1;
    println!("  or_insert: {m:?}");

    // or_insert_with: 默认值由闭包生成 (懒求值, 可避免不必要的分配)
    let mut m: HashMap<&str, Vec<i32>> = HashMap::new();
    m.entry("group_a").or_insert_with(Vec::new).push(1);
    m.entry("group_a").or_insert_with(Vec::new).push(2);
    m.entry("group_b").or_insert_with(Vec::new).push(99);
    println!("  or_insert_with: {m:?}");

    // or_default: 等价 or_insert_with(V::default)
    let mut counts: HashMap<&str, i32> = HashMap::new();
    *counts.entry("hits").or_default() += 1;
    *counts.entry("hits").or_default() += 1;
    *counts.entry("miss").or_default() += 1;
    println!("  or_default: {counts:?}");
}

// ============================================================================
// 3. and_modify：存在时先改一下
// ============================================================================
//
// 链式表达"存在则 X, 不存在则 Y", 极优雅。
//
//   m.entry(k)
//    .and_modify(|v| { /* 已存在: v 是 &mut V */ })
//    .or_insert(default);

fn and_modify_demo() {
    let mut m: HashMap<&str, i32> = HashMap::from([("a", 10), ("b", 20)]);

    // 已存在: a += 1; 不存在: 插入 0
    m.entry("a").and_modify(|v| *v += 1).or_insert(0);
    m.entry("z").and_modify(|v| *v += 1).or_insert(0);
    println!("  {m:?}");

    // 真实例子: 更新最近访问时间, 不存在则记 0
    let mut last_visit: HashMap<&str, u64> = HashMap::new();
    let now = 1729000000;
    let visitors = ["alice", "bob", "alice", "carol", "alice"];
    for v in visitors {
        last_visit
            .entry(v)
            .and_modify(|t| *t = now)
            .or_insert(now);
    }
    println!("  last_visit: {last_visit:?}");
}

// ============================================================================
// 4. 三大经典模式
// ============================================================================

// 4.1 计数器 (counter)
fn counter() {
    let words = "the quick brown fox jumps over the lazy dog the fox";
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for w in words.split_whitespace() {
        *counts.entry(w).or_insert(0) += 1;
    }

    // 打印 by 排序后的频次
    let mut pairs: Vec<_> = counts.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    println!("  词频:");
    for (w, c) in pairs {
        println!("    {w} -> {c}");
    }
}

// 4.2 分组 (group_by)
fn group_by_first_letter() {
    let words = ["apple", "banana", "avocado", "blueberry", "carrot", "cherry"];
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();

    for w in words {
        let first = w.chars().next().unwrap();
        groups.entry(first).or_insert_with(Vec::new).push(w);
    }

    let mut keys: Vec<&char> = groups.keys().collect();
    keys.sort();
    println!("  分组结果:");
    for k in keys {
        println!("    {k} -> {:?}", groups[k]);
    }
}

// 4.3 缓存 / 备忘 (memoization)
fn memoized_fib() {
    fn fib_with_cache(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if n <= 1 { return n; }
        if let Some(&v) = cache.get(&n) {
            return v;
        }
        let v = fib_with_cache(n - 1, cache) + fib_with_cache(n - 2, cache);
        cache.insert(n, v);
        v
    }

    let mut cache = HashMap::new();
    for n in [10, 20, 30, 40, 50] {
        println!("  fib({n}) = {}", fib_with_cache(n, &mut cache));
    }
    println!("  缓存大小 = {}", cache.len());
}

// ============================================================================
// 5. 边角：在不存在时也只查不改
// ============================================================================
//
// 有时你只想"试试看", 不想造成插入。这种就别用 entry, 直接 .get():

fn read_only_lookup() {
    let m: HashMap<&str, i32> = HashMap::from([("a", 1), ("b", 2)]);

    // 想知道 'a' 是否存在并取值
    if let Some(&v) = m.get("a") {
        println!("  a 存在, 值 = {v}");
    }

    // 错误: 用 entry().or_insert(0) 会"无意中插入" 0
    // 想纯查询请用 .get() / .contains_key()
}

// ============================================================================
// 6. entry 的"返回 &mut V"性质：可以连续操作
// ============================================================================

fn chain_demo() {
    let mut m: HashMap<&str, Vec<u32>> = HashMap::new();

    // entry → or_insert_with 返回 &mut Vec<u32>, 可以连续 push:
    let bucket = m.entry("hits").or_insert_with(Vec::new);
    bucket.push(1);
    bucket.push(2);
    bucket.push(3);

    println!("  hits = {:?}", m.get("hits"));
}

fn main() {
    println!("===== 1. 等价写法对比 =====");
    let words = ["a", "b", "a", "c", "a", "b"];
    let m1 = count_naive(&words);
    let m2 = count_entry(&words);
    println!("  naive  -> {m1:?}");
    println!("  entry  -> {m2:?}");

    println!("\n===== 2. or_insert / or_insert_with / or_default =====");
    or_insert_demo();

    println!("\n===== 3. and_modify =====");
    and_modify_demo();

    println!("\n===== 4.1 计数器 =====");
    counter();

    println!("\n===== 4.2 按首字母分组 =====");
    group_by_first_letter();

    println!("\n===== 4.3 备忘式 fib =====");
    memoized_fib();

    println!("\n===== 5. 只查不改 =====");
    read_only_lookup();

    println!("\n===== 6. 连续操作返回的 &mut V =====");
    chain_demo();

    println!("\n===== 要点回顾 =====");
    println!("· entry 把'查 + 改 / 缺则插入' 合并成一次查询, 高效优雅");
    println!("· 计数器: *m.entry(k).or_insert(0) += 1");
    println!("· 分组:   m.entry(k).or_insert_with(Vec::new).push(x)");
    println!("· 已存在/不存在分支用: and_modify(...).or_insert(...)");
    println!("· 只读查询用 .get(), 别用 entry, 否则会插入默认值");
}
