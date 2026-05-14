//! 11. 其它常用集合：VecDeque / BinaryHeap / LinkedList
//!
//! 运行：cargo run --example 11_other_collections
//!
//! 标准库 std::collections 里的集合除了 Vec / HashMap / BTreeMap / HashSet / BTreeSet 之外，
//! 还有几个"二线"集合，在合适的场景里能省不少事：
//!
//!   VecDeque<T>      双端队列（环形缓冲区）
//!   BinaryHeap<T>    二叉堆（最大堆）—— 优先队列首选
//!   LinkedList<T>    双向链表 —— 工程里几乎不用，仅做对照

#![allow(dead_code)]
// 注：111 行的两层 if let 是为了"先判 len, 再窥视元素"的清晰展示, 不折叠
#![allow(clippy::collapsible_if)]

use std::collections::{BinaryHeap, LinkedList, VecDeque};

// ============================================================================
// 1. VecDeque：双端队列
// ============================================================================
//
// VecDeque 用环形缓冲区实现, 头尾的 push / pop 都是 **O(1)**。
// 这就是它和 Vec 的本质差别——Vec 头部插入要 memmove, 复杂度 O(n).
//
//   操作               Vec       VecDeque
//   ────────────────  ────────  ──────────
//   push_back / pop   O(1)*     O(1)
//   push_front        O(n)      O(1)
//   pop_front         O(n)      O(1)
//   随机索引            O(1)      O(1) (内部还要做一次"环形->线性")
//
// 用途: 队列 (BFS / 任务队列 / 滑动窗口)

fn vecdeque_demo() {
    let mut dq: VecDeque<i32> = VecDeque::new();

    // 双端入队
    dq.push_back(1);
    dq.push_back(2);
    dq.push_back(3);
    dq.push_front(0);
    dq.push_front(-1);
    println!("  初始: {dq:?}");          // [-1, 0, 1, 2, 3]

    // 双端出队
    println!("  pop_front -> {:?}", dq.pop_front());
    println!("  pop_back  -> {:?}", dq.pop_back());
    println!("  剩余: {dq:?}");

    // 用作 BFS 队列
    println!("  --- BFS 模拟 ---");
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((0, 0));
    while let Some(node) = queue.pop_front() {
        println!("  访问: {node:?}");
        if node.0 < 2 {
            queue.push_back((node.0 + 1, node.1));
        }
        if node == (0, 0) {
            queue.push_back((node.0, node.1 + 1));
        }
    }

    // 滑动窗口: 维护"窗口大小不超过 K"
    println!("  --- 滑动窗口 ---");
    let stream = [10, 20, 30, 40, 50, 60, 70];
    let k = 3;
    let mut window: VecDeque<i32> = VecDeque::new();
    for x in stream {
        if window.len() == k { window.pop_front(); }
        window.push_back(x);
        println!("    after {x}: {window:?}, sum={}", window.iter().sum::<i32>());
    }
}

// ============================================================================
// 2. BinaryHeap：最大堆 / 优先队列
// ============================================================================
//
// 默认是**最大堆** (`pop()` 拿出最大值)。
// 想要最小堆? 配合 `Reverse(...)`。
//
//   操作               复杂度
//   push / pop         O(log n)
//   peek               O(1)

fn binary_heap_demo() {
    use std::cmp::Reverse;

    let mut heap = BinaryHeap::from([3, 1, 4, 1, 5, 9, 2, 6, 5]);
    println!("  从 [3,1,4,1,5,9,2,6,5] 建堆");
    println!("  peek (max) = {:?}", heap.peek());

    // 顺序 pop, 拿到从大到小的序列
    print!("  顺序 pop : ");
    while let Some(v) = heap.pop() { print!("{v} "); }
    println!();

    // 最小堆: 用 Reverse
    let mut min_heap: BinaryHeap<Reverse<i32>> =
        [3, 1, 4, 1, 5, 9, 2].into_iter().map(Reverse).collect();
    print!("  最小堆 pop: ");
    while let Some(Reverse(v)) = min_heap.pop() { print!("{v} "); }
    println!();

    // Top-K 算法: 用最小堆维护 K 个最大值
    let stream = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let k = 5;
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k);
    for x in stream {
        if heap.len() < k {
            heap.push(Reverse(x));
        } else if let Some(&Reverse(min)) = heap.peek() {
            if x > min {
                heap.pop();
                heap.push(Reverse(x));
            }
        }
    }
    let mut top: Vec<i32> = heap.into_iter().map(|Reverse(x)| x).collect();
    top.sort_unstable_by(|a, b| b.cmp(a));
    println!("  Top-5 最大值 = {top:?}");
}

// ============================================================================
// 3. 自定义优先级
// ============================================================================
//
// 让 BinaryHeap 按"自定义维度"排序: 让结构体实现 Ord, 或者把要比的字段放第一位。

fn task_priority_demo() {
    #[derive(Debug, PartialEq, Eq)]
    struct Task {
        priority: u8,
        id: u64,
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // 注意: 默认是最大堆。我们想让 "priority 大的先出", 直接顺序比即可
            self.priority.cmp(&other.priority).then(self.id.cmp(&other.id))
        }
    }

    let mut tasks = BinaryHeap::new();
    tasks.push(Task { priority: 1, id: 100 });
    tasks.push(Task { priority: 5, id: 101 });
    tasks.push(Task { priority: 3, id: 102 });
    tasks.push(Task { priority: 5, id: 103 });

    while let Some(t) = tasks.pop() {
        println!("  执行 -> {t:?}");
    }
}

// ============================================================================
// 4. LinkedList：知道有这个就行
// ============================================================================
//
// Rust 标准库提供了双向链表, 但**几乎不该用**：
//   - 缓存不友好 (节点散落在堆里)
//   - 任何操作的常数都比 Vec 大
//   - 现代 CPU 对连续内存极度友好
//
// 真正适合 LinkedList 的场景在 Rust 里也极少：
//   - 频繁在中间插入 (但 VecDeque 又能解决头尾)
//   - 需要"切分 / 拼接"链表 (LinkedList::split_off)
// 这里只演示 split_off, 别的用法看看就行。

fn linked_list_demo() {
    let mut a: LinkedList<i32> = LinkedList::new();
    for x in 1..=5 { a.push_back(x); }
    println!("  原链表 = {a:?}");

    // O(1) 切分
    let mut b = a.split_off(2);
    println!("  split_off(2): a = {a:?}, b = {b:?}");

    // O(1) 拼接
    a.append(&mut b);
    println!("  append 后: a = {a:?}, b = {b:?}");

    println!("  ⚠️ 实际工程里 90% 用 Vec / VecDeque 就够了, LinkedList 极少出场");
}

// ============================================================================
// 5. 集合选型小结
// ============================================================================

fn selection_chart() {
    println!("  ┌──────────────────────────────┬──────────────────────────┐");
    println!("  │ 你的需求                      │ 选哪个                    │");
    println!("  ├──────────────────────────────┼──────────────────────────┤");
    println!("  │ 动态数组 (顺序、索引)          │ Vec<T>                    │");
    println!("  │ 字符串                        │ String                    │");
    println!("  │ 键值映射 (无序、最快)          │ HashMap<K, V>             │");
    println!("  │ 键值映射 (有序、范围查询)       │ BTreeMap<K, V>            │");
    println!("  │ 集合 (去重、集合运算、无序)     │ HashSet<T>                │");
    println!("  │ 集合 (去重、有序)              │ BTreeSet<T>               │");
    println!("  │ 双端队列 / BFS / 滑动窗口      │ VecDeque<T>               │");
    println!("  │ 优先队列 / Top-K              │ BinaryHeap<T>             │");
    println!("  │ 双向链表 (极少用)              │ LinkedList<T>             │");
    println!("  └──────────────────────────────┴──────────────────────────┘");
}

fn main() {
    println!("===== 1. VecDeque =====");
    vecdeque_demo();

    println!("\n===== 2. BinaryHeap (最大堆) =====");
    binary_heap_demo();

    println!("\n===== 3. 自定义优先级 =====");
    task_priority_demo();

    println!("\n===== 4. LinkedList =====");
    linked_list_demo();

    println!("\n===== 5. 选型小结 =====");
    selection_chart();

    println!("\n===== 要点回顾 =====");
    println!("· VecDeque 头尾 O(1), 是 BFS / 滑动窗口的标准容器");
    println!("· BinaryHeap 默认最大堆, 用 Reverse 包装得到最小堆");
    println!("· LinkedList 几乎不用, 现代 CPU 偏爱 Vec / VecDeque");
    println!("· 选型先想'访问模式': 是否要顺序、范围查询、最值、双端");
}
