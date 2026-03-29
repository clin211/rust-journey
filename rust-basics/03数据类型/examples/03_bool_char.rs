fn main() {
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
    println!("中文字符\"中\"的 Unicode 码点为 {}", c as u32); // 20013（Unicode 码点 U+4E2D）

    // ❌ char 不能直接当 u8
    // let b: u8 = c as u8; // 编译错误：char 范围远超 u8

    // ❌ 中文字符占 3 个字节（UTF-8），不是 1 个
    let s = "中";
    println!("中文字符\"中\"的字节长度 {}", s.len()); // 3（字节数），不是 1！
    println!("中文字符\"中\"的字符数 {}", s.chars().count()); // 1（字符数）

    // ❌ 混淆字节和字符
    let s = "hello🦀";
    println!("s 的字节长度 {}", s.len()); // 9（5 + 4），不是 6
    println!("s 的字符数{}", s.chars().count()); // 6（字符数）
    println!("s 的字节数{}", s.bytes().count()); // 9（字节数）
}
