fn main() {
    // 字符串字面量
    {
        // 普通字符串字面量：类型是 &'static str，编译时嵌入二进制文件
        let s1 = "hello"; // &'static str
        let _s2: &'static str = "world"; // 显式标注

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

        println!("s1 = {}, s3 = {}", s1, s3);
        println!("s4 = {}, s5 = {}, s6 = {}", s4, s5, s6);
    }

    // String 创建方式
    {
        // 从 &str 创建（堆分配，复制数据）
        let s1 = String::from("hello"); // 方法一：From trait
        let s2 = "hello".to_string(); // 方法二：Display trait 的 to_string
        let s3 = "hello".to_owned(); // 方法三：ToOwned trait
        let s4: String = "hello".into(); // 方法四：Into trait

        // 创建空字符串
        let _s5 = String::new(); // 空字符串，""，容量 0
        let _s6 = String::with_capacity(100); // 预分配 100 字节，减少重新分配

        // 从 char 创建
        let s7 = String::from('🦀'); // "🦀"
        let s8 = "abc".repeat(3); // "abcabcabc"

        // 从字节数组创建（必须是合法 UTF-8）
        let bytes = vec![104, 101, 108, 108, 111];
        let s9 = String::from_utf8(bytes).unwrap(); // "hello"

        let bytes = vec![0xff, 0xfe]; // 不是合法 UTF-8
        let result = String::from_utf8(bytes);
        println!("{:?}", result); // Err(FromUtf8Error { bytes: [255, 254], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } })

        println!("s1 = {}, s2 = {}, s3 = {}, s4 = {}", s1, s2, s3, s4);
        println!("s7 = {}, s8 = {}, s9 = {}", s7, s8, s9);
    }
}
