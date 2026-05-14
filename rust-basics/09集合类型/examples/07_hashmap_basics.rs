//! 07. HashMap<K, V> 基础：创建、增删查改、遍历、所有权
//!
//! 运行：cargo run --example 07_hashmap_basics
//!
//! 本例覆盖：
//! - 创建、insert、get、remove、contains_key
//! - 所有权语义（key/value 移动 vs 借用）
//! - 遍历：iter / iter_mut / into_iter / keys / values
//! - K 必须实现 Eq + Hash
//! - HashMap 的随机顺序与 HashDoS 防御
//! - HashMap vs BTreeMap 选型预告

#![allow(dead_code)]
// 注：vec![] 用于演示;
//     into_iter().map(|(_, v)| v) 故意演示"如何从 (K,V) 元组中取 V", 而非用 into_values()
#![allow(clippy::useless_vec, clippy::iter_kv_map)]

use std::collections::HashMap;

// ============================================================================
// 1. 创建 HashMap
// ============================================================================
//
//   HashMap::new()                      空 HashMap, 类型靠后续推断
//   HashMap::with_capacity(n)           预分配
//   HashMap::from([(k, v), (k, v), ..]) 从数组直接造 (Rust 1.56+)
//   collect 从 (K, V) 元组迭代器收集

fn create_demo() {
    // 1) 显式 new + 推断
    let mut a: HashMap<String, i32> = HashMap::new();
    a.insert("alice".into(), 95);
    a.insert("bob".into(), 87);
    println!("  a = {a:?}");

    // 2) HashMap::from
    let b = HashMap::from([
        ("apple", 3),
        ("banana", 2),
        ("cherry", 5),
    ]);
    println!("  b = {b:?}");

    // 3) collect
    let pairs = vec![("x", 1), ("y", 2), ("z", 3)];
    let c: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("  c = {c:?}");

    // 4) 把两个 Vec 通过 zip 拉链构造 HashMap
    let names = vec!["alice", "bob", "carol"];
    let scores = vec![95, 87, 78];
    let map: HashMap<&&str, &i32> = names.iter().zip(scores.iter()).collect();
    println!("  zip -> {map:?}");
}

// ============================================================================
// 2. insert / get / remove / contains_key
// ============================================================================
//
//   insert(k, v) → Option<V>           覆盖已有的, 返回旧值
//   get(&k)      → Option<&V>          安全取值
//   get_mut(&k)  → Option<&mut V>      可变借用
//   remove(&k)   → Option<V>           取出并删除
//   contains_key(&k) → bool

fn crud_demo() {
    let mut m: HashMap<String, i32> = HashMap::new();

    // insert 第一次返回 None
    let old = m.insert("a".into(), 1);
    println!("  第一次 insert('a',1) 返回 {old:?}");

    // 覆盖已有值: 返回旧值
    let old = m.insert("a".into(), 100);
    println!("  覆盖 'a' -> 100, 旧值 = {old:?}");

    m.insert("b".into(), 2);
    m.insert("c".into(), 3);

    // get
    println!("  get('a') = {:?}", m.get("a"));
    println!("  get('z') = {:?}", m.get("z"));

    // get_mut: 可变借用, 在不知道是否存在时配合 if let
    if let Some(v) = m.get_mut("b") {
        *v *= 10;
    }
    println!("  b 翻 10 倍后 = {:?}", m.get("b"));

    // remove: 移除 + 取出
    let removed = m.remove("c");
    println!("  remove('c') = {removed:?}");

    // contains_key
    println!("  contains 'a'? {}", m.contains_key("a"));
    println!("  contains 'c'? {}", m.contains_key("c"));

    println!("  最终: {m:?}, len={}", m.len());
}

// ============================================================================
// 3. 所有权：key 和 value 都被 move 进 HashMap
// ============================================================================

fn ownership_demo() {
    // String key: 被 move 进去了
    let key = String::from("name");
    let value = String::from("alice");

    let mut m: HashMap<String, String> = HashMap::new();
    m.insert(key, value);                    // key, value 都被 move
    // println!("{key}");                    // ❌ borrow of moved value

    println!("  插入后 m = {m:?}");

    // 取值时返回的是 &V, 不取走所有权
    let v: Option<&String> = m.get("name");
    println!("  get('name') = {v:?}");

    // 想直接拿走 V? 用 remove
    let v: Option<String> = m.remove("name");
    println!("  remove('name') = {v:?}");
    println!("  m 现在: {m:?}");
}

// ============================================================================
// 4. 遍历：keys / values / iter / iter_mut / into_iter
// ============================================================================
//
// HashMap 的遍历**顺序是不确定的**（每次运行可能不同）。
// 这是为了防御 HashDoS 攻击（旧版语言里, 攻击者可以构造大量哈希冲突让程序变慢）。
// Rust 默认随机化 hasher seed, 让顺序不可预测。

fn iter_demo() {
    let mut m = HashMap::from([
        ("alice", 95),
        ("bob", 87),
        ("carol", 78),
    ]);

    // 只看 key
    print!("  keys:    ");
    for k in m.keys() { print!("{k} "); }
    println!();

    // 只看 value
    print!("  values:  ");
    for v in m.values() { print!("{v} "); }
    println!();

    // 都要 (借用)
    print!("  iter:    ");
    for (k, v) in &m {
        print!("({k}={v}) ");
    }
    println!();

    // 可变 value
    for v in m.values_mut() { *v += 1; }
    println!("  values+1 后: {m:?}");

    // 消耗式: 把 (K, V) 都拿走
    let total: i32 = m.into_iter().map(|(_, v)| v).sum();
    println!("  into_iter 求和 = {total}");
    // m 不能再用
}

// ============================================================================
// 5. 想要稳定顺序? 排序后再遍历
// ============================================================================

fn sorted_iter() {
    let m = HashMap::from([
        ("apple", 3),
        ("banana", 2),
        ("cherry", 5),
    ]);

    // 把 (K, V) 收集到 Vec, 排序
    let mut pairs: Vec<(&&str, &i32)> = m.iter().collect();
    pairs.sort_by_key(|&(k, _)| *k);
    println!("  按 key 排序:");
    for (k, v) in pairs {
        println!("    {k} = {v}");
    }

    let mut pairs: Vec<(&&str, &i32)> = m.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));   // 按 value 降序
    println!("  按 value 降序:");
    for (k, v) in pairs {
        println!("    {k} = {v}");
    }

    // 想要默认就有序? 用 BTreeMap (见 09_btreemap.rs)
}

// ============================================================================
// 6. K 必须实现 Eq + Hash
// ============================================================================
//
// 标准库提供了一组开箱即用的 K：
//   - 整数、bool、char
//   - String / &str
//   - Tuple / Vec / 数组（要求元素也是 Eq + Hash）
//   - 自定义 struct/enum（用 #[derive(PartialEq, Eq, Hash)]）
//
// **不能直接用作 key 的**:
//   - f32 / f64        (NaN != NaN)
//   - 含 f32/f64 的复合类型 (如 Vec<f64>)

fn custom_key_demo() {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct UserId {
        region: String,
        id: u64,
    }

    let mut scores: HashMap<UserId, i32> = HashMap::new();
    scores.insert(UserId { region: "us-west-1".into(), id: 1 }, 95);
    scores.insert(UserId { region: "eu-east-1".into(), id: 1 }, 88);

    let key = UserId { region: "us-west-1".into(), id: 1 };
    println!("  scores[key] = {:?}", scores.get(&key));
}

// ============================================================================
// 7. 一些好用的小技巧
// ============================================================================

fn tricks() {
    let m = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    // 求 value 之和
    let total: i32 = m.values().sum();
    println!("  values().sum() = {total}");

    // 求 max value 对应的 key
    let max = m.iter().max_by_key(|&(_, v)| *v);
    println!("  max value 的 key = {max:?}");

    // 把 HashMap 变 Vec<(K, V)>
    let mut pairs: Vec<(&str, i32)> = m.iter().map(|(k, v)| (*k, *v)).collect();
    pairs.sort();
    println!("  pairs sorted = {pairs:?}");

    // 把 HashMap 反转 (V → K)
    let inverted: HashMap<i32, &str> = m.iter().map(|(k, v)| (*v, *k)).collect();
    println!("  inverted = {inverted:?}");
}

fn main() {
    println!("===== 1. 创建 =====");
    create_demo();

    println!("\n===== 2. 增删查改 =====");
    crud_demo();

    println!("\n===== 3. 所有权语义 =====");
    ownership_demo();

    println!("\n===== 4. 遍历（顺序不确定）=====");
    iter_demo();

    println!("\n===== 5. 排序后遍历 =====");
    sorted_iter();

    println!("\n===== 6. 自定义 key =====");
    custom_key_demo();

    println!("\n===== 7. 实用小技巧 =====");
    tricks();

    println!("\n===== 要点回顾 =====");
    println!("· HashMap 是无序的, 默认 SipHash 防 HashDoS");
    println!("· insert 返回旧值; get / remove 返回 Option");
    println!("· K 要 Eq + Hash; f32/f64 不能直接当 key");
    println!("· 想要默认有序 → 用 BTreeMap (下一例)");
    println!("· entry API 是工程里最高频的写法 → 见 08_hashmap_entry.rs");
}
