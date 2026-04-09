use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 字符串切片 &str
//
//   &str 的本质：胖指针（fat pointer）
//     · 栈上占 2 个 usize 大小：ptr（指针）+ len（字节长度）
//     · ptr → 指向某段合法 UTF-8 字节序列的起始位置
//     · len → 该序列包含的字节数（不是字符数！）
//     · 不拥有数据，不负责 drop，只是对某段内存的"只读借用窗口"
//
//   数据来源可以是：
//     · String 的堆内存（let s = String::from("hi"); let r = &s[..];）
//     · 程序二进制的只读段（let s: &str = "hi"; → &'static str）
//     · 另一个 &str 的子区间（let s = "hello"; let r = &s[1..3];）
//
//   为什么优先用 &str 而非 &String 作为函数参数？
//     &String  只能接受 String（通过 Deref 强制转换变成 &str）
//     &str     能接受 String / 字符串字面量 / &str 切片 → 更通用
//     → Rust 惯用法：字符串参数首选 &str
//
//   UTF-8 警告：
//     字符串切片的索引是字节位置，不是字符（char）序号
//     非 ASCII 字符（如汉字）每个字符占 3 个字节
//     按错误字节边界切片 → 运行时 panic（byte index X is not a char boundary）
//     安全做法：用 char_indices() 找到合法的字节边界再切片
// ─────────────────────────────────────────────────────────────────────────

// ✅ 参数写 &str，比 &String 更通用：String / 字面量 / 切片都可以传入
fn word_count(text: &str) -> usize {
    text.split_whitespace().count() // split_whitespace 跳过连续空白，统计单词数
}

// ✅ 统计字符数（正确方式：chars().count()，不是 .len()）
fn char_count(text: &str) -> usize {
    text.chars().count() // 按 Unicode 标量值计数，处理多字节字符
}

// ✅ 从输入 &str 中截取第一个单词（返回的切片来自参数，生命周期由调用方决定）
fn first_word(s: &str) -> &str {
    // 编译器自动推断生命周期（生命周期省略规则）：
    // fn first_word(s: &str) -> &str
    // 等价于：fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes(); // 以字节视图遍历字符串
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {    // 找到第一个 ASCII 空格字节
            return &s[..i];  // 返回空格前的切片（来自参数 s，非局部变量）
        }
    }
    &s[..] // 没有空格，返回整个字符串的切片
}

// ✅ 按字符数（而非字节数）安全截取前 n 个字符
fn safe_prefix(s: &str, char_count: usize) -> &str {
    // char_indices() 返回 (字节偏移, 字符) 的迭代器，字节偏移是合法的 UTF-8 边界
    match s.char_indices().nth(char_count) {
        Some((byte_idx, _)) => &s[..byte_idx], // byte_idx 是合法字节边界，安全切片
        None => s,                              // 字符总数不足 char_count，返回全部
    }
}

fn main() {
    println!("{}", "=== 字符串切片 &str ===".green().bold());

    println!("\n1、切片的本质：胖指针（ptr + len），不拥有数据");
    // String 是拥有堆内存所有权的智能指针
    let s = String::from("hello world"); // String：栈上控制块 + 堆上字节数据
    // 切片：只是对 s 堆内存某段区间的借用，栈上仅存 ptr 和 len
    let slice: &str = &s[..5]; // ptr → s 堆内存起始, len = 5
    println!("  String（owner）= \"{s}\"");
    println!("  &str（借用者）  = \"{slice}\"");
    println!("  &str 在栈上的布局（示意）：");
    println!("  ┌──────────────────────────────────────────┐");
    println!("  │ ptr: → 指向 s 堆内存中 'h' 的字节地址   │");
    println!("  │ len: 5  (字节数，不是字符数)             │");
    println!("  └──────────────────────────────────────────┘");
    println!("  s 和 slice 共享同一块堆内存，s 负责释放，slice 只是借用");
    println!("  复制一个 &str 只需复制 ptr + len 两个值，开销极小");
    println!("小结：&str 是胖指针，轻量高效，不拥有数据，不负责释放");

    println!("\n2、字符串切片语法：四种范围写法");
    let s = String::from("hello, world!"); // 重新绑定 s（遮蔽上面的 s）
    //                    0123456789012   ← 字节索引
    let a = &s[0..5]; // 字节 0~4，"hello"（完整写法）
    let b = &s[..5];  // 等同于 &s[0..5]，起始索引 0 可省略
    let c = &s[7..];  // 字节 7 到末尾，"world!"（结束索引可省略）
    let d = &s[..];   // 整个字符串，等同于 &s[0..s.len()]
    println!("  s = \"{s}\"");
    println!("  &s[0..5] = \"{a}\"  ← 完整写法，字节 0 到 4（含）");
    println!("  &s[..5]  = \"{b}\"  ← 省略起始索引，从头开始");
    println!("  &s[7..]  = \"{c}\"  ← 省略结束索引，到字符串末尾");
    println!("  &s[..]   = \"{d}\" ← 整个字符串的切片视图");
    println!("  注意：索引是字节位置，两端都必须落在字符边界上");
    println!("小结：范围语法灵活，可省略起始或结束；记住索引是字节不是字符");

    println!("\n3、&str 比 &String 更通用：同一函数接受三种来源");
    let owned = String::from("hello rust world"); // 来源①：String（堆上）
    let literal = "learn rust today";             // 来源②：字面量（只读段）
    let slice_str: &str = &owned[6..];            // 来源③：&str 切片，"rust world"

    // word_count 参数是 &str，三种来源都能直接传入
    println!("  String 的单词数:   {} (\"{owned}\")", word_count(&owned));
    // &owned 是 &String，会通过 Deref 自动转成 &str，再传入函数
    println!("  字面量的单词数:    {} (\"{literal}\")", word_count(literal));
    // &str 字面量直接传，类型完全匹配
    println!("  切片 &str 的单词数: {} (\"{slice_str}\")", word_count(slice_str));
    // &str 切片直接传

    // ❌ 错误：如果参数写 &String，字面量和切片就无法传入
    // fn word_count_bad(text: &String) -> usize { text.split_whitespace().count() }
    // word_count_bad(literal);    // 编译错误：expected &String, found &str
    // word_count_bad(slice_str);  // 编译错误：&str 不能自动转换为 &String

    println!("  Rust 惯用法：字符串参数用 &str，而非 &String");
    println!("小结：&str 接受范围最广，是字符串参数的首选类型");

    println!("\n4、字符串字面量就是 &'static str（存在程序二进制的只读段）");
    // 字面量在编译时嵌入程序二进制文件的 .rodata（只读数据段）
    // 程序运行的整个生命周期内都有效，因此生命周期是 'static
    let s1: &'static str = "I live in the binary!"; // 显式写出 'static
    let s2: &str = "me too, inferred!";              // 'static 可省略，编译器推断
    println!("  s1 = \"{s1}\"");
    println!("  s2 = \"{s2}\"");
    println!("  两者都存储在程序二进制文件中，不在堆上，不需要 String");
    println!("  'static 是 Rust 中最长的生命周期，贯穿整个程序运行期");
    println!("  &'static str 可以安全地赋值给任何较短生命周期的 &str 变量");
    // 'static str 传给普通 &str 参数，完全合法（长生命周期可赋给短）
    let count = word_count(s1);
    println!("  字面量的单词数: {count}");
    println!("小结：字面量是最简单的 &str，零堆分配，整个程序有效");

    println!("\n5、UTF-8 边界问题：按字节切片的危险性");
    let zh = String::from("你好，Rust！"); // 中文 UTF-8 字符串
    // UTF-8 编码规则：ASCII 字符 1 字节，中文字符 3 字节，全角标点 3 字节
    println!("  字符串: \"{}\"", zh);
    println!("  字节数（.len()）:    {}  ← UTF-8 编码的字节总数", zh.len());
    println!("  字符数（.chars().count()）: {}  ← Unicode 标量值数量", char_count(&zh));
    println!("  字节边界详细映射：");
    for (byte_idx, ch) in zh.char_indices() {
        // char_indices() 给出每个字符的字节起始偏移
        println!("    字节偏移 {:2} → 字符 '{}' （占 {} 字节）",
            byte_idx, ch, ch.len_utf8());
    }

    // ❌ 危险：按错误的字节边界切片 → 运行时 panic
    // let bad = &zh[0..1];
    // println!("bad = {bad}");
    // panic 信息：byte index 1 is not a char boundary;
    //             it is inside '你' (bytes 0..3) of `你好，Rust！`
    // 原因：'你' 占字节 0,1,2；索引 1 落在字符内部，不是合法边界

    // ❌ 另一个危险示例：
    // let bad2 = &zh[3..5];
    // panic：byte index 5 is not a char boundary; it is inside '好' (bytes 3..6)

    // ✅ 安全：用 char_indices() 找合法字节边界后再切片
    let prefix_2 = safe_prefix(&zh, 2); // 前 2 个字符："你好"
    let prefix_4 = safe_prefix(&zh, 4); // 前 4 个字符："你好，R"
    println!("  safe_prefix(&zh, 2) = \"{}\"  (前 2 个字符)", prefix_2);
    println!("  safe_prefix(&zh, 4) = \"{}\" (前 4 个字符)", prefix_4);

    // ✅ 另一种安全方式：先收集成 Vec<char>，再按字符索引操作
    let chars: Vec<char> = zh.chars().collect();
    println!("  chars[0] = '{}', chars[1] = '{}'", chars[0], chars[1]);
    println!("小结：非 ASCII 字符串切片要用 char_indices() 找边界，避免运行时 panic");

    println!("\n6、切片借用活跃时不能修改原字符串（借用冲突）");
    // ❌ 错误：切片借用活跃期间，尝试可变借用原字符串
    // let mut data = String::from("rust slices");
    // let first = &data[..4];        // 不可变借用 data（first 借用开始）
    // data.push_str(" are cool");    // 编译错误：cannot borrow `data` as mutable
    //                                // because it is also borrowed as immutable
    // println!("first = {first}");   // first 的借用延伸到这里
    //
    // 为什么 Rust 禁止？
    // push_str 可能触发 String 内部扩容（重新分配堆内存）
    // 扩容后堆地址改变，first 的 ptr 指向已释放的旧内存 → 悬垂切片 → panic
    // Rust 在编译期阻止这种情况，而不是等到运行时崩溃

    // ✅ 正确：先用完切片，再修改原字符串（NLL 会精确判断借用结束点）
    let mut data = String::from("rust slices");
    let first = first_word(&data); // 不可变借用开始
    println!("  先用完切片: first_word = \"{}\"", first); // NLL：first 借用在此结束
    // 上面这行是 first 的最后一次使用，NLL 判定借用区间到此结束
    data.push_str(" are powerful"); // ✅ first 借用已结束，现在可以安全修改
    println!("  再修改 data: \"{}\"", data);
    println!("  关键：NLL（非词法生命周期）让借用在最后一次使用后立即结束");
    println!("小结：先用完不可变切片，再做修改操作；两者不能时间重叠");

    println!("\n7、常见字符串方法：返回 &str 的零拷贝操作");
    let raw = "  Hello, Rust World!\n  Learn borrowing well.  \n  Practice daily.  ";
    println!("  原始文本: {:?}", raw);
    println!();

    // trim()：去除首尾空白字符，返回原字符串内部的切片（零拷贝，无堆分配）
    let trimmed = raw.trim();
    println!("  .trim()               → \"{}\"", &trimmed[..20]); // 只打印前20字节供展示

    // lines()：按行分割，每行是原字符串的子切片（无堆分配）
    println!("  .lines() 逐行输出：");
    for (i, line) in raw.lines().enumerate() {
        println!("    行{}: \"{}\"", i, line.trim()); // 每行再 trim 去首尾空白
    }

    // split_whitespace()：按空白分割（连续空白视为一个分隔符），返回 &str 迭代器
    let words: Vec<&str> = trimmed.split_whitespace().collect();
    println!("  .split_whitespace()   → {:?}", words);

    // split()：按指定字符或字符串分割
    let csv = "apple,banana,cherry,date";
    let fruits: Vec<&str> = csv.split(',').collect(); // 按逗号分割
    println!("  \"{}\".split(',') → {:?}", csv, fruits);

    // starts_with / ends_with：检查前缀/后缀（不分配新字符串）
    let sentence = "Rust is fast and safe";
    println!("  starts_with(\"Rust\")   → {}", sentence.starts_with("Rust"));
    println!("  ends_with(\"safe\")     → {}", sentence.ends_with("safe"));

    // contains()：检查是否包含子串
    println!("  contains(\"fast\")      → {}", sentence.contains("fast"));

    // to_uppercase / to_lowercase：这两个返回新的 String（需要堆分配，因为大小可能变）
    let upper = sentence.to_uppercase();
    println!("  .to_uppercase()       → \"{}\" (新 String，有分配)", upper);

    println!("小结：split/trim/lines/starts_with 等方法返回 &str，零拷贝；");
    println!("      to_uppercase/to_lowercase 等返回 String，需要堆分配");
}