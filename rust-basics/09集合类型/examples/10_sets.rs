//! 10. HashSet / BTreeSet：去重、集合运算
//!
//! 运行：cargo run --example 10_sets
//!
//! 本例覆盖：
//! - HashSet：哈希集合, 无序
//! - BTreeSet：有序集合
//! - 集合运算：union / intersection / difference / symmetric_difference / is_subset / is_disjoint
//! - 去重三种方式：HashSet / BTreeSet / Vec + dedup
//! - 集合算法实战：判定字谜、排列组合、集合论

#![allow(dead_code)]

use std::collections::{BTreeSet, HashSet};

// ============================================================================
// 1. HashSet vs BTreeSet
// ============================================================================
//
// HashSet<T> 本质就是 HashMap<T, ()>; BTreeSet<T> 就是 BTreeMap<T, ()>。
// 标准库给它们提供了一组"集合运算" API。
//
//   维度          HashSet           BTreeSet
//   ──────────   ────────────────  ──────────────────
//   元素要求       Eq + Hash         Ord
//   平均查找       O(1)              O(log n)
//   遍历顺序       随机              升序
//   范围查询       ❌                ✅ range(...)

fn create_demo() {
    // 空集合
    let mut s: HashSet<i32> = HashSet::new();
    s.insert(1);
    s.insert(2);
    s.insert(2);                                 // 重复, 实际上不会变
    s.insert(3);
    println!("  HashSet  = {s:?}, len={}", s.len());

    // 从迭代器
    let v = vec![1, 2, 2, 3, 4, 4, 5];
    let dedup: HashSet<i32> = v.into_iter().collect();
    let mut sorted: Vec<_> = dedup.iter().copied().collect();
    sorted.sort();
    println!("  Vec 去重   = {sorted:?}");

    // BTreeSet 直接给你有序输出
    let bts: BTreeSet<i32> = [3, 1, 4, 1, 5, 9, 2, 6].into_iter().collect();
    println!("  BTreeSet = {bts:?}  (天然有序)");

    // 字符串集合
    let langs: HashSet<&str> = ["Rust", "Go", "C++", "Rust"].into_iter().collect();
    println!("  langs    = {langs:?}");
}

// ============================================================================
// 2. 增删查改
// ============================================================================

fn crud_demo() {
    let mut s: HashSet<&str> = HashSet::new();

    // insert: 已存在返回 false
    println!("  insert 'a' => {}", s.insert("a"));
    println!("  insert 'a' => {} (重复)", s.insert("a"));
    println!("  insert 'b' => {}", s.insert("b"));

    // contains
    println!("  contains 'a' = {}", s.contains("a"));
    println!("  contains 'z' = {}", s.contains("z"));

    // remove
    println!("  remove 'a'   = {}", s.remove("a"));
    println!("  remove 'z'   = {}", s.remove("z"));

    println!("  最终 = {s:?}");
}

// ============================================================================
// 3. 集合运算
// ============================================================================
//
// HashSet / BTreeSet 都提供了集合运算 API:
//   a.union(&b)               并集 ∪
//   a.intersection(&b)        交集 ∩
//   a.difference(&b)          差集 a − b
//   a.symmetric_difference(&b) 对称差 (a ∪ b) − (a ∩ b)
//   a.is_subset(&b)           a ⊆ b
//   a.is_superset(&b)         a ⊇ b
//   a.is_disjoint(&b)         a ∩ b = ∅
//
// 注意: 它们返回**迭代器**, 不是 Set; 想要新 Set 要 collect.

fn ops_demo() {
    let a: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5, 6, 7].into_iter().collect();

    let sorted = |s: &HashSet<i32>| {
        let mut v: Vec<i32> = s.iter().copied().collect();
        v.sort();
        v
    };

    println!("  a = {:?}", sorted(&a));
    println!("  b = {:?}", sorted(&b));

    let union: Vec<i32> = a.union(&b).copied().collect();
    let mut union = union; union.sort();
    println!("  a ∪ b      = {union:?}");

    let inter: Vec<i32> = a.intersection(&b).copied().collect();
    let mut inter = inter; inter.sort();
    println!("  a ∩ b      = {inter:?}");

    let diff: Vec<i32> = a.difference(&b).copied().collect();
    let mut diff = diff; diff.sort();
    println!("  a − b      = {diff:?}");

    let sym: Vec<i32> = a.symmetric_difference(&b).copied().collect();
    let mut sym = sym; sym.sort();
    println!("  a △ b      = {sym:?}");

    println!("  a ⊆ b ?    = {}", a.is_subset(&b));
    println!("  a ⊇ {{1,2}} ?= {}", a.is_superset(&[1, 2].into_iter().collect()));
    let c: HashSet<i32> = [100, 200].into_iter().collect();
    println!("  a 与 c 互斥? = {}", a.is_disjoint(&c));
}

// ============================================================================
// 4. BTreeSet 也支持范围查询
// ============================================================================

fn btreeset_range() {
    let nums: BTreeSet<i32> = (1..=20).collect();

    // 范围
    let in_range: Vec<&i32> = nums.range(5..=10).collect();
    println!("  5..=10 = {in_range:?}");

    // 取最大 / 最小
    println!("  min = {:?}", nums.first());
    println!("  max = {:?}", nums.last());

    // 排序后取前 3 / 后 3 (因为本身有序, 直接 take / take(rev))
    let top3: Vec<&i32> = nums.iter().rev().take(3).collect();
    println!("  top 3 = {top3:?}");
}

// ============================================================================
// 5. 实战 1：判断两个字符串是不是字谜 (anagram)
// ============================================================================
//
// "字谜" = 重新排列字母后能相等 (例如 "listen" 和 "silent")。
// 集合无法判断 (重复字符就丢了); 应该用计数。
// 这里演示用 BTreeSet 判定"含字母集合是否相同"——是更宽松的版本。

fn loose_anagram(a: &str, b: &str) -> bool {
    let ca: BTreeSet<char> = a.chars().filter(|c| !c.is_whitespace()).collect();
    let cb: BTreeSet<char> = b.chars().filter(|c| !c.is_whitespace()).collect();
    ca == cb
}

// 严格 anagram 用计数
fn strict_anagram(a: &str, b: &str) -> bool {
    use std::collections::HashMap;
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in a.chars().filter(|c| !c.is_whitespace()) {
        *counts.entry(c).or_insert(0) += 1;
    }
    for c in b.chars().filter(|c| !c.is_whitespace()) {
        let v = counts.entry(c).or_insert(0);
        *v -= 1;
        if *v < 0 { return false; }
    }
    counts.values().all(|&v| v == 0)
}

// ============================================================================
// 6. 实战 2：从两组 ID 里找"被订阅但已注销"的用户
// ============================================================================

fn churn_demo() {
    let active: HashSet<u64> = [1, 2, 3, 5, 8, 13, 21, 34].into_iter().collect();
    let subscribers: HashSet<u64> = [1, 3, 4, 5, 7, 13, 100].into_iter().collect();

    let mut churn: Vec<u64> = subscribers.difference(&active).copied().collect();
    churn.sort();
    println!("  订阅但已注销 = {churn:?}");

    let mut both: Vec<u64> = subscribers.intersection(&active).copied().collect();
    both.sort();
    println!("  订阅且活跃   = {both:?}");
}

// ============================================================================
// 7. 三种"去重"方式对比
// ============================================================================

fn dedup_three_ways() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    // 方式 1: HashSet (最快, 但顺序丢了)
    let s: HashSet<i32> = v.iter().copied().collect();
    let mut a: Vec<i32> = s.into_iter().collect(); a.sort();
    println!("  HashSet 去重 + 排序 = {a:?}");

    // 方式 2: BTreeSet (天然有序)
    let s: BTreeSet<i32> = v.iter().copied().collect();
    println!("  BTreeSet 去重       = {s:?}");

    // 方式 3: Vec + dedup (保留首次出现的顺序)
    let mut seen = HashSet::new();
    let v3: Vec<i32> = v.iter().copied().filter(|x| seen.insert(*x)).collect();
    println!("  保留出现顺序去重     = {v3:?}");
}

fn main() {
    println!("===== 1. 创建 =====");
    create_demo();

    println!("\n===== 2. 增删查改 =====");
    crud_demo();

    println!("\n===== 3. 集合运算 =====");
    ops_demo();

    println!("\n===== 4. BTreeSet range =====");
    btreeset_range();

    println!("\n===== 5. 字谜判定 =====");
    for (a, b) in [("listen", "silent"), ("hello", "world"), ("rail safety", "fairy tales")] {
        println!("  '{a}' vs '{b}'");
        println!("    宽松 (字符集合)   = {}", loose_anagram(a, b));
        println!("    严格 (字符计数)   = {}", strict_anagram(a, b));
    }

    println!("\n===== 6. 用户流失分析 =====");
    churn_demo();

    println!("\n===== 7. 三种去重方式 =====");
    dedup_three_ways();

    println!("\n===== 要点回顾 =====");
    println!("· HashSet 无序、O(1); BTreeSet 有序、O(log n)");
    println!("· union/intersection/difference 返回'迭代器', 想要 Set 要 collect");
    println!("· 去重三选一: HashSet / BTreeSet / Vec + filter+HashSet");
    println!("· 严格相等比较 (字谜) 用 HashMap 计数, 别用集合");
}
