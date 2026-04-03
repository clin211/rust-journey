use colored::*;

fn makes_copy(x: i32) {
    println!("makes_copy 收到了: {x}");
}

// ─────────────────────────────────────────────────────────────────────────
// 为自定义类型实现 Copy 的条件：
//   1. 类型的所有字段都实现了 Copy
//   2. 类型没有实现 Drop（实现了 Drop 意味着有需要特殊清理的资源）
//   3. 显式为类型加上 #[derive(Copy, Clone)]
//
// 常见 Copy 类型：
//   整数：i8/i16/i32/i64/i128/isize/u8/u16/u32/u64/u128/usize
//   浮点：f32/f64
//   布尔：bool
//   字符：char
//   不可变引用：&T（注意：&mut T 不是 Copy！）
//   只包含 Copy 字段的元组和数组
//
// 常见非 Copy 类型（拥有堆资源或实现了 Drop）：
//   String、Vec<T>、Box<T>、HashMap、File、MutexGuard 等
// ─────────────────────────────────────────────────────────────────────────

#[derive(Debug, Copy, Clone)] // 自定义类型可以派生 Copy
struct Point {
    x: f64,
    y: f64,
}

// #[derive(Copy, Clone)]
// struct BadPoint {
//     name: String, // ❌ String 没有 Copy，所以 BadPoint 不能 derive Copy
// }

fn main() {
    println!("{}", "=== Copy 类型 ===".green().bold());

    println!("\n1、整数类型是 Copy：赋值时 bitwise 复制，双方都可用");
    let x: i32 = 10;
    let y = x; // 栈上 4 字节直接复制，没有堆操作
    println!("x = {x}, y = {y}"); // 两个都能用
    println!("小结：Copy 类型赋值就是把栈上的字节复制一份，原变量继续有效");

    println!("\n2、bool、char、只包含 Copy 字段的元组/数组也是 Copy");
    let flag = true;
    let another_flag = flag;
    let letter = 'R';
    let another_letter = letter;
    let point = (3_i32, 5_i32); // 元组：所有字段都是 Copy，元组也是 Copy
    let another_point = point;
    println!("flag = {flag}, another_flag = {another_flag}");
    println!("letter = {letter}, another_letter = {another_letter}");
    println!("point = {:?}, another_point = {:?}", point, another_point);
    println!("小结：所有字段都 Copy → 组合类型才能 Copy");

    println!("\n3、自定义 struct 可以 derive Copy（前提：所有字段都 Copy）");
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // Copy，不是 move
    println!("p1 = {:?}", p1); // p1 仍然有效
    println!("p2 = {:?}", p2);
    println!("小结：#[derive(Copy, Clone)] 让自定义类型也能像整数一样赋值");

    println!("\n4、把 Copy 类型传给函数时，原值仍然可用");
    let number = 42_i32;
    makes_copy(number); // 复制一份传进去，number 本身不动
    println!("number 仍然可用: {number}");
    println!("小结：Copy 参数传递不会转移所有权，函数得到的是副本");

    println!("\n5、不可变引用 &T 也是 Copy（但 &mut T 不是！）");
    let text = String::from("Rust ownership");
    let r1: &String = &text;
    let r2 = r1; // &String 是 Copy，r1 的引用地址被复制给 r2
    // r1 和 r2 都指向同一块数据，但 text 才是真正的 owner
    println!("r1 = {r1}");
    println!("r2 = {r2}");
    println!("text 仍然是真正的 owner: {text}");
    println!("小结：复制引用 ≠ 复制数据；引用只是一个地址，Copy 复制的是地址");

    println!("\n  [重要区别] &T 是 Copy，&mut T 不是");
    let mut s = String::from("hello");
    let m1 = &mut s;
    // let m2 = m1; // ❌ &mut T 不是 Copy，m1 会 move 给 m2
    // 且同时只能有一个 &mut，所以 &mut T 根本就不能 Copy
    m1.push_str(" world");
    println!("s = {s}");

    println!("\n6、错误演示（注释）：把 String/Vec 当成 Copy 类型使用");
    let name = String::from("Rust");
    let numbers = vec![1, 2, 3];
    // let copied_name = name;
    // println!("name = {name}");
    // ❌ String 不是 Copy，赋值是 move，name 失效

    // let copied_numbers = numbers;
    // println!("numbers = {:?}", numbers);
    // ❌ Vec<T> 不是 Copy，赋值是 move，numbers 失效

    println!("只读场景用借用：");
    println!("name = {}", &name);
    println!("numbers = {:?}", &numbers);

    println!("需要独立副本才 clone（有堆分配开销）：");
    let copied_name = name.clone();
    let copied_numbers = numbers.clone();
    println!("name = {name}, copied_name = {copied_name}");
    println!("numbers = {:?}, copied_numbers = {:?}", numbers, copied_numbers);

    println!("\n7、判断 Copy 的经验：");
    println!("  ✅ 纯粹存活在栈上的值 → 通常是 Copy（整数、bool、char、&T）");
    println!("  ❌ 拥有堆资源的值    → 不是 Copy（String、Vec、Box、HashMap）");
    println!("  ❌ 实现了 Drop 的类型 → 不能是 Copy（有资源需要特殊清理）");
}
