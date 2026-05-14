//! 09. BTreeMap：有序键值映射 + range 范围查询
//!
//! 运行：cargo run --example 09_btreemap
//!
//! 本例覆盖：
//! - BTreeMap 与 HashMap 的对比
//! - 创建、insert、get、remove
//! - 自带顺序：遍历总是按 key 升序
//! - range / range_mut：高效范围查询
//! - first_key_value / last_key_value
//! - K 必须实现 Ord（不是 Hash）

#![allow(dead_code)]
// 注：or_insert_with(Vec::new) 是教学清晰写法
#![allow(clippy::unwrap_or_default)]

use std::collections::{BTreeMap, HashMap};

// ============================================================================
// 1. HashMap vs BTreeMap：什么时候用谁
// ============================================================================
//
//   维度              HashMap                BTreeMap
//   ───────────────  ─────────────────────  ─────────────────────────
//   key 要求          Eq + Hash              Ord
//   平均查找          O(1)                   O(log n)
//   最坏情形           O(n)                   O(log n)
//   遍历顺序          随机（HashDoS 防御）    按 key 升序
//   范围查询          ❌ 不支持              ✅ range(...) 高效
//   首尾元素          需要遍历                first_key_value / last_key_value
//   内部实现          开放寻址 / 拉链        B-Tree
//   内存开销          中（buckets + slack）   低（按需分配）
//
// 经验法则:
//   - 默认 HashMap, 性能最好;
//   - 需要"顺序遍历 / 范围查询 / 取最值" 用 BTreeMap;
//   - K 是浮点数 → 只能 BTreeMap (不是 Hash);
//   - 容器要求"确定性"输出 (例如可重复构建) → BTreeMap.

// ============================================================================
// 2. 创建与基础操作
// ============================================================================

fn basic_demo() {
    // 多种构造方式
    let mut m: BTreeMap<&str, i32> = BTreeMap::new();
    m.insert("b", 2);
    m.insert("a", 1);
    m.insert("c", 3);

    let m2: BTreeMap<&str, i32> = BTreeMap::from([("y", 25), ("x", 24), ("z", 26)]);

    let pairs = vec![("alice", 30), ("bob", 25), ("carol", 28)];
    let m3: BTreeMap<&str, i32> = pairs.into_iter().collect();

    println!("  m  = {m:?}");
    println!("  m2 = {m2:?}");
    println!("  m3 = {m3:?}");
    println!("  ↑ 注意: 输出永远按 key 升序排列");

    // get / contains_key / remove 与 HashMap 完全一致
    println!("  m.get(\"b\")          = {:?}", m.get("b"));
    println!("  m.contains_key(\"x\") = {}", m.contains_key("x"));
    println!("  m.remove(\"a\")       = {:?}", m.remove("a"));
    println!("  m 现在 = {m:?}");
}

// ============================================================================
// 3. 自带顺序：遍历 = 升序
// ============================================================================

fn sorted_traversal() {
    let m: BTreeMap<i32, &str> = BTreeMap::from([
        (3, "three"),
        (1, "one"),
        (4, "four"),
        (1, "ONE"),     // 重复 key, 后者覆盖
        (5, "five"),
        (9, "nine"),
        (2, "two"),
        (6, "six"),
    ]);

    print!("  按 key 升序: ");
    for (k, v) in &m {
        print!("({k}, {v}) ");
    }
    println!();

    // 反向遍历 (从大到小)
    print!("  按 key 降序: ");
    for (k, v) in m.iter().rev() {
        print!("({k}, {v}) ");
    }
    println!();

    // keys / values 也都是有序的
    let keys: Vec<&i32> = m.keys().collect();
    println!("  keys = {keys:?}");
}

// ============================================================================
// 4. range：范围查询 (BTreeMap 的"杀手锏")
// ============================================================================
//
// `range` 接受任意可比较的范围 (Range / RangeInclusive / RangeFrom 等),
// 返回**有序**的迭代器, 内部时间复杂度 O(log n + k), k 是返回的元素数。

fn range_demo() {
    let mut scores = BTreeMap::new();
    for (k, v) in [("alex", 85), ("bob", 72), ("carol", 91), ("david", 60),
                   ("eve", 78), ("frank", 88), ("grace", 95)] {
        scores.insert(k, v);
    }

    println!("  全部:");
    for (k, v) in &scores {
        println!("    {k} = {v}");
    }

    // 字符串范围查询: 取 c..=f
    println!("\n  range(\"c\"..=\"f\"):");
    for (k, v) in scores.range("c"..="f") {
        println!("    {k} = {v}");
    }

    // 数值范围: 取 70..=85 的"分数"无法直接, 因为是 V 不是 K
    // 但反过来——用 BTreeMap<u32, &str> 就能按"分数"做范围查询:
    let mut by_score: BTreeMap<u32, Vec<&str>> = BTreeMap::new();
    for (name, score) in &scores {
        by_score.entry(*score).or_insert_with(Vec::new).push(*name);
    }
    println!("\n  by_score (key=分数):");
    for (s, names) in &by_score {
        println!("    {s} -> {names:?}");
    }
    println!("\n  分数 70..85:");
    for (s, names) in by_score.range(70..85) {
        println!("    {s} -> {names:?}");
    }

    // range_mut: 在范围内修改
    let mut m: BTreeMap<i32, i32> = (1..=10).map(|x| (x, x)).collect();
    for (_, v) in m.range_mut(3..=7) {
        *v *= 100;
    }
    println!("\n  range_mut(3..=7) *= 100 后: {m:?}");
}

// ============================================================================
// 5. first_key_value / last_key_value：拿首尾不用遍历
// ============================================================================

fn first_last_demo() {
    let m: BTreeMap<&str, i32> = BTreeMap::from([
        ("apple", 1),
        ("banana", 2),
        ("cherry", 3),
    ]);

    println!("  first = {:?}", m.first_key_value());
    println!("  last  = {:?}", m.last_key_value());

    // 用作"优先队列"的两端: pop_first / pop_last
    let mut m: BTreeMap<i32, &str> = BTreeMap::from([
        (3, "three"), (1, "one"), (4, "four"), (1, "ONE"), (5, "five"),
    ]);
    while let Some((k, v)) = m.pop_first() {
        println!("  pop_first -> ({k}, {v})");
    }
}

// ============================================================================
// 6. K 是浮点数：HashMap 不行, BTreeMap 凑合
// ============================================================================
//
// `f64` 实现了 PartialOrd 但不是 Ord (因为 NaN), 所以**不能**直接当 BTreeMap 的 key。
// 实战常用替代:
//   - 把 f64 量化成 u64 (例如 (x * 1000.0).round() as u64)
//   - 用 OrderedFloat / NotNan 这类第三方类型

fn float_key_workaround() {
    // 量化方案：把 f64 价格量化到 u64 (单位: 千分之一)
    let mut m: BTreeMap<u64, &str> = BTreeMap::new();

    let entries: [(f64, &str); 3] = [(1.5, "low"), (4.25, "mid"), (9.99, "high")];
    for (price, label) in entries {
        let key = (price * 1000.0).round() as u64;
        m.insert(key, label);
    }
    for (k, v) in &m {
        println!("  key={k} (= {} 元) -> {v}", *k as f64 / 1000.0);
    }
}

// ============================================================================
// 7. entry API：BTreeMap 也有
// ============================================================================

fn entry_demo() {
    let mut counts: BTreeMap<char, u32> = BTreeMap::new();
    for c in "the quick brown fox".chars() {
        if c.is_alphabetic() {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    println!("  字母频次 (按 key 升序输出):");
    for (c, n) in &counts {
        println!("    {c} -> {n}");
    }
}

// ============================================================================
// 8. 性能直觉：HashMap vs BTreeMap 简单对比
// ============================================================================

fn performance_hint() {
    use std::time::Instant;
    let n = 100_000;

    // HashMap
    let mut hm: HashMap<i32, i32> = HashMap::with_capacity(n);
    let t = Instant::now();
    for i in 0..n as i32 { hm.insert(i, i); }
    let t1 = t.elapsed();

    // BTreeMap
    let mut bm: BTreeMap<i32, i32> = BTreeMap::new();
    let t = Instant::now();
    for i in 0..n as i32 { bm.insert(i, i); }
    let t2 = t.elapsed();

    println!("  插入 {n} 个元素:");
    println!("    HashMap   = {t1:?}");
    println!("    BTreeMap  = {t2:?}");

    // 查询
    let t = Instant::now();
    let mut sum = 0i64;
    for i in 0..n as i32 { sum += hm.get(&i).copied().unwrap_or(0) as i64; }
    let q1 = t.elapsed();
    let t = Instant::now();
    let mut sum2 = 0i64;
    for i in 0..n as i32 { sum2 += bm.get(&i).copied().unwrap_or(0) as i64; }
    let q2 = t.elapsed();

    println!("  查询 {n} 次:");
    println!("    HashMap   = {q1:?} (sum={sum})");
    println!("    BTreeMap  = {q2:?} (sum={sum2})");
    println!("  → 查询速度通常 HashMap 更快; 但 BTreeMap 自带有序");
}

fn main() {
    println!("===== 1. HashMap vs BTreeMap =====");
    println!("  详见文件顶部对照表");

    println!("\n===== 2. 基础操作 =====");
    basic_demo();

    println!("\n===== 3. 有序遍历 =====");
    sorted_traversal();

    println!("\n===== 4. range 范围查询 =====");
    range_demo();

    println!("\n===== 5. 首尾操作 =====");
    first_last_demo();

    println!("\n===== 6. 浮点 key 解决方案 =====");
    float_key_workaround();

    println!("\n===== 7. entry API =====");
    entry_demo();

    println!("\n===== 8. 简单性能对比 =====");
    performance_hint();

    println!("\n===== 要点回顾 =====");
    println!("· BTreeMap 自带顺序; HashMap 速度更快但无序");
    println!("· 需要范围查询 / 取最值 / 确定性遍历 → BTreeMap");
    println!("· K 是浮点 → BTreeMap 也不行, 量化 / OrderedFloat");
    println!("· entry / range 是工程里最常用的两组 API");
}
