//! 01. Vec<T> 基础：创建、增删、访问、长度与容量
//!
//! 运行：cargo run --example 01_vec_basics
//!
//! 本例覆盖：
//! - 三种创建方式：Vec::new / vec! / Vec::with_capacity
//! - push / pop / insert / remove / swap_remove / clear / truncate
//! - 索引 [i] vs 安全访问 .get(i)
//! - len / is_empty / capacity / shrink_to_fit
//! - Vec 的"所有权 / 借用"语义
//! - 内存布局：栈 24B 胖指针 + 堆数据

#![allow(dead_code)]
// 注：本章主题就是 Vec, vec![] 是教学的核心语法之一, 故保留 useless_vec lint
// v.get(0) 是和 v.get(10) 配对演示"按索引安全访问"的, 故保留 get_first lint
#![allow(clippy::useless_vec, clippy::get_first)]

// ============================================================================
// 1. 创建 Vec：三种典型方式
// ============================================================================
//
// Rust 没有"内置数组"的可变扩张能力——`[T; N]` 长度固定在编译期。
// 真正的"动态数组" = `Vec<T>`，住在标准库里。
//
//   方式            语法                          适用
//   ────────────    ───────────────────────────  ──────────────────────────
//   空 Vec          Vec::<T>::new()               已知元素类型，先构造再 push
//   字面量          vec![1, 2, 3]                 立刻有几个固定元素
//   预分配          Vec::with_capacity(1024)      已知大约要塞多少，避免反复 realloc

fn create_demo() {
    let a: Vec<i32> = Vec::new();              // 类型必须靠注解或后续推断给出
    let b = vec![1, 2, 3, 4, 5];               // 自动推断为 Vec<i32>
    let c: Vec<String> = Vec::with_capacity(8); // 预分配容量

    println!("  a = {a:?}, len={}, cap={}", a.len(), a.capacity());
    println!("  b = {b:?}, len={}, cap={}", b.len(), b.capacity());
    println!("  c = {c:?}, len={}, cap={}", c.len(), c.capacity());

    // vec![value; count]：构造一个全是相同值的 Vec
    let zeros = vec![0; 5];
    println!("  vec![0; 5] = {zeros:?}");

    // 从迭代器收集：极常见
    let evens: Vec<i32> = (0..10).filter(|n| n % 2 == 0).collect();
    println!("  collect 偶数 = {evens:?}");
}

// ============================================================================
// 2. 增加元素：push / insert / extend
// ============================================================================
//
// 一旦你想对 Vec 做"修改"，原变量必须 `mut`：
//
//   let mut v = ...;            ← 没有 mut 就连 push 都不能调

fn add_demo() {
    let mut v: Vec<i32> = Vec::new();

    // push: O(1) 摊还，最常用
    v.push(10);
    v.push(20);
    v.push(30);
    println!("  push 三次后 = {v:?}");

    // insert(idx, value): 在 idx 插入，后面元素整体后移 → O(n)
    v.insert(1, 99);
    println!("  insert(1, 99) 后 = {v:?}");      // [10, 99, 20, 30]

    // extend：把另一个迭代器的所有元素塞进来
    v.extend([100, 200, 300]);
    v.extend(40..43);                            // 0..N 也是 IntoIterator
    println!("  extend 完后 = {v:?}");

    // 注意：extend 不消费 v 自己, 等价于反复 push
}

// ============================================================================
// 3. 删除元素：pop / remove / swap_remove / clear / truncate / drain
// ============================================================================
//
//   方法              复杂度    保持顺序   返回值
//   ──────────────   ────────  ─────────  ────────────
//   pop()             O(1)     是          Option<T>  (空时 None)
//   remove(i)         O(n)     是          T         (越界 panic)
//   swap_remove(i)    O(1)     否          T         (用末尾元素填空, 越界 panic)
//   clear()           O(n)     —           ()        (drop 全部 + len=0, cap 不变)
//   truncate(k)       O(n-k)   是          ()        (只保留前 k 个)
//   drain(range)      O(n)     是          迭代器     (按范围拿走, 同时清空那段)

fn remove_demo() {
    let mut v = vec![10, 20, 30, 40, 50];
    println!("  原始: {v:?}");

    let last = v.pop();                        // Some(50), 末尾被取走
    println!("  pop()        => {last:?},  v={v:?}");

    let removed = v.remove(0);                 // 拿走第 0 个, 后面所有元素左移
    println!("  remove(0)    => {removed},  v={v:?}");

    let swapped = v.swap_remove(0);            // 用末尾填回当前位置 → 顺序乱
    println!("  swap_remove  => {swapped},  v={v:?}");

    let mut v2 = vec![1, 2, 3, 4, 5];
    let drained: Vec<i32> = v2.drain(1..4).collect();
    println!("  drain(1..4)  => {drained:?},  v2={v2:?}");

    let mut v3 = vec![1, 2, 3, 4, 5];
    v3.truncate(2);
    println!("  truncate(2)  => v3={v3:?}");

    v3.clear();
    println!("  clear()      => v3={v3:?}, len={}, cap={}", v3.len(), v3.capacity());
}

// ============================================================================
// 4. 访问元素：v[i] vs v.get(i)
// ============================================================================
//
//   v[i]      ← 越界 panic; 简洁但危险
//   v.get(i)  ← 返回 Option<&T>; 安全但需 match / unwrap_or

fn access_demo() {
    let v = vec![10, 20, 30];

    // 索引：越界直接 panic, 类似数组下标
    let first = v[0];
    println!("  v[0] = {first}");
    // let oops = v[10];   // ❌ thread 'main' panicked at 'index out of bounds'

    // 安全访问 get
    println!("  v.get(0)  = {:?}", v.get(0));
    println!("  v.get(10) = {:?}", v.get(10));   // None

    // first / last 直接给 Option<&T>
    println!("  v.first() = {:?}", v.first());
    println!("  v.last()  = {:?}", v.last());

    // 修改要 get_mut / 借用
    let mut v2 = vec![1, 2, 3];
    if let Some(first) = v2.get_mut(0) {
        *first = 100;
    }
    println!("  get_mut(0) 后 v2 = {v2:?}");
}

// ============================================================================
// 5. 遍历：iter / iter_mut / into_iter
// ============================================================================
//
//   iter()        → 产生 &T，不消耗 Vec
//   iter_mut()    → 产生 &mut T，原地修改
//   into_iter()   → 产生 T，消耗 Vec（之后不能再用）
//
// for 循环里写 `for x in &v`、`for x in &mut v`、`for x in v` 实际上分别对应这三种。

fn iter_demo() {
    let v = vec![1, 2, 3, 4, 5];

    // 不可变遍历
    print!("  iter:     ");
    for x in &v { print!("{x} "); }
    println!();

    // 可变遍历
    let mut v2 = vec![1, 2, 3, 4, 5];
    for x in &mut v2 { *x *= 10; }
    println!("  iter_mut: {v2:?}");

    // 消耗式遍历: 之后 v3 就不能用了
    let v3 = vec!["a".to_string(), "b".to_string()];
    for s in v3 {                                // for s in v3 = into_iter
        println!("  into_iter: 拿走了 {s}");
    }
    // println!("{v3:?}");                       // ❌ borrow of moved value
}

// ============================================================================
// 6. 长度 vs 容量
// ============================================================================
//
//   len      ← Vec 当前持有的元素个数
//   capacity ← 已分配的存储槽位（>=len）
//
// 当 len 即将超过 cap 时, Vec 会触发"扩容":
//   - 分配新的更大的堆空间 (一般 2x)
//   - 把旧元素 memcpy 过去
//   - 释放旧空间
//
// 这个开销是 O(n)，所以"已知规模"时应当 with_capacity 预分配。

fn capacity_demo() {
    let mut v: Vec<i32> = Vec::new();
    println!("  初始: len={}, cap={}", v.len(), v.capacity());

    for i in 1..=10 {
        v.push(i);
        println!("  push({i}) 后: len={}, cap={}", v.len(), v.capacity());
    }

    // shrink_to_fit: 释放多余容量
    v.shrink_to_fit();
    println!("  shrink_to_fit 后: len={}, cap={}", v.len(), v.capacity());
}

// ============================================================================
// 7. Vec 的所有权语义
// ============================================================================
//
// Vec<T> 本身是"拥有型"——它独占堆上那段数据。
// 移动 Vec = 整体所有权移动；clone Vec = 深拷贝整个数组。

fn ownership_demo() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;                               // move: v1 所有权交给 v2
    // println!("{v1:?}");                     // ❌ v1 已经 move 走

    let v3 = v2.clone();                       // 深拷贝
    println!("  v2 = {v2:?}");
    println!("  v3 = {v3:?}, 这是独立的一份数据");

    // Vec<T: Copy> 元素的 indexing 不会 move 整个 Vec
    let v4 = vec![10, 20, 30];
    let x = v4[0];                             // i32 是 Copy, 直接复制
    println!("  v4[0] = {x}, v4 仍可用 = {v4:?}");

    // Vec<String> 的元素是 String (非 Copy), 直接 v[0] 会试图 move
    let v5 = vec!["a".to_string(), "b".to_string()];
    // let s = v5[0];                          // ❌ cannot move out of index
    let s = &v5[0];                            // ✅ 借用
    let s2 = v5[0].clone();                    // ✅ 显式 clone
    println!("  v5[0] 借用 = {s}, clone = {s2}");
}

// ============================================================================
// 8. 内存布局直觉
// ============================================================================
//
//   栈上 (24 B, 64 位):
//   ┌──────────┬──────────┬──────────┐
//   │   ptr    │   len    │   cap    │
//   └──────────┴──────────┴──────────┘
//        │
//        ▼
//   堆上 (cap × size_of::<T>())
//   ┌────┬────┬────┬────┬────┬...┐
//   │ 0  │ 1  │ 2  │ ?  │ ?  │   │  ← len 个真实元素 + 余下槽位
//   └────┴────┴────┴────┴────┴...┘
//
//   - len <= cap 永远成立
//   - cap 的扩张策略一般是 max(old*2, requested)
//   - 这就是为什么 Vec push 摊还是 O(1)（每次扩容代价 / 2x 元素 → 平均 O(1)）

fn layout_hint() {
    use std::mem::size_of;
    println!("  size_of::<Vec<i32>>()    = {} B (栈上 ptr+len+cap)", size_of::<Vec<i32>>());
    println!("  size_of::<Vec<String>>() = {} B (一样大, 因为只是个胖指针)", size_of::<Vec<String>>());
}

fn main() {
    println!("===== 1. 创建 =====");
    create_demo();

    println!("\n===== 2. 增加元素 =====");
    add_demo();

    println!("\n===== 3. 删除元素 =====");
    remove_demo();

    println!("\n===== 4. 访问元素 =====");
    access_demo();

    println!("\n===== 5. 遍历 =====");
    iter_demo();

    println!("\n===== 6. 容量增长 =====");
    capacity_demo();

    println!("\n===== 7. 所有权语义 =====");
    ownership_demo();

    println!("\n===== 8. 内存布局直觉 =====");
    layout_hint();

    println!("\n===== 要点回顾 =====");
    println!("· Vec<T> 是拥有型动态数组，3 种创建方式");
    println!("· push/pop/insert/remove/swap_remove 各有用法 + 复杂度");
    println!("· 索引越界 panic, 想安全用 .get() / .first() / .last()");
    println!("· iter / iter_mut / into_iter 三种遍历对应不同所有权语义");
    println!("· 已知规模就 with_capacity, 避免反复 realloc");
}
