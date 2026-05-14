//! 03. Vec<T> 进阶：sort / dedup / retain / binary_search / chunks / windows / split
//!
//! 运行：cargo run --example 03_vec_advanced
//!
//! 本例覆盖：
//! - 排序家族：sort / sort_by / sort_by_key / sort_unstable_*
//! - 二分查找：binary_search / binary_search_by_key
//! - 就地变换：dedup / dedup_by_key / retain / fill / reverse
//! - 切分视图：split_at / chunks / windows / chunks_exact
//! - 拼接：concat / join
//! - 一些有用的"批量操作"

#![allow(dead_code)]
// 注：vec![] 是教学语法; while let 配合 chunks_exact 是为了演示 .remainder()
#![allow(clippy::useless_vec, clippy::while_let_on_iterator)]

// ============================================================================
// 1. 排序家族
// ============================================================================
//
//   sort()             ← 稳定排序, 默认升序, 要求 Ord
//   sort_by(|a,b| ...) ← 自定义比较函数
//   sort_by_key(|x| ..) ← 提供 key, 按 key 排序（要求 key: Ord）
//   sort_unstable*     ← 不稳定但更快; "稳定" 指相等元素相对顺序是否保持
//
// 排序都是**就地**修改, 不返回新 Vec.

fn sort_demo() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    v.sort();
    println!("  sort:        {v:?}");

    // 自定义比较：降序
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    v.sort_by(|a, b| b.cmp(a));
    println!("  desc by_by:  {v:?}");

    // 按 key 排序：按字符串长度
    let mut words = vec!["banana", "apple", "kiwi", "grape", "cherry"];
    words.sort_by_key(|s| s.len());
    println!("  by len:      {words:?}");

    // 按结构体的某个字段
    #[derive(Debug)]
    struct User { name: String, age: u32 }
    let mut users = vec![
        User { name: "alice".into(), age: 30 },
        User { name: "bob".into(),   age: 25 },
        User { name: "carol".into(), age: 28 },
    ];
    users.sort_by_key(|u| u.age);
    println!("  by age:");
    for u in &users {
        println!("    {u:?}");
    }

    // 多键排序：先按某字段, 相同则按另一字段
    let mut data = vec![(1, "b"), (2, "a"), (1, "a"), (2, "b")];
    data.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(b.1)));
    println!("  multi-key:   {data:?}");

    // sort_unstable: 比 sort 快, 但相等元素的相对顺序可能改变
    let mut v = vec![5, 1, 4, 2, 3];
    v.sort_unstable();
    println!("  unstable:    {v:?}");
}

// ============================================================================
// 2. 二分查找：binary_search 系列
// ============================================================================
//
// 前提：Vec 必须**已排序**, 否则结果未定义。
//
//   binary_search(&x)           → Result<usize, usize>
//                                  Ok(i)  : 找到了, 在索引 i
//                                  Err(i) : 没找到, 但 "如果要插入, i 是合适位置"
//
// 这个返回值非常巧妙——同时支持"查找"和"有序插入"两种用法。

fn bsearch_demo() {
    let v = vec![1, 3, 5, 7, 9, 11, 13];

    println!("  search 5  = {:?} (找到了, 索引 2)", v.binary_search(&5));
    println!("  search 6  = {:?} (没找到, 应插在索引 3)", v.binary_search(&6));
    println!("  search 0  = {:?}", v.binary_search(&0));
    println!("  search 14 = {:?}", v.binary_search(&14));

    // 维护一个**有序的 Vec**：用二分插入
    let mut sorted = vec![1, 3, 5, 7, 9];
    let to_insert = 6;
    match sorted.binary_search(&to_insert) {
        Ok(_) => { /* 已存在, 不重复插入 */ }
        Err(pos) => sorted.insert(pos, to_insert),
    }
    println!("  保序插入 6 -> {sorted:?}");
}

// ============================================================================
// 3. 就地变换：dedup / retain / fill / reverse
// ============================================================================

fn in_place_demo() {
    // dedup：连续相同元素去重（不全局去重！排过序之后再 dedup 才是完整去重）
    let mut v = vec![1, 1, 2, 3, 3, 3, 4, 1, 1];
    v.dedup();
    println!("  dedup（连续）   = {v:?}");      // [1, 2, 3, 4, 1]
    v.sort();
    v.dedup();
    println!("  sort+dedup (全局)= {v:?}");

    // dedup_by_key：按 key 去重
    let mut words = vec!["foo", "bar", "Bar", "baz", "BaZ"];
    words.dedup_by_key(|s| s.to_lowercase());
    println!("  dedup_by_key 大小写 = {words:?}");

    // retain：只保留满足条件的元素（in-place）
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    v.retain(|&x| x % 2 == 0);
    println!("  retain even     = {v:?}");

    // fill：所有元素填一样的值
    let mut buf = vec![0u8; 5];
    buf.fill(0xff);
    println!("  fill            = {buf:?}");

    // reverse
    let mut v = vec![1, 2, 3, 4, 5];
    v.reverse();
    println!("  reverse         = {v:?}");
}

// ============================================================================
// 4. 切分视图：split_at / chunks / windows / chunks_exact
// ============================================================================
//
// 这些都返回 **借用视图**, 不复制数据。
//
//   split_at(i)        → (&[T], &[T])    一刀切两段
//   chunks(n)          → 迭代器, 每次产出 &[T] 长度 <=n
//   chunks_exact(n)    → 同上, 但拒绝最后那不足 n 的余数
//   windows(n)         → 长度为 n 的滑动窗口

fn split_demo() {
    let v: Vec<i32> = (1..=10).collect();

    // split_at：一刀两段
    let (head, tail) = v.split_at(3);
    println!("  split_at(3): head={head:?}, tail={tail:?}");

    // chunks(3)：每 3 个一块
    println!("  chunks(3):");
    for c in v.chunks(3) {
        println!("    {c:?}");
    }

    // chunks_exact(3)：拒绝余数
    let v2: Vec<i32> = (1..=8).collect();
    let mut iter = v2.chunks_exact(3);
    while let Some(c) = iter.next() {
        println!("  chunks_exact(3): {c:?}");
    }
    println!("  剩余 (remainder): {:?}", iter.remainder());

    // windows(3)：滑动窗口
    println!("  windows(3):");
    for w in v.windows(3) {
        println!("    {w:?}");
    }

    // 一个常见技巧: 用 windows 求"连续 3 个之和最大"
    let v = vec![1, -3, 4, 5, -2, 8, 1];
    let max_sum = v.windows(3).map(|w| w.iter().sum::<i32>()).max();
    println!("  3 元滑窗最大和 = {max_sum:?}");
}

// ============================================================================
// 5. 切分元素：split / splitn / rsplit
// ============================================================================
//
// split 系列：根据"谓词"把 Vec 切成多段, 段之间被分隔元素隔开（分隔元素被丢弃）。

fn split_predicate_demo() {
    let v = vec![1, 2, 0, 3, 4, 0, 5, 0, 6];

    // 按值切分: 用谓词
    let groups: Vec<Vec<i32>> = v.split(|&x| x == 0).map(|s| s.to_vec()).collect();
    println!("  split by 0  = {groups:?}");

    // splitn: 限制最多切几段
    let groups: Vec<Vec<i32>> = v.splitn(2, |&x| x == 0).map(|s| s.to_vec()).collect();
    println!("  splitn(2)   = {groups:?}");
}

// ============================================================================
// 6. 拼接：concat / join / extend_from_slice
// ============================================================================

fn join_demo() {
    let v = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    let flat: Vec<i32> = v.concat();           // 直接拼接
    println!("  concat   = {flat:?}");

    let words = vec!["rust", "is", "fun"];
    let joined: String = words.join(" ");      // String 风格 join
    println!("  join \" \"  = {joined}");

    // 也可以用切片拼接
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let mut c: Vec<i32> = Vec::new();
    c.extend_from_slice(&a);
    c.extend_from_slice(&b);
    println!("  extend_from_slice = {c:?}");
}

// ============================================================================
// 7. 综合：把"无序数组"做完整的"统计分析"
// ============================================================================

fn analyse(numbers: &[i32]) {
    if numbers.is_empty() {
        println!("  (空数组)");
        return;
    }

    let len = numbers.len() as f64;
    let sum: i32 = numbers.iter().sum();
    let mean = sum as f64 / len;
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let mut sorted: Vec<i32> = numbers.to_vec();
    sorted.sort_unstable();
    let median = if sorted.len() % 2 == 1 {
        sorted[sorted.len() / 2] as f64
    } else {
        let a = sorted[sorted.len() / 2 - 1] as f64;
        let b = sorted[sorted.len() / 2] as f64;
        (a + b) / 2.0
    };

    let variance: f64 = numbers.iter().map(|&x| {
        let d = x as f64 - mean;
        d * d
    }).sum::<f64>() / len;
    let stddev = variance.sqrt();

    println!("  count = {}, min = {min}, max = {max}", numbers.len());
    println!("  sum = {sum}, mean = {mean:.2}");
    println!("  median = {median:.2}, stddev = {stddev:.2}");
}

fn main() {
    println!("===== 1. 排序家族 =====");
    sort_demo();

    println!("\n===== 2. 二分查找 =====");
    bsearch_demo();

    println!("\n===== 3. 就地变换 =====");
    in_place_demo();

    println!("\n===== 4. 切分视图 =====");
    split_demo();

    println!("\n===== 5. 谓词切分 =====");
    split_predicate_demo();

    println!("\n===== 6. 拼接 =====");
    join_demo();

    println!("\n===== 7. 综合：统计分析 =====");
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7];
    analyse(&numbers);

    println!("\n===== 要点回顾 =====");
    println!("· sort_by_key 是工程里最常用的排序写法");
    println!("· binary_search 的 Ok/Err 同时支持'查找'和'保序插入'");
    println!("· retain / dedup / fill / reverse 是就地变换四件套");
    println!("· chunks / windows 是处理'分块/滑动'的标准答案");
    println!("· concat / join 是拼接的标准操作");
}
