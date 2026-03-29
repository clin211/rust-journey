use colored::*;

fn main() {
    // 基本用法
    let n1: u8 = 20; // 显示类型
    let n2 = 20; // 自动推导类型为 i32
    println!("n1 is {}, n2 is {}", n1, n2);
    // 打印类型
    println!(
        "1、类型获取函数 type_name_of_val，n1 is {}, n2 is {}",
        std::any::type_name_of_val(&n1).green(),
        std::any::type_name_of_val(&n2).green()
    ); // n1 is u8, n2 is i32

    // 类型后缀
    let n3 = 100u16;
    let n4 = 100i16;
    println!(
        "2、在值前加上类型后缀 u16/i16，n3 is {}, n4 is {}",
        std::any::type_name_of_val(&n3),
        std::any::type_name_of_val(&n4)
    ); // n3 is u16, n4 is i16

    // 不同类型运算
    let a: i32 = 10;
    let b: u32 = 5;
    // let c = a + b; // ❌ 编译错误：mismatched types
    let c = a + b as i32; // ✅ 显式转换后才能运算
    println!("3、不同类型运算的时候，需要显式转换后才能运算，c is {}", c);

    // 整型溢出的四种处理方式
    let a = u8::MAX.wrapping_add(1);
    println!(
        "4、整型溢出的四种处理方式一：{}，a 的值为 {:?}, a 的二进制表示 {:08b}",
        "回绕".green(),
        a,
        a
    );
    let b = 200u8.checked_add(1);
    println!(
        "4、整型溢出的四种处理方式二：使用 checked_add 方法，b 的值为 {:?}, is_none = {:?}",
        b,
        b.is_none() // true 表示溢出
    );
    let c = 200u8.overflowing_add(1); // 返回一个元组, 第一个元素是结果，第二个元素是是否溢出
    println!(
        "4、整型溢出的四种处理方式三：使用 overflowing_add 方法，c 的值为 {:?}, is_overflow = {:?}",
        c.0,
        c.1 // true 表示溢出
    );
    let d = u8::MAX.saturating_add(1); // 该类型能表达的最大值
    let e = u8::MIN.saturating_sub(1); // 该类型能表达的最小值
    println!(
        "4、整型溢出的四种处理方式四：使用 saturating_add 方法，d 的值为 {:?}, d 的二进制表示 {:08b}, e 的值为 {:?}, e 的二进制表示 {:08b}",
        d, d, e, e
    );

    // 这里不只是有 _add 方法，在官方文档中还有不少方法在 https://doc.rust-lang.org/std/

    // ❌ 错误：0.1 + 0.2 != 0.3
    let x: f64 = 0.1 + 0.2;
    println!(" 0.1 + 0.2 == 0.3 {}, x 的实际值为 {}", x == 0.3, x); // false！x 实际为 0.30000000000000004

    // ❌ 错误：大数精度丢失
    let a: f32 = 16_777_217.0; // 超出 f32 精度范围
    let b: f32 = 16_777_216.0;
    println!("大精度进度丢失：{}", a == b); // true！因为 f32 无法区分这两个数

    // ✅ 正确：用容差比较
    let eps = 1e-9; // 即 10 的 -9 次方，一个很小的浮点数，1e-9 = 0.000000001
    let equal = (x - 0.3).abs() < eps; // abs() 表示取绝对值, 然后和 eps 比较
    println!("用容差比较：{}", equal);

    // ✅ 正确：需要精确计算时用整数
    // 用"分"而非"元"表示金额：100 分 = 1.00 元
    let price_in_cents: i64 = 1999; // 19.99 元
    println!("price in cents: {}", price_in_cents);

    let eps = 1e-9_f64; // 推荐写法
    let eps2: f64 = 0.000_000_001; // 也可以用下划线分隔的可读形式

    println!("浮点数 eps = {}", eps); // 输出: 1e-9
    println!("浮点数 eps = {:.10}", eps); // 输出更清晰的十进制形式
    println!("浮点数 eps2 的值为 {}", eps2);

    // let pos_inf = f64::INFINITY; // 正无穷
    // let neg_inf = f64::NEG_INFINITY; // 负无穷
    let nan = f64::NAN; // Not a Number

    println!("nan == nan {}", nan == nan); // false！NaN 不等于任何值，包括自己
    println!("使用 is_nan 函数检测是否是 NaN {}", nan.is_nan()); // true — 用 is_nan() 检测
    println!("0.0 / 0.0 是 NaN  {}", 0.0 / 0.0); // NaN
    println!(" 1.0 / 0.0 是 正无穷大 {}", 1.0 / 0.0); // inf

    // ⚠️ bool 不能直接当整数用
    let t: bool = true;
    let x: i32 = t as i32; // 可以，值为 1

    // let y: bool = 1; // ❌ 编译错误：不能把整数当 bool

    println!("x = {}", x);

    let c = 'z';
    let emoji = '🦀';
    let heart = '\u{2764}'; // ❤
    println!("c = {}, emoji = {}, heart = {}", c, emoji, heart);

    let c = '中';
    println!("中文字符“中”的 Unicode 码点为 {}", c as u32); // 20013（Unicode 码点 U+4E2D）

    // ❌ char 不能直接当 u8
    // let b: u8 = c as u8; // 编译错误：char 范围远超 u8

    // ❌ 中文字符占 3 个字节（UTF-8），不是 1 个
    let s = "中";
    println!("中文字符“中”的字节长度 {}", s.len()); // 3（字节数），不是 1！
    println!("中文字符“中”的字符数 {}", s.chars().count()); // 1（字符数）

    // ❌ 混淆字节和字符
    let s = "hello🦀";
    println!("s 的字节长度 {}", s.len()); // 9（5 + 4），不是 6
    println!("s 的字符数{}", s.chars().count()); // 6（字符数）
    println!("s 的字节数{}", s.bytes().count()); // 9（字节数）

    {
        let x: i32 = if true { 42 } else { panic!("不会执行") };
        // else 分支类型是 !，可以转为 i32，编译通过

        let y: &str = match Some("hello") {
            Some(s) => s,
            None => panic!("None!"), // panic! 返回 !，可以转为 &str
        };
        println!("x = {}, y = {}", x, y);
    }

    {
        let unit = (); // 单元类型的唯一值
        println!("{}", unit == ()); // true，因为 () 只有一个值

        // 不写返回值的函数，实际返回的就是 ()
        fn say_hello() {
            // 等价于 fn say_hello() -> ()
            println!("hello");
        }

        let result = say_hello();
        // result 的类型是 ()，值为 ()
        println!("result 的类型是 {}", std::any::type_name_of_val(&result)); // "()"
    }

    {
        // 1. 函数没有显式返回值时，默认返回 ()
        fn print_sum(a: i32, b: i32) {
            // 隐式返回 ()
            println!("'隐式返回 {}", a + b);
        }

        print_sum(1, 2);
    }

    {
        let tup: (i32, f64, bool) = (500, 6.4, true);
        let (x, y, z) = tup; // 解构
        let first = tup.0; // 索引访问
        let unit = (); // 单元类型

        println!("x = {}, y = {}, z = {}", x, y, z);
        println!("first = {}, unit = {:?}", first, unit);
    }

    {
        let tup: (i32, f64, &str) = (42, 3.14, "hello");

        // ❌ 数量不匹配
        // let (a, b) = tup; // 编译错误：expected 3 elements, found 2

        // ✅ 用 _ 忽略不需要的元素
        let (a, _, c) = tup;

        // ❌ 类型不匹配
        // let (a, b, c): (i32, i32, i32) = tup; // 编译错误：类型不匹配

        // ⚠️ 单元素元组必须有逗号
        let single = (5); // 这不是元组！类型是 i32
        let tuple = (5,); // 这才是元组，类型是 (i32,)
        println!("{}", single * 2); // 10 — 说明 single 是 i32，不是元组

        {
            let x: (i32, f64, u8) = (500, 6.4, 1);

            let five_hundred = x.0; // 索引访问元组的第一个元素

            let six_point_four = x.1; // 索引访问元组的第二个元素

            let one = x.2; // 索引访问元组的第三个元素
            println!(
                "five_hundred = {}, six_point_four = {}, one = {}",
                five_hundred, six_point_four, one
            ); // five_hundred = 500, six_point_four = 6.4, one = 1
        }
    }

    {
        // 从数组创建切片
        let arr = [1, 2, 3, 4, 5];
        let slice: &[i32] = &arr; // 完整切片
        let slice: &[i32] = &arr[1..3]; // [2, 3]，左闭右开, 从索引 1 到 3（不含）
        let slice: &[i32] = &arr[2..]; // [3, 4, 5]，从索引 2 到末尾，不包含 2
        let slice: &[i32] = &arr[..3]; // [1, 2, 3]，从开头到索引 3（不含）

        println!("第一个元素：{}", slice[0]); // 索引访问

        // 字符串切片 &str 本质上就是对 [u8] 的封装
        let s: &str = "hello";
        let bytes: &[u8] = s.as_bytes(); // 获取底层字节切片
    }

    {
        fn sum(slice: &[i32]) -> i32 {
            slice.iter().sum()
        }

        let arr = [1, 2, 3];
        let vec = vec![4, 5, 6];
        println!("arr 数组所有元素的和 {}", sum(&arr)); // arr 数组所有元素的和 6
        println!("vec 中所有元素的和 {}", sum(&vec)); // vec 中所有元素的和 15
    }

    {
        let s: &str = "hello";
        println!("&str 大小：{} 字节", std::mem::size_of_val(&s)); // &str 大小：16 字节

        let s = String::from("hello");
        println!("String 大小：{} 字节", std::mem::size_of_val(&s)); // String 大小：24 字节
        println!("String 容量：{}", s.capacity()); // String 容量：5
    }

    {
        // 普通字符串字面量：类型是 &'static str，编译时嵌入二进制文件
        let s1 = "hello"; // &'static str
        let s2: &'static str = "world"; // 显式标注

        // 多行字符串字面量：换行和前导空格都会保留
        let s3 = "第一行
第二行
    第三行（有缩进）";

        // 原始字符串（Raw String）：用 r"..." 包裹，反斜杠不转义
        let s4 = r"C:\Users\forest\file.txt"; // 不需要双写反斜杠
        let s5 = r#"内容包含"双引号"也没关系"#; // 用 # 号界定边界
        let s6 = r##"内容包含"#也能正常工作"##; // 多层 # 号匹配

        // 字节字符串：类型是 &[u8; N]，不是 &str
        let bytes = b"hello"; // 类型：&[u8; 5]
        println!("{}", bytes.len()); // 5

        // ⚠️ 字节字符串只能包含 ASCII，不能用中文
        // let bad = b"你好";                // ❌ 编译错误
    }
    {
        // 从 &str 创建（堆分配，复制数据）
        let s1 = String::from("hello"); // 方法一：From trait
        let s2 = "hello".to_string(); // 方法二：Display trait 的 to_string
        let s3 = "hello".to_owned(); // 方法三：ToOwned trait
        let s4: String = "hello".into(); // 方法四：Into trait

        // 创建空字符串
        let s5 = String::new(); // 空字符串，""，容量 0
        let s6 = String::with_capacity(100); // 预分配 100 字节，减少重新分配

        // 从 char 创建
        let s7 = String::from('🦀'); // "🦀"
        let s8 = "abc".repeat(3); // "abcabcabc"

        // 从字节数组创建（必须是合法 UTF-8）
        let bytes = vec![104, 101, 108, 108, 111];
        let s9 = String::from_utf8(bytes).unwrap(); // "hello"

        let bytes = vec![0xff, 0xfe]; // 不是合法 UTF-8
        let result = String::from_utf8(bytes);
        println!("{:?}", result); // Err(FromUtf8Error { bytes: [255, 254], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } })
    }
}
