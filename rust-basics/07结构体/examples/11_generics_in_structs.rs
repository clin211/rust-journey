#![allow(dead_code)]

use colored::*;
use std::fmt::Display;

// ─────────────────────────────────────────────────────────────────────────────
// 泛型结构体（Generic Structs）
//
// 到目前为止，我们的结构体字段类型都是「写死」的（u32, String, f64 ...）。
// 但是现实世界里经常会遇到这种需求：
//   · 容器：「装任意类型的列表 / 栈 / 队列 / 键值对」
//   · 几何：「点既可以是整数坐标 (i32, i32)，也可以是浮点 (f64, f64)」
//   · 业务：「缓存 key/value 都可以任意类型」
//
// 在 C 里这几乎是灾难：你要么用 void*（丢失类型信息），要么复制十份几乎一样的代码。
// 在 Go 里（1.18 前）只能 interface{} + 运行时反射。
// 在 Java 里有泛型，但有类型擦除、装箱拆箱，运行时开销不小。
//
// Rust 的答案：编译期单态化（monomorphization）的泛型。
//   · 写一次 struct Point<T>，编译器按使用的具体 T 生成多份特化版本
//   · 零运行时开销 —— 生成的代码和你「手写 PointI32/PointF64」完全一致
//   · 类型安全 —— 所有操作在编译期检查，不会有类型错误下到运行时
//
// 本示例把泛型结构体的几个关键用法系统讲清楚：
//
//   1. 单类型参数：struct Container<T>
//   2. 多类型参数：struct Pair<T, U>
//   3. 通用 impl：impl<T> Container<T>
//   4. 特化 impl：impl Container<f64>（只对特定 T 可用的方法）
//   5. 带 trait bound 的 impl：impl<T: Display + PartialOrd> ...
//   6. where 子句：更灵活的 bound 写法
//   7. 泛型 + 生命周期的组合：struct Ref<'a, T>
//   8. 单态化可视化：为什么"零成本"不是夸张
//
// 关键直觉：
//   · 泛型不是运行时能力，而是「让编译器帮你写多份代码」
//   · 每个具体 T 都会得到一份独立的机器码
//   · 这让 Rust 既能表达高级抽象，又不牺牲性能
// ─────────────────────────────────────────────────────────────────────────────

// ── 1. 单类型参数 ────────────────────────────────────────────────────────────
// 最基础的泛型结构体：一个盒子，装什么都行
struct Container<T> {
    value: T,
}

// 通用 impl：对所有 T 都生效
// 注意 impl 后面也要写 <T>，它是「这个 impl 块引入的类型参数」
impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    // 消费自身，取出内部值
    fn into_inner(self) -> T {
        self.value
    }

    // 借用内部值
    fn get(&self) -> &T {
        &self.value
    }
}

// ── 2. 多类型参数 ────────────────────────────────────────────────────────────
// 两个类型参数可以互相独立，也可以组合
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    // 交换字段顺序：Pair<T, U> → Pair<U, T>
    // 注意返回类型里 T 和 U 位置颠倒了
    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

// ── 3. 泛型几何点：Point<T> ──────────────────────────────────────────────────
// 经典泛型例子：x / y 可以是 i32 / f64 / 任何数值类型
// 但必须强调：T 是一个类型，不是"数值类型的集合"
// 如果你想让 Point<T> 支持加减乘除，就要通过 trait bound 约束 T
#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

// ── 通用 impl：对所有 Point<T> 都可用 ───────────────────────────────────────
// 不对 T 加任何约束 → 只能做"不依赖 T 具体能力"的事情
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    // 返回字段引用：不需要 T 有任何能力
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// ── 特化 impl：只有 Point<f64> 才有 distance_from_origin ─────────────────────
// 关键用法：给「具体类型参数」写特化实现
// 这让 Point<f64>::distance_from_origin() 可用，而 Point<i32> 没有这个方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ── 特化 impl：只有 Point<i32> 才有的方法 ───────────────────────────────────
// 可以为不同的具体 T 写完全不同的方法集
impl Point<i32> {
    // 曼哈顿距离（整数坐标场景）
    fn manhattan_distance(&self, other: &Point<i32>) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// ── 带 trait bound 的 impl ───────────────────────────────────────────────────
// impl<T: Trait> 表示「只有当 T 满足 Trait 时，这段 impl 才生效」
// 这里要求 T: Display（能被 {} 打印）+ PartialOrd（能比较大小）
impl<T: Display + PartialOrd> Point<T> {
    // 打印「较大的那个坐标分量」
    fn print_larger(&self) {
        if self.x > self.y {
            println!("  较大的分量是 x = {}", self.x);
        } else {
            println!("  较大的分量是 y = {}", self.y);
        }
    }
}

// ── 4. where 子句：复杂 bound 的更优雅写法 ──────────────────────────────────
// 当 trait bound 多起来，放在尖括号里会挤成一团
// where 子句把所有约束放在签名后面，阅读更舒服
#[derive(Debug)]
struct Stats<T>
where
    T: Copy + PartialOrd + Display,
{
    min: T,
    max: T,
}

impl<T> Stats<T>
where
    T: Copy + PartialOrd + Display,
{
    // 从一个切片中提取最小/最大值
    // 不空切片前提：调用方应保证 items.len() >= 1（这里不做错误处理，专注泛型用法）
    fn from_slice(items: &[T]) -> Self {
        let mut min = items[0];
        let mut max = items[0];
        for &v in &items[1..] {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
        Stats { min, max }
    }

    fn describe(&self) {
        println!("  最小 = {}, 最大 = {}", self.min, self.max);
    }
}

// ── 5. 泛型 + 生命周期参数：Ref<'a, T> ──────────────────────────────────────
// 结构体可以同时带「生命周期参数」和「类型参数」
// 惯例：生命周期写在前，类型参数写在后
// 这种写法在集合迭代器、slice 封装等场景非常常见
struct Ref<'a, T> {
    value: &'a T,
}

impl<'a, T: Display> Ref<'a, T> {
    fn new(value: &'a T) -> Self {
        Ref { value }
    }

    fn show(&self) {
        println!("  Ref 指向的值 = {}", self.value);
    }
}

// ── 6. 经典对比：标准库里常见的泛型结构体 ───────────────────────────────────
// 下面都是你早就在用的泛型结构体，只是以前没意识到：
//   · Vec<T>          → 动态数组，装任意 T
//   · Option<T>       → 要么有值 Some(T)，要么没有 None
//   · Result<T, E>    → 成功 Ok(T) 或失败 Err(E)，两个类型参数！
//   · HashMap<K, V>   → 键值对
//   · Box<T>          → 堆分配的 T
//   · Rc<T>/Arc<T>    → 引用计数的 T
//
// 理解了泛型结构体，你会发现：Rust 的标准库就是一大堆精心设计的泛型容器

fn main() {
    println!("{}", "=== 泛型结构体（Generic Structs） ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、最基础的泛型：Container<T>");
    // ─────────────────────────────────────────

    let c_int: Container<i32> = Container::new(42);
    let c_str: Container<String> = Container::new("Rust".into());
    let c_vec: Container<Vec<i32>> = Container::new(vec![1, 2, 3]);

    println!("  c_int.get()  = {}", c_int.get());
    println!("  c_str.get()  = {}", c_str.get());
    println!("  c_vec.get()  = {:?}", c_vec.get());

    // 消费后取出内部值
    let raw_int = c_int.into_inner();
    println!("  into_inner() 拿到原始值 {}", raw_int);

    println!("  同一个 Container<T> 结构体，装了 i32 / String / Vec<i32> 三种不同类型");
    println!("  Rust 在编译时为每种 T 分别生成一份特化代码");
    println!("小结：泛型让一份代码复用到任意类型，同时保持类型安全和零开销");

    // ─────────────────────────────────────────
    println!("\n2、多类型参数：Pair<T, U>");
    // ─────────────────────────────────────────

    let p1 = Pair::new(1, "hello");              // T=i32, U=&str
    let p2 = Pair::new(3.14, true);              // T=f64, U=bool

    println!("  p1.first = {}, p1.second = {}", p1.first, p1.second);
    println!("  p2.first = {}, p2.second = {}", p2.first, p2.second);

    // 交换两个字段：Pair<T, U> → Pair<U, T>
    let swapped = p1.swap();
    println!("  p1.swap() 之后 first = {}, second = {}", swapped.first, swapped.second);

    println!("  Pair 两个类型参数彼此独立，swap() 返回类型也跟着变");
    println!("小结：多参数泛型适合「字段之间没有关系，各自可以是任意类型」的场景");

    // ─────────────────────────────────────────
    println!("\n3、泛型几何点：Point<T>");
    // ─────────────────────────────────────────

    // 同一个 Point<T> 模板，产出两种完全不同的类型
    let p_int = Point::<i32>::new(3, 4);          // 显式写 Point::<i32>::new
    let p_float = Point::new(3.0_f64, 4.0_f64);   // 由字面量推断出 f64

    println!("  p_int    = ({}, {})", p_int.x, p_int.y);
    println!("  p_float  = ({}, {})", p_float.x, p_float.y);

    // ⚠️ Point<i32> 和 Point<f64> 是两种完全不同的类型，不能互相赋值
    // let bad: Point<f64> = p_int;               // ❌ mismatched types

    println!("  Point<i32> 和 Point<f64> 是不同类型，不能互相赋值");
    println!("小结：泛型结构体的不同具体化是「不同类型」，类型系统会严格区分");

    // ─────────────────────────────────────────
    println!("\n4、特化 impl：只有某种 T 才有的方法");
    // ─────────────────────────────────────────

    // distance_from_origin 只在 impl Point<f64> 里定义
    let d = p_float.distance_from_origin();
    println!("  p_float.distance_from_origin() = {:.3}", d);

    // p_int.distance_from_origin();              // ❌ Point<i32> 没有这个方法

    // manhattan_distance 只在 impl Point<i32> 里
    let q_int = Point::new(0, 0);
    let md = p_int.manhattan_distance(&q_int);
    println!("  p_int.manhattan_distance(&q_int) = {}", md);

    // p_float.manhattan_distance(...)            // ❌ Point<f64> 没有这个方法

    println!("  特化 impl 能为「某一种具体 T」增加独占方法，非常强大");
    println!("  这就是为什么 Vec<u8> 有 .as_ascii() 系列方法，而 Vec<i32> 没有");

    println!("小结：impl Type<具体T> 让你按「具体参数」定制不同的方法集");

    // ─────────────────────────────────────────
    println!("\n5、带 trait bound 的 impl：限制 T 必须满足某些能力");
    // ─────────────────────────────────────────

    // 这两个 Point<T> 的 T 都是 i32，i32 同时实现了 Display 和 PartialOrd
    // 所以 print_larger 可用
    let p1 = Point::new(10, 5);
    p1.print_larger();

    let p2 = Point::new(3.14_f64, 2.71_f64);
    p2.print_larger();

    // 如果 T 不满足 Display + PartialOrd，编译期就会拒绝
    // 例如 Point<Vec<i32>> 不能调用 print_larger，因为 Vec<i32> 没实现 PartialOrd

    println!("  impl<T: Display + PartialOrd> Point<T> 的方法只对满足约束的 T 可用");
    println!("  不满足时 Rust 直接在调用点报错，不会到运行时才爆");
    println!("小结：trait bound 让泛型拥有「有条件的能力」，灵活度与安全性兼顾");

    // ─────────────────────────────────────────
    println!("\n6、where 子句：复杂 bound 的优雅写法");
    // ─────────────────────────────────────────

    // 两种写法等价：
    //   impl<T: Copy + PartialOrd + Display> Stats<T>
    //   impl<T> Stats<T> where T: Copy + PartialOrd + Display
    // 后者在 bound 多时可读性更好

    let nums = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let stats = Stats::<i32>::from_slice(&nums);
    stats.describe();

    let floats = [1.1, 2.2, 0.5, 3.3, 2.7];
    let stats = Stats::from_slice(&floats);
    stats.describe();

    let words = ["rust", "is", "awesome", "fast", "safe"];
    let stats = Stats::from_slice(&words);
    stats.describe();

    println!("  Stats<T> 同一份代码适配了 i32 / f64 / &str 三种类型");
    println!("  只要 T 满足 Copy + PartialOrd + Display 即可");
    println!("小结：where 子句把 bound 挪到签名后面，让复杂约束更易读");

    // ─────────────────────────────────────────
    println!("\n7、泛型 + 生命周期：Ref<'a, T>");
    // ─────────────────────────────────────────

    let n = 42;
    let r = Ref::new(&n);                        // T=i32, 'a='临时
    r.show();

    let s = String::from("borrowed");
    let r2 = Ref::new(&s);                       // T=String, 'a=s 的生命周期
    r2.show();

    println!("  结构体可以同时带生命周期 'a 和类型参数 T");
    println!("  写法惯例：<'a, T> —— 生命周期在前，类型参数在后");
    println!("  这是迭代器、切片封装等「借用 + 泛型」场景的标配");
    println!("小结：生命周期和类型参数可以自由组合，描述「借用某个类型」");

    // ─────────────────────────────────────────
    println!("\n8、标准库里的泛型结构体：你其实早就在用");
    // ─────────────────────────────────────────

    // Option<T>：最常见的泛型 enum（底层也是一种类型构造器）
    let some_int: Option<i32> = Some(5);
    let some_str: Option<&str> = Some("hello");

    // Result<T, E>：两个类型参数
    let ok_result: Result<i32, String> = Ok(42);
    let err_result: Result<i32, String> = Err("failed".into());

    // Vec<T>：动态数组
    let v_int: Vec<i32> = vec![1, 2, 3];
    let v_str: Vec<String> = vec!["a".into(), "b".into()];

    // HashMap<K, V>：两个类型参数
    use std::collections::HashMap;
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("one".into(), 1);

    println!("  Option<i32>     = {:?}", some_int);
    println!("  Option<&str>    = {:?}", some_str);
    println!("  Result<i32, _>  = {:?} 和 {:?}", ok_result, err_result);
    println!("  Vec<i32>        = {:?}", v_int);
    println!("  Vec<String>     = {:?}", v_str);
    println!("  HashMap<String, i32> = {:?}", map);

    println!("  标准库的几乎所有容器都是泛型结构体（或泛型枚举）");
    println!("  Vec / Option / Result / HashMap / Box / Rc / Arc 无一例外");
    println!("小结：掌握了泛型结构体，你就掌握了整个标准库的「组合积木」");

    // ─────────────────────────────────────────
    println!("\n9、单态化（monomorphization）：零成本的秘密");
    // ─────────────────────────────────────────

    // 泛型在 Rust 里是「编译期」的能力
    // Container<i32> 和 Container<String> 在编译后
    // 会被展开成两份独立的代码，就像你手写了两个不同的结构体
    //
    // 等价于下面这种「手写展开」的效果：
    //
    //   struct Container_i32    { value: i32 }
    //   struct Container_String { value: String }
    //
    // 好处：运行时零开销，每一个 Container<T>::get() 调用
    //       都是针对具体类型的优化代码，不需要任何运行时「查表」
    //
    // 代价：编译时间略长、二进制体积略大（几乎不可见的量级）

    let a = Container::new(1_i32);
    let b = Container::new("x".to_string());

    println!("  Container<i32>   的方法调用被编译成针对 i32 的代码");
    println!("  Container<String> 的方法调用被编译成针对 String 的代码");
    println!("  两者完全独立，运行时没有任何 dispatch/查表");
    println!("  Rust 称之为「零成本抽象」：写得优雅，跑得飞快");

    // 可以查看具体类型
    println!("  a 的类型是 Container<i32>, value = {}", a.value);
    println!("  b 的类型是 Container<String>, value = {}", b.value);
    println!("小结：Rust 泛型 = 编译期单态化 = 零运行时开销 + 最大表达力");

    // ─────────────────────────────────────────
    println!("\n【总结】泛型结构体要点");
    // ─────────────────────────────────────────
    println!("  · 语法：struct S<T> {{ ... }}，impl<T> S<T> {{ ... }}");
    println!("  · 多参数：struct Pair<T, U>，impl<T, U> Pair<T, U>");
    println!("  · 通用 impl：对所有 T 都生效（但方法能做的事受限于 T 的能力）");
    println!("  · 特化 impl：为「具体 T」定制方法集（Point<f64>、Point<i32>）");
    println!("  · trait bound：impl<T: Trait> 让方法依赖 T 的特定能力");
    println!("  · where 子句：多约束更易读，等价于尖括号里的写法");
    println!("  · 生命周期 + 类型：<'a, T> 惯例，两者可以并存");
    println!("  · 单态化：Rust 在编译期给每个具体 T 生成一份独立代码，零运行时开销");
    println!("  · 标准库：Option/Result/Vec/HashMap/Box 都是泛型结构体（或泛型枚举）");
    println!();
    println!("  进阶预告：");
    println!("    · const 泛型：struct Array<T, const N: usize>（固定长度数组）");
    println!("    · 关联类型：trait 内的 type 别名，让 trait 更灵活");
    println!("    · 泛型方法：方法本身带类型参数，不一定跟着 impl 走");
    println!("  这些话题会在「trait 与泛型」专题里继续展开。");
}
