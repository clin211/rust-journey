use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 切片（Slice）的本质：
//
//   切片是对某段连续内存的"只读窗口"（胖指针）
//   栈上存储：ptr（指向数据起始位置） + len（元素个数）
//   不拥有数据，不负责释放，只是"借用"一段范围
//
//   &str  = &[u8] 的视图，但保证是合法 UTF-8
//   &[T]  = 对数组/Vec 的只读视图
//
//   关键用途：让函数可以接受"部分数据"而不需要复制
// ─────────────────────────────────────────────────────────────────────────

fn first_word(s: &str) -> &str {
    // 返回的 &str 的生命周期与参数 s 绑定（编译器自动推断）
    // 意思是：只要 s 还有效，返回的切片就有效
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn sum(nums: &[i32]) -> i32 {
    // &[i32]：接受数组切片或 Vec 切片，不接管所有权
    nums.iter().sum()
}

fn prefix_by_chars(s: &str, n: usize) -> &str {
    // 按字符数（非字节数）安全截取前缀
    if n == 0 {
        return "";
    }
    match s.char_indices().nth(n) {
        Some((idx, _)) => &s[..idx], // idx 是字节边界，安全
        None => s,
    }
}

fn main() {
    println!("{}", "=== 切片（slice） ===".green().bold());

    println!("\n【切片的底层结构】");
    println!("  &str 在栈上是一个胖指针：[ptr, len]");
    println!("  ptr → 指向字符串数据的某个字节位置");
    println!("  len → 切片包含的字节数");
    println!("  不拥有数据，不负责 drop，只是「借了一个窗口」");

    println!("\n1、字符串切片是对字符串某段内存的借用");
    let s = String::from("hello world");
    //                              0123456789...
    let hello = &s[..5]; // 字节 0~4，ptr = s.ptr + 0, len = 5
    let world = &s[6..]; // 字节 6~10，ptr = s.ptr + 6, len = 5
    println!("hello = {hello}, world = {world}");
    println!("hello 和 world 指向 s 内部的不同位置，s 仍然是 owner");
    println!("小结：切片是借用，不会拿走原字符串所有权");

    println!("\n2、为什么函数参数要写 &str 而不是 &String？");
    let word = first_word(&s); // String → &str 自动 deref
    println!("第一个单词: {word}");
    let literal = "rust language"; // 字面量本身就是 &str（存储在 .rodata 段）
    let word2 = first_word(literal);
    println!("字面量的第一个单词: {word2}");
    let slice_word = first_word(&s[6..]); // 切片也是 &str
    println!("切片的第一个单词: {slice_word}");
    println!("小结：&str 参数可接受 String/字面量/切片，&String 只能接受 String");

    println!("\n3、&[T]：数组和 Vec 的通用切片类型");
    let numbers: [i32; 5] = [10, 20, 30, 40, 50]; // 栈上数组
    let part: &[i32] = &numbers[1..4]; // 切片：20, 30, 40
    println!("part = {:?}, sum = {}", part, sum(part));

    let scores: Vec<i32> = vec![60, 70, 80, 90]; // 堆上 Vec
    println!("sum(&scores[1..]) = {}", sum(&scores[1..])); // Vec 切片也是 &[i32]
    println!("小结：集合只读场景优先传 &[T]，比传整个 &Vec<T> 更通用");

    println!("\n4、[重要] 字符串切片索引是字节位置，不是字符序号");
    let chinese = String::from("你好 Rust");
    // 每个汉字在 UTF-8 中占 3 个字节：
    //   你 → 字节 0,1,2
    //   好 → 字节 3,4,5
    //   空格 → 字节 6
    //   R  → 字节 7  ...
    println!("字节数: {}", chinese.len()); // 字节数（不是字符数）
    println!("字符数: {}", chinese.chars().count());
    println!("字节边界: {:?}", chinese.char_indices().collect::<Vec<_>>());

    // ❌ 危险：按错误字节边界切片
    // let bad = &chinese[0..1]; // 编译通过，但运行时 panic：byte index 1 is not a char boundary
    // println!("bad = {bad}");

    // ✅ 安全：按字符边界（char_indices）截取
    let prefix = prefix_by_chars(&chinese, 2); // 前 2 个字符
    println!("前 2 个字符（正确）: {prefix}"); // "你好"
    println!("小结：字符串切片索引是字节位置，非 ASCII 字符要用 char_indices 找边界");

    println!("\n5、[借用冲突] 切片借用活跃时不能可变借用原字符串");
    // ❌ 错误：切片 first 还在用，不能同时修改 title
    // let mut title = String::from("rust slice");
    // let first = &title[..4];  // first 借用了 title 的数据
    // title.push('!');          // 编译错误：cannot borrow `title` as mutable because it is also borrowed as immutable
    // println!("first = {first}");
    // 原因：push 可能触发 Vec 内部重新分配内存，导致 first 的 ptr 失效

    // ✅ 正确：先用完切片，再修改原字符串
    let mut title = String::from("rust slice");
    let first = &title[..4];
    println!("先用完切片 -> {first}"); // first 最后一次使用，借用结束
    title.push('!'); // 此时切片借用已结束，可以安全修改
    println!("再修改原字符串 -> {title}");
    println!("小结：借用和修改必须时间上不重叠");

    println!("\n6、字符串字面量就是 &str（存在程序二进制文件的只读段）");
    let literal: &str = "I'm in the binary!"; // 'static 生命周期
    println!("literal = {literal}");
    println!("字面量的生命周期是 'static，整个程序运行期间都有效");
    println!("这也是为什么 &str 可以在没有 String owner 的情况下存在");
}
