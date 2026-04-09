use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 数组切片与向量切片 &[T]
//
//   &[T] 的本质：胖指针（fat pointer）
//     · 栈上仅占 2 个 usize：ptr（指向首个元素地址）+ len（元素个数）
//     · 不拥有数据，不负责释放，只是对连续内存的"只读借用窗口"
//     · T 可以是任意类型：i32、f64、String、自定义结构体……
//
//   数据来源可以是：
//     · 固定大小数组：[T; N]（通常在栈上）
//     · 动态数组 Vec<T>（堆上）
//     · 另一个 &[T] 的子区间（切片的切片）
//
//   &[T] vs &Vec<T> vs &[T; N]（为什么函数参数要写 &[T]？）
//     &Vec<T>  只能接受 Vec<T>（局限性最大）
//     &[T; N]  只能接受恰好 N 个元素的数组（过于严格）
//     &[T]     能接受数组 / Vec / 任意切片 → 最通用，是函数参数首选
//
//   &mut [T]（可变切片）：
//     可以原地修改切片中的元素（如排序、批量修改）
//     遵守借用规则：&[T] 和 &mut [T] 不能在同一时间活跃
//
//   切片模式匹配：
//     match 可以对切片进行结构解构，优雅处理空/单元素/多元素等情况
// ─────────────────────────────────────────────────────────────────────────

// ✅ 参数写 &[i32]，比 &Vec<i32> 更通用
// 既能接受栈上数组的切片（&arr[..]），也能接受 Vec 的切片（&vec[..]）
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum() // iter() 返回元素不可变引用的迭代器，sum() 求和
}

// ✅ 打印切片的基本信息，演示 &[T] 作为通用参数的优势
fn describe_slice(label: &str, data: &[i32]) {
    println!(
        "  {label}: len={}, is_empty={}, 内容={:?}",
        data.len(),
        data.is_empty(),
        data
    );
}

// ✅ 切片模式匹配：根据切片长度和结构分情况处理
fn summarize(nums: &[i32]) -> String {
    match nums {
        // 空切片
        [] => String::from("（空切片）"),
        // 恰好一个元素，x 绑定该元素的引用
        [x] => format!("仅一个元素: {x}"),
        // 恰好两个元素，分别绑定
        [first, second] => format!("两个元素: {first} 和 {second}"),
        // 三个及以上：首尾绑定，中间用 .. 忽略
        [first, .., last] => format!(
            "首={first}, 尾={last}, 共 {} 个元素",
            nums.len()
        ),
    }
}

// ✅ 找出切片中的最大值（返回 Option，空切片返回 None）
fn find_max(nums: &[i32]) -> Option<i32> {
    // 切片模式匹配：先处理空切片，再处理非空情况
    match nums {
        [] => None, // 空切片没有最大值
        [first, rest @ ..] => {
            // first: 第一个元素引用；rest: 剩余部分的切片
            let mut max = *first; // 解引用得到 i32 值
            for &x in rest {
                if x > max {
                    max = x;
                }
            }
            Some(max)
        }
    }
}

fn main() {
    println!("{}", "=== 数组切片与向量切片 &[T] ===".green().bold());

    // ─────────────────────────────────────────────────────────────────────
    println!("\n1、数组切片基础：对数组某段区间的只读借用");
    let arr: [i32; 5] = [10, 20, 30, 40, 50]; // 固定大小数组，存在栈上
    //                     0   1   2   3   4    ← 索引

    let s1: &[i32] = &arr[1..4]; // 索引 1,2,3 → 元素 20, 30, 40
    let s2: &[i32] = &arr[..];   // 整个数组 → 所有 5 个元素
    let s3: &[i32] = &arr[3..];  // 索引 3 到末尾 → 元素 40, 50
    let s4: &[i32] = &arr[..2];  // 索引 0 到 1 → 元素 10, 20

    println!("  arr 原始数组     = {:?}", arr);
    println!("  &arr[1..4]      = {:?}  (索引 1,2,3 对应的元素)", s1);
    println!("  &arr[..]        = {:?}  (整个数组的切片)", s2);
    println!("  &arr[3..]       = {:?}  (从索引 3 到末尾)", s3);
    println!("  &arr[..2]       = {:?}  (从头到索引 1)", s4);
    println!("  arr 切片后依然有效: {:?}", arr); // 切片只是借用，不移走所有权
    println!("小结：切片是对数组某段区间的借用窗口，原数组保留所有权，可继续使用");

    // ─────────────────────────────────────────────────────────────────────
    println!("\n2、&[T] 的本质：胖指针（ptr + len），不拥有数据");
    println!("  &[i32] 在栈上的内存布局（以 &arr[1..4] 为例）：");
    println!("  ┌────────────────────────────────────────────────┐");
    println!("  │ ptr: 指向 arr[1] 所在的内存地址（即 20 的位置）│");
    println!("  │ len: 3  （包含 3 个 i32 元素，不是字节数）     │");
    println!("  └────────────────────────────────────────────────┘");
    println!("  数组在栈上：[10][20][30][40][50]");
    println!("                   ↑←── ptr 指向这里");
    println!("                   └─────────────┘ len=3 个元素");
    println!();
    let demo_slice = &arr[1..4];
    println!(
        "  demo_slice: len={}, sum={}",
        demo_slice.len(),
        sum(demo_slice)
    );
    println!("  切片的复制开销极小：仅复制 ptr（8字节）和 len（8字节）");
    println!("  数据本身（20, 30, 40）一字节都没有被复制");
    println!("小结：胖指针是零开销抽象，无论切片多大，传递成本固定为 2×usize");

    // ─────────────────────────────────────────────────────────────────────
    println!("\n3、Vec 切片：Vec<T> 与 &[T] 的关系");
    let vec: Vec<i32> = vec![100, 200, 300, 400, 500]; // Vec 数据分配在堆上
    let v_all: &[i32] = &vec[..];    // 整个 Vec 的切片（等同于 vec.as_slice()）
    let v_part: &[i32] = &vec[1..3]; // 部分切片：200, 300

    println!("  vec（Vec<i32>，数据在堆）= {:?}", vec);
    println!("  &vec[..]   = {:?}  (等同于 vec.as_slice())", v_all);
    println!("  &vec[1..3] = {:?}  (部分切片)", v_part);
    println!("  vec.as_slice() 也返回 &[T]，是更明确的写法");
    println!("  sum(&vec[..]) = {}", sum(&vec[..]));
    println!("  sum(&vec[2..]) = {}", sum(&vec[2..])); // 300+400+500=1200
    println!("  Vec 切片后 vec 本身仍然有效: {:?}", vec);
    println!("小结：Vec<T> 通过 &vec[..] 或 vec.as_slice() 转为 &[T]，传给接受 &[T] 的函数");

    // ─────────────────────────────────────────────────────────────────────
    println!("\n4、函数参数写 &[T] 比 &Vec<T> 更通用（演示同一函数接受多种来源）");
    let arr2: [i32; 4] = [1, 2, 3, 4];          // 栈上固定大小数组
    let vec2: Vec<i32> = vec![10, 20, 30];       // 堆上动态数组
    let slice2: &[i32] = &arr2[1..];             // 另一个切片

    // sum() 参数是 &[i32]，三种来源都可以直接传入
    println!("  sum(&arr2)        = {}  (数组 [i32;4] → &[i32] 自动强制转换)", sum(&arr2));
    println!("  sum(&vec2)        = {}  (Vec<i32> → &[i32] 通过 Deref)", sum(&vec2));
    println!("  sum(slice2)       = {}  (&[i32] 直接传入)", sum(slice2));
    println!("  sum(&arr2[..2])   = {}  (数组切片的切片)", sum(&arr2[..2]));
    println!("  sum(&vec2[1..])   = {}  (Vec 的部分切片)", sum(&vec2[1..]));
    println!();

    // ❌ 如果参数写成 &Vec<i32>，数组和切片就无法传入
    // fn sum_bad(nums: &Vec<i32>) -> i32 { nums.iter().sum() }
    // sum_bad(&arr2);       // 编译错误：expected &Vec<i32>, found &[i32; 4]
    // sum_bad(&arr2[..]);   // 编译错误：expected &Vec<i32>, found &[i32]
    // sum_bad(slice2);      // 编译错误：expected &Vec<i32>, found &[i32]
    // 错误原因：&Vec<T> 不是 &[T] 的父类型，没有协变关系

    println!("  Rust 惯用法：接受集合只读数据时，参数用 &[T] 而非 &Vec<T>");
    describe_slice("arr2 全部", &arr2);
    describe_slice("vec2 全部", &vec2);
    describe_slice("slice2 部分", slice2);
    println!("小结：&[T] 是集合只读参数的最佳选择，一个函数同时兼容数组、Vec 和切片");

    // ─────────────────────────────────────────────────────────────────────
    println!("\n5、切片的常用方法");
    let data: &[i32] = &[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]; // 圆周率前几位

    // 基本信息
    println!("  data = {:?}", data);
    println!("  len()          = {}", data.len()); // 元素个数
    println!("  is_empty()     = {}", data.is_empty()); // 是否为空

    // 首尾访问（返回 Option<&T>，比直接 [0] 更安全，不会 panic）
    println!("  first()        = {:?}", data.first()); // 第一个元素，Some(&3)
    println!("  last()         = {:?}", data.last()); // 最后一个元素，Some(&5)

    // 查找
    println!("  contains(&9)   = {}", data.contains(&9)); // 是否包含 9
    println!("  contains(&7)   = {}", data.contains(&7)); // 是否包含 7

    // 迭代器：iter() 返回 &T 的迭代器，配合链式调用处理数据
    let doubled: Vec<i32> = data.iter().map(|&x| x * 2).collect();
    println!("  iter().map(x*2)= {:?}", doubled);
    let evens: Vec<i32> = data.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("  iter().filter(偶数) = {:?}", evens);

    // 查找最大值（使用我们自定义的函数，展示模式匹配）
    println!("  find_max()     = {:?}", find_max(data));

    // 切片的切片
    let sub: &[i32] = &data[2..7]; // 取第 2~6 位
    println!("  &data[2..7]    = {:?}", sub);

    // 空切片的安全访问
    let empty: &[i32] = &[];
    println!("  空切片 first() = {:?}, last() = {:?}", empty.first(), empty.last());
    // ❌ 危险：直接下标访问空切片会 panic
    // let _ = empty[0]; // panic: index out of bounds: the len is 0 but the index is 0
    // ✅ 安全：用 first()/last() 或 get(i) 返回 Option，不会 panic
    println!("  空切片 get(0)  = {:?}", empty.get(0)); // 返回 None，不 panic

    println!("小结：用 first()/last()/get() 替代直接下标访问，避免越界 panic");

    // ─────────────────────────────────────────────────────────────────────
    println!("\n6、可变切片 &mut [T]：原地修改切片元素");

    // sort()：对可变切片原地排序（修改的是原数组的数据）
    let mut nums = [5, 3, 8, 1, 9, 2, 7, 4, 6];
    println!("  排序前: {:?}", nums);
    nums.sort(); // 等同于 (&mut nums[..]).sort()
    println!("  sort() 后: {:?}", nums);

    // sort_by()：自定义排序规则（降序）
    let mut scores: Vec<i32> = vec![88, 72, 95, 60, 84];
    println!("  排序前: {:?}", scores);
    scores.sort_by(|a, b| b.cmp(a)); // 降序：b 与 a 比较（反转）
    println!("  降序排序后: {:?}", scores);

    // 通过可变切片批量修改元素
    let mut data2 = [1, 2, 3, 4, 5];
    let part: &mut [i32] = &mut data2[1..4]; // 可变切片，指向索引 1,2,3
    for x in part.iter_mut() {
        *x *= 10; // iter_mut() 返回 &mut i32，用 * 解引用后修改
    }
    println!("  将 &mut data2[1..4] 每元素 ×10 后: {:?}", data2);

    // ❌ 错误：可变切片借用活跃期间，不能同时有不可变引用
    // let mut v = vec![1, 2, 3];
    // let read = &v[..];           // 不可变借用 v
    // let write = &mut v[..];      // 编译错误：cannot borrow `v` as mutable
    //                              // because it is also borrowed as immutable
    // println!("{:?} {:?}", read, write);

    // ✅ 正确：先用完不可变借用，再创建可变借用
    let mut v = vec![1, 2, 3];
    let read = &v[..];
    println!("  先读: {:?}", read);       // read 借用在此结束（NLL）
    let write = &mut v[..];
    write[0] = 100;                       // 通过可变切片修改第一个元素
    println!("  再写后 v: {:?}", v);

    println!("小结：&mut [T] 可原地修改元素，sort()/iter_mut() 是常用操作，借用规则同样适用");

    // ─────────────────────────────────────────────────────────────────────
    println!("\n7、切片模式匹配：对切片结构进行解构");

    // 演示 summarize 函数对不同长度切片的处理
    let test_cases: &[&[i32]] = &[
        &[],              // 空切片
        &[42],            // 单元素
        &[1, 2],          // 两个元素
        &[10, 20, 30],    // 三个元素
        &[1, 2, 3, 4, 5], // 五个元素
    ];

    for &case in test_cases {
        let result = summarize(case);
        println!("  {:?} → {}", case, result);
    }

    println!();
    println!("  模式语法详解：");
    println!("  []                    → 空切片（精确匹配零元素）");
    println!("  [x]                   → 单元素，x 绑定第 0 个元素的引用");
    println!("  [a, b]                → 精确两个元素，分别绑定");
    println!("  [first, ..]           → 至少一个元素，.. 忽略剩余部分");
    println!("  [first, .., last]     → 至少两个元素，绑定首尾，.. 忽略中间");
    println!("  [a, b, rest @ ..]     → rest 绑定为剩余部分的切片引用");

    // 演示 rest @ .. 语法（将剩余部分作为切片绑定到 rest）
    let nums_demo: &[i32] = &[10, 20, 30, 40, 50];
    match nums_demo {
        [] => println!("  空"),
        [head, tail @ ..] => {
            // head: &i32，tail: &[i32]（剩余部分的切片）
            println!("  head={head}, tail={tail:?}"); // head=10, tail=[20,30,40,50]
        }
    }

    // 实战：使用模式匹配实现"安全 split_first"语义
    fn process(data: &[i32]) {
        match data {
            [] => println!("    → 数据为空，跳过处理"),
            [only] => println!("    → 唯一元素: {only}，直接处理"),
            [head, rest @ ..] => {
                // head 是第一个元素，rest 是剩余切片
                let rest_sum: i32 = rest.iter().sum();
                println!("    → 头部: {head}，剩余 {} 个元素之和: {rest_sum}", rest.len());
            }
        }
    }
    println!("  process 演示：");
    process(&[]);
    process(&[99]);
    process(&[1, 2, 3, 4, 5]);

    println!("小结：切片模式匹配让处理不同长度的数组数据变得简洁优雅，是函数式风格的体现");

    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "── 本章知识点总结 ──".cyan().bold());
    println!("  &[T]       → 胖指针，通用只读切片，函数参数首选");
    println!("  &mut [T]   → 胖指针，可原地修改，遵守借用规则");
    println!("  &arr[..]   → 数组转切片");
    println!("  &vec[..]   → Vec 转切片（或 vec.as_slice()）");
    println!("  &[T] > &Vec<T>：函数参数写 &[T]，兼容更多来源");
    println!("  模式匹配   → 对 [] / [x] / [first, .., last] 等结构解构");
    println!("  安全访问   → 用 first() / last() / get(i) 替代 [i]，避免 panic");
}