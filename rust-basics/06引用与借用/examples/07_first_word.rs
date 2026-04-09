use colored::*;

// ─────────────────────────────────────────────────────────────────────────
// 综合练习 —— first_word 函数：
//
//   fn first_word(s: &str) -> &str
//
//   这是引用与借用章节的经典综合练习，完美地展示了：
//
//     1. 参数使用 &str：借用字符串数据，不取得所有权
//     2. 返回 &str：返回对输入数据的切片，零拷贝，无需分配新内存
//     3. 返回值生命周期与参数绑定：编译器自动推断（Lifetime Elision）
//     4. 借用规则的实际限制：返回的切片存活期间，原字符串受保护
//
//   本文件提供三种实现版本与多个扩展练习函数：
//
//     first_word_v1  → 字节迭代（as_bytes + enumerate）
//     first_word_v2  → 迭代器写法（split_whitespace + next）
//     first_word_v3  → Unicode 安全版（兼顾中文、日文等）
//     last_word      → 返回最后一个单词
//     nth_word       → 返回第 n 个单词（返回 Option）
//     word_count     → 统计单词数量
//
//   借用规则在本练习中的体现：
//     规则 A：返回的 &str 借用活跃期间，可同时存在其他 &str 借用
//     规则 B：返回的 &str 借用活跃期间，不能对原字符串做可变操作
//     规则 C：返回的 &str 生命周期不能超过原字符串的生命周期
// ─────────────────────────────────────────────────────────────────────────

// ── 版本一：按字节查找空格（经典实现）────────────────────────────────────
fn first_word_v1(s: &str) -> &str {
    let bytes = s.as_bytes();                       // 获取字符串的字节视图 &[u8]，不复制数据
    for (i, &byte) in bytes.iter().enumerate() {    // enumerate() 产生 (字节索引, 字节值)
        if byte == b' ' {                           // b' ' 是空格字节字面量，值为 0x20
            return &s[..i];                         // 找到空格，返回其前的切片
        }
    }
    &s[..]                                          // 没有空格，返回整个字符串的切片
}

// ── 版本二：迭代器写法（更惯用的 Rust 风格）──────────────────────────────
fn first_word_v2(s: &str) -> &str {
    // split_whitespace 按任何 Unicode 空白（空格/制表符/换行符等）分割
    // .next() 取迭代器第一个元素，返回 Option<&str>
    // .unwrap_or("") 当字符串无单词时（空串或纯空格）返回空字符串
    s.split_whitespace().next().unwrap_or("")
}

// ── 版本三：Unicode 安全版（兼顾中文、日文等多字节字符）─────────────────
fn first_word_v3(s: &str) -> &str {
    // split_whitespace 内部按 Unicode 空白分类，正确处理多字节字符
    // unwrap_or(s)：当输入本身无空格时，返回整个字符串而非空串
    s.split_whitespace().next().unwrap_or(s)
    //
    // 【为什么版本一按字节找 b' ' 对 Unicode 也是安全的？】
    //   UTF-8 编码规则：多字节字符（如汉字）的每个字节值均 >= 0x80
    //   ASCII 空格字节值 = 0x20，远低于 0x80
    //   因此 b' ' 不可能出现在汉字等多字节字符的字节序列内部
    //   → 用字节找 ASCII 空格是安全的，不会误切多字节字符
    //   ⚠️  但注意：全角空格（U+3000，字节为 0xE3 0x80 0x80）
    //       用 b' ' 无法识别；split_whitespace 可以正确处理
}

// ── 扩展练习：last_word ──────────────────────────────────────────────────
fn last_word(s: &str) -> &str {
    // .last() 消耗迭代器，返回最后一个元素 Option<&str>
    s.split_whitespace().last().unwrap_or("")       // 无单词时返回空串
}

// ── 扩展练习：nth_word ───────────────────────────────────────────────────
fn nth_word(s: &str, n: usize) -> Option<&str> {
    // .nth(n) 跳过前 n 个元素，返回第 n 个（0-based 索引）
    // 直接返回 Option，让调用方决定如何处理"不存在"的情况
    s.split_whitespace().nth(n)
}

// ── 扩展练习：word_count ─────────────────────────────────────────────────
fn word_count(s: &str) -> usize {
    // .count() 消耗迭代器，统计元素个数
    // split_whitespace 忽略前后空白和连续空白，只统计实际单词数
    s.split_whitespace().count()
}

fn main() {
    println!("{}", "=== 综合练习：first_word ===".green().bold());

    // ─────────────────────────────────────────────────────────────────
    println!("\n1、版本一：as_bytes + enumerate 按字节查找空格");

    let s_en = "hello world";                       // 字符串字面量，类型是 &str
    let s_owned = String::from("rust language");    // 堆上 String
    let s_no_space = "单词";                        // 无空格，应返回整个字符串
    let s_empty = "";                               // 空字符串，应返回空串

    // ✅ 正确：&String 通过 Deref 自动转为 &str，函数参数类型可以统一
    println!("  v1(\"{s_en}\")      = \"{}\"", first_word_v1(s_en));
    println!("  v1(\"{s_owned}\") = \"{}\"（&String 自动 deref 为 &str）",
        first_word_v1(&s_owned));
    println!("  v1(\"{s_no_space}\")        = \"{}\"（无空格 → 返回全串）",
        first_word_v1(s_no_space));
    println!("  v1(\"\")            = \"{}\"（空字符串）", first_word_v1(s_empty));

    // 展示字节迭代的内部过程，帮助理解返回切片的原理
    println!("  内部原理：\"hello world\" 各字节的值");
    let demo = "hello world";
    for (i, byte) in demo.bytes().enumerate() {
        if byte == b' ' {
            println!("    [{}] 0x{:02X} ← 空格！在此截断，返回 &s[..{}]", i, byte, i);
        } else {
            println!("    [{}] 0x{:02X} = '{}'", i, byte, byte as char);
        }
    }
    println!("小结：as_bytes() 返回 &[u8]，按字节迭代定位空格，返回切片不复制数据");

    // ─────────────────────────────────────────────────────────────────
    println!("\n2、版本二：split_whitespace + next（惯用 Rust 风格）");

    let s_tab  = "\t制表符\t分割";                  // 制表符也是空白字符
    let s_nl   = "换行\n分割";                      // 换行符也是空白字符
    let s_only = "   ";                             // 纯空格，无实际单词
    let s_lead = "  前置空格 多个词  ";              // 前后有多余空格

    println!("  v2(\"{s_en}\")       = \"{}\"", first_word_v2(s_en));
    println!("  v2(\"{s_tab}\")  = \"{}\"（制表符分隔）", first_word_v2(s_tab));
    println!("  v2(\"{s_nl}\")    = \"{}\"（换行符分隔）", first_word_v2(s_nl));
    println!("  v2(\"   \")            = \"{}\"（纯空格 → 空串）", first_word_v2(s_only));
    println!("  v2(\"{s_lead}\") = \"{}\"（前置空格被忽略）", first_word_v2(s_lead));
    println!("小结：split_whitespace 自动处理任何 Unicode 空白及连续空白，代码更简洁");

    // ─────────────────────────────────────────────────────────────────
    println!("\n3、版本三：Unicode 安全（中文、日文等多字节字符）");

    let s_zh      = "你好 世界";                    // 中文，每个汉字占 3 字节
    let s_ja      = "こんにちは Rust";              // 日文混合英文
    let s_zh_only = "纯中文无空格";                 // 无空格，应返回整个字符串
    let s_mixed   = "中文english混合 test";          // 中英文混合

    println!("  v3(\"{s_zh}\")     = \"{}\"", first_word_v3(s_zh));
    println!("  v3(\"{s_ja}\") = \"{}\"", first_word_v3(s_ja));
    println!("  v3(\"{s_zh_only}\") = \"{}\"（无空格 → 返回原串）",
        first_word_v3(s_zh_only));
    println!("  v3(\"{s_mixed}\") = \"{}\"", first_word_v3(s_mixed));

    // 展示汉字字节值均 >= 0x80，证明 b' ' 查找不会误匹配
    println!("  为什么 ASCII 空格查找对 Unicode 安全：");
    println!("  \"你好\" 的每个字节：");
    for (i, byte) in "你好".bytes().enumerate() {
        println!("    字节[{i}] = 0x{byte:02X}（>= 0x80，不可能与 ASCII 空格 0x20 混淆）");
    }
    println!("  UTF-8 设计保证：多字节字符的所有字节 >= 0x80，ASCII 字符均 <= 0x7F");
    println!("  ⚠️  全角空格（U+3000）不是 ASCII 空格，v1 无法识别，推荐使用 v2/v3");
    println!("小结：split_whitespace 处理 Unicode 最健壮，字节查找对 ASCII 空格也安全");

    // ─────────────────────────────────────────────────────────────────
    println!("\n4、扩展练习：last_word / nth_word / word_count");

    let sentence = "the quick brown fox jumps";      // 5 个单词

    println!("  句子: \"{}\"", sentence);
    println!("  last_word       = \"{}\"", last_word(sentence));          // "jumps"
    println!("  nth_word(0)     = {:?}", nth_word(sentence, 0));          // Some("the")
    println!("  nth_word(2)     = {:?}", nth_word(sentence, 2));          // Some("brown")
    println!("  nth_word(4)     = {:?}", nth_word(sentence, 4));          // Some("jumps")
    println!("  nth_word(99)    = {:?}（超出范围 → None）",
        nth_word(sentence, 99));
    println!("  word_count      = {}", word_count(sentence));             // 5

    // 边界情况测试
    let empty  = "";
    let spaces = "   ";
    let single = "唯一";
    println!("  last_word(\"\")    = \"{}\"（空串）", last_word(empty));
    println!("  last_word(\"   \") = \"{}\"（纯空格）", last_word(spaces));
    println!("  last_word(\"{single}\")   = \"{}\"（单个词）", last_word(single));
    println!("  nth_word(\"\", 0)  = {:?}（空串无第 0 词）", nth_word(empty, 0));
    println!("  word_count(\"\")   = {}（空串）", word_count(empty));
    println!("  word_count(\"   \")= {}（纯空格）", word_count(spaces));
    println!("小结：split_whitespace 的迭代器组合子让各种词操作都简洁自然");

    // ─────────────────────────────────────────────────────────────────
    println!("\n5、借用规则在实际中的体现（返回 &str 与原字符串的生命周期关系）");

    // ✅ 正确：返回的 &str 与原 String 共享内存，零拷贝
    let owned = String::from("ownership and borrowing");
    let word = first_word_v1(&owned);               // word 是对 owned 内部数据的借用
    println!("  first_word = \"{word}\"");           // word 的最后一次使用（借用在此结束）
    println!("  owned 仍然有效: \"{owned}\"");        // owned 的所有权从未被转移

    // ✅ 正确：先用完借用，再修改原字符串（NLL 保证借用已结束）
    let mut text = String::from("hello world");
    let first = first_word_v1(&text);               // first 借用 text 的内部数据
    println!("  先用完借用: \"{first}\"");           // ← NLL：first 的借用在这行结束
    text.push_str("!!");                             // ✅ first 借用已结束，可以安全修改
    println!("  再修改原字符串: \"{text}\"");

    // ❌ 错误：在 first 仍然存活时调用 bad.clear()（无法编译，已注释）
    // let mut bad = String::from("hello world");
    // let first = first_word_v1(&bad);             // first 借用 bad 的数据
    // bad.clear();                                 // ❌ 编译错误：
    //                                              //    cannot borrow `bad` as mutable
    //                                              //    because it is also borrowed as immutable
    // println!("first = {first}");                 // first 仍在使用中，借用未结束
    //
    // 为什么编译器阻止这种操作？
    //   clear() 会释放堆上的字符串缓冲区（或重置 len 为 0）
    //   如果 clear() 内部触发重新分配，first 的指针将悬空
    //   Rust 在编译期检测到不可变借用与可变操作的冲突，直接拒绝

    println!("  ❌ 借用活跃期间调用 clear()：编译错误，保护 first 不成为悬垂引用");
    println!("小结：Rust 借用检查器确保返回的 &str 始终指向有效数据");

    // ─────────────────────────────────────────────────────────────────
    println!("\n6、生命周期自动推断（Lifetime Elision Rules）");

    println!("  函数签名：fn first_word_v1(s: &str) -> &str");
    println!("  编译器自动补全为：");
    println!("    fn first_word_v1<'a>(s: &'a str) -> &'a str");
    println!("  含义：返回的切片的有效期 ≤ 参数 s 的有效期");
    println!("        调用方的字符串有效，返回的切片就有效");
    println!();
    println!("  生命周期省略规则（Elision Rules）：");
    println!("    规则1：每个引用参数获得独立的生命周期参数");
    println!("    规则2：只有一个引用输入时，其生命周期赋给所有输出引用  ← 本例适用");
    println!("    规则3：有 &self/&mut self 时，self 的生命周期赋给所有输出引用");

    // 演示返回的切片可以继续传给其他接受 &str 的函数
    let phrase = String::from("rust programming language");
    let first  = first_word_v2(&phrase);            // first 生命周期与 phrase 绑定
    let second = nth_word(&phrase, 1);              // second 生命周期与 phrase 绑定
    let total  = word_count(&phrase);               // usize，无引用，无生命周期问题
    println!("  phrase  = \"{phrase}\"");
    println!("  first   = \"{first}\"（&phrase 内部的切片）");
    println!("  second  = {:?}（&phrase 内部的切片）", second);
    println!("  total   = {total}（词数，usize，无生命周期问题）");
    println!("  first 和 second 指向 phrase 的堆内存，phrase 的生命周期覆盖两者");
    println!("小结：编译器自动推断生命周期，单参数函数通常无需手写 'a 注解");

    // ─────────────────────────────────────────────────────────────────
    println!("\n7、综合测试：用各种字符串测试所有函数");

    // 定义结构体存放测试用例（可在函数内定义结构体）
    struct Case {
        input: &'static str,                        // 测试输入（字面量，'static 生命周期）
        desc:  &'static str,                        // 用例描述
    }

    let cases = [
        Case { input: "hello world",         desc: "普通英文" },
        Case { input: "你好 世界",            desc: "中文空格分隔" },
        Case { input: "  leading space",      desc: "前置空格" },
        Case { input: "trailing space  ",     desc: "尾置空格" },
        Case { input: "one",                  desc: "单个词" },
        Case { input: "",                     desc: "空字符串" },
        Case { input: "   ",                  desc: "只有空格" },
        Case { input: "a b c d e",            desc: "多个单词" },
        Case { input: "中文english混合 test",  desc: "混合字符" },
        Case { input: "こんにちは Rust",       desc: "日文+英文" },
    ];

    for case in &cases {
        let v1    = first_word_v1(case.input);      // 字节查找版本
        let v2    = first_word_v2(case.input);      // 迭代器版本
        let last  = last_word(case.input);           // 最后一个词
        let count = word_count(case.input);          // 词数
        println!("  [{}] 输入: \"{}\"", case.desc, case.input);
        println!("    v1=\"{v1}\"  v2=\"{v2}\"  last=\"{last}\"  词数={count}");
    }
    println!("小结：三个版本结果一致，split_whitespace 最简洁健壮，推荐日常使用");

    // ─────────────────────────────────────────────────────────────────
    println!("\n{}", "── 本章综合要点回顾 ──".cyan().bold());
    println!("  ① &str 参数：借用字符串数据，不取得所有权，调用后原变量仍然有效");
    println!("  ② 返回 &str：返回对输入的切片引用，零拷贝，生命周期与输入自动绑定");
    println!("  ③ 借用保护：返回的切片借用活跃期间，编译器阻止可变操作（防悬垂引用）");
    println!("  ④ NLL 精度：借用在最后一次使用后结束（不是在 }} 处），减少假冲突");
    println!("  ⑤ Unicode：split_whitespace 安全处理多字节字符，字节查找 ASCII 空格也安全");
    println!("  ⑥ 省略规则：单引用参数函数的生命周期由编译器自动推断，无需手写 'a");
}