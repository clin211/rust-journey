//! 02. Vec<T> 的迭代器世界：map / filter / fold / zip / enumerate / collect
//!
//! 运行：cargo run --example 02_vec_iter
//!
//! 本例覆盖：
//! - 三种迭代器：iter / iter_mut / into_iter（再深入一层）
//! - 适配器（Adapter）：map / filter / take / skip / step_by / chain / zip / enumerate / rev
//! - 终结子（Consumer）：collect / count / sum / product / fold / reduce / any / all
//! - turbofish `::<>`：当 collect 的目标类型推断不出来时
//! - 链式管道：写"数据加工流水线"
//!
//! 迭代器是 Rust 写"数据处理"最自然的方式 —— 后面的章节（迭代器与闭包）会单独深入，
//! 这里先掌握配合 Vec 时最高频的用法。

#![allow(dead_code)]
// 注：本章演示迭代器 + Vec, vec![] / .iter().count() / fold / iter().copied().collect()
//     都是要展示的 API 教学素材, 故保留对应 lint
#![allow(
    clippy::useless_vec,
    clippy::iter_count,
    clippy::unnecessary_fold,
    clippy::iter_cloned_collect,
)]

// ============================================================================
// 1. iter / iter_mut / into_iter 三兄弟
// ============================================================================
//
//   方法            产生类型     消耗 Vec   适用
//   ───────────    ──────────   ─────────  ──────────────────────────
//   iter()          Item = &T    否         只读遍历, 90% 场景
//   iter_mut()      Item = &mut T 否        原地修改
//   into_iter()     Item = T     是         转移所有权 / 收集到别的容器

fn three_iters() {
    let v = vec![1, 2, 3];

    // iter()：只读
    let sum: i32 = v.iter().sum();           // 1+2+3 = 6
    println!("  iter sum = {sum}, v 仍可用 = {v:?}");

    // iter_mut()：可变
    let mut v2 = vec![1, 2, 3];
    v2.iter_mut().for_each(|x| *x += 10);
    println!("  iter_mut +10 -> {v2:?}");

    // into_iter()：消耗
    let v3 = vec!["a".to_string(), "b".to_string()];
    let collected: Vec<String> = v3.into_iter().map(|s| s + "!").collect();
    println!("  into_iter map -> {collected:?}");
    // println!("{v3:?}");   // ❌ v3 已经被消耗
}

// ============================================================================
// 2. 适配器（Adapter）：把迭代器变成新的迭代器
// ============================================================================
//
// 适配器是惰性的（lazy）—— 它本身不做计算，只是包装一层"未来要做的变换"。
// 真正"动手"是在终结子（collect / sum / count 等）那一刻。

fn adapters() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map: 把每个元素映射成新值
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("  doubled  = {doubled:?}");

    // filter: 只保留满足条件的
    let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  evens    = {evens:?}");

    // filter_map: 同时做 filter + map（返回 None 即丢弃）
    let big_doubled: Vec<i32> = v
        .iter()
        .filter_map(|&x| if x > 5 { Some(x * 2) } else { None })
        .collect();
    println!("  big_x2   = {big_doubled:?}");

    // take / skip：取前 N / 跳过前 N
    let first3: Vec<&i32> = v.iter().take(3).collect();
    let after3: Vec<&i32> = v.iter().skip(3).collect();
    println!("  take(3)  = {first3:?}");
    println!("  skip(3)  = {after3:?}");

    // step_by：按步长取
    let step2: Vec<&i32> = v.iter().step_by(2).collect();
    println!("  step_by(2) = {step2:?}");

    // chain: 接两个迭代器
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let merged: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    println!("  chain    = {merged:?}");

    // zip: 拉链合并
    let names = vec!["alice", "bob", "carol"];
    let ages = vec![30, 25, 28];
    let combined: Vec<(&&str, &i32)> = names.iter().zip(ages.iter()).collect();
    println!("  zip      = {combined:?}");

    // enumerate: 给元素加上下标
    print!("  enumerate: ");
    for (i, x) in v.iter().enumerate().take(5) {
        print!("({i}, {x}) ");
    }
    println!();

    // rev: 反向
    let reversed: Vec<&i32> = v.iter().rev().take(3).collect();
    println!("  rev top3 = {reversed:?}");
}

// ============================================================================
// 3. 终结子（Consumer）：让"流水线"产生最终结果
// ============================================================================

fn consumers() {
    let v = vec![1, 2, 3, 4, 5];

    // count / sum / product
    println!("  count    = {}", v.iter().count());
    println!("  sum      = {}", v.iter().sum::<i32>());
    println!("  product  = {}", v.iter().product::<i32>());

    // min / max / max_by / min_by_key
    println!("  min      = {:?}", v.iter().min());
    println!("  max      = {:?}", v.iter().max());

    let by_abs: Vec<i32> = vec![-5, 3, -2, 4];
    println!("  max_by_key abs = {:?}", by_abs.iter().max_by_key(|x| x.abs()));

    // fold: 通用的"折叠/累积"
    let sum_via_fold: i32 = v.iter().fold(0, |acc, x| acc + x);
    println!("  fold sum = {sum_via_fold}");

    let csv: String = v.iter().fold(String::new(), |acc, x| {
        if acc.is_empty() { x.to_string() } else { format!("{acc},{x}") }
    });
    println!("  fold csv = {csv}");

    // reduce: 用第一个元素当种子的 fold
    let max_via_reduce = v.iter().copied().reduce(i32::max);
    println!("  reduce max = {max_via_reduce:?}");

    // any / all
    println!("  any > 3  = {}", v.iter().any(|&x| x > 3));
    println!("  all > 0  = {}", v.iter().all(|&x| x > 0));

    // find / position
    println!("  find  >3 = {:?}", v.iter().find(|&&x| x > 3));
    println!("  position > 3 = {:?}", v.iter().position(|&x| x > 3));
}

// ============================================================================
// 4. collect：把迭代器收成"任意能容纳"的集合
// ============================================================================
//
// collect 的强大之处：目标类型由你指定 —— Vec / String / HashMap / HashSet ...
// 都能从迭代器里"长出来"。
//
//   语法可选写法：
//     let v: Vec<i32> = it.collect();          ← 类型注解
//     let v = it.collect::<Vec<i32>>();        ← turbofish
//
// 当目标类型推断不出来时, 一定要显式给出类型。

fn collect_demo() {
    use std::collections::{HashMap, HashSet};

    let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    // 收成 Vec
    let into_vec: Vec<i32> = v.iter().copied().collect();
    println!("  Vec  = {into_vec:?}");

    // 收成 HashSet（自动去重）
    let into_set: HashSet<i32> = v.iter().copied().collect();
    let mut sorted: Vec<i32> = into_set.into_iter().collect();
    sorted.sort();
    println!("  Set  = {sorted:?} (去重后)");

    // 收成 HashMap：迭代器要产 (K, V) 元组
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let into_map: HashMap<&str, i32> = pairs.into_iter().collect();
    let mut keys: Vec<&&str> = into_map.keys().collect();
    keys.sort();
    println!("  Map keys (sorted) = {keys:?}");

    // 收成 String：直接 join
    let words = vec!["hello", "rust", "world"];
    let joined: String = words.join(" ");          // join 比 fold 自然
    println!("  join = {joined}");

    // turbofish 写法
    let n = (1..=5).collect::<Vec<_>>();
    println!("  turbofish = {n:?}");
}

// ============================================================================
// 5. 一个真实小流水线：从一段文本里挑出"长度 >= 4 的非数字单词"
// ============================================================================

fn pipeline_demo() {
    let text = "The 42 quick brown fox jumps over 7 lazy dogs and 1 cat";

    let words: Vec<String> = text
        .split_whitespace()                       // 切词
        .filter(|w| !w.chars().all(|c| c.is_ascii_digit()))  // 不是纯数字
        .filter(|w| w.len() >= 4)                 // 长度 >= 4
        .map(|w| w.to_lowercase())                // 小写化
        .collect();

    println!("  pipeline -> {words:?}");

    // 链式聚合
    let total_len: usize = words.iter().map(|w| w.len()).sum();
    println!("  字符总长 = {total_len}");

    // 统计平均长度
    if !words.is_empty() {
        let avg = total_len as f64 / words.len() as f64;
        println!("  平均长度 = {avg:.2}");
    }
}

// ============================================================================
// 6. 惰性求值的小坑
// ============================================================================
//
// 适配器是惰性的：如果你忘了"终结子", 整条管道压根不会执行!

fn lazy_pitfall() {
    let v = vec![1, 2, 3];
    let _ = v.iter().map(|x| {
        println!("  [side-effect map: {x}]");
        x * 2
    });
    // ↑ 没有 collect / for_each, 上面的 println! 一次都不会触发

    println!("  --- 加上 .for_each(...) 才会真正跑 ---");
    v.iter().for_each(|x| println!("  for_each x={x}"));
}

fn main() {
    println!("===== 1. iter / iter_mut / into_iter =====");
    three_iters();

    println!("\n===== 2. 适配器 =====");
    adapters();

    println!("\n===== 3. 终结子 =====");
    consumers();

    println!("\n===== 4. collect 收成各种集合 =====");
    collect_demo();

    println!("\n===== 5. 真实流水线 =====");
    pipeline_demo();

    println!("\n===== 6. 惰性求值的坑 =====");
    lazy_pitfall();

    println!("\n===== 要点回顾 =====");
    println!("· iter / iter_mut / into_iter 与 & / &mut / move 一一对应");
    println!("· 适配器是惰性的, 必须配合终结子才会真正执行");
    println!("· collect 能把迭代器收成 Vec / String / HashMap / HashSet ...");
    println!("· 写'链式管道'是 Rust 处理数据最自然的方式");
}
