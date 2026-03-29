use std::fs;

fn main() {
    let content = fs::read_to_string("example.json").expect("无法读取 example.json");

    for (i, line) in content.lines().enumerate() {
        // {:>2} — 右对齐，占 2 个字符宽度（如 " 1"、"10"），用于行号对齐
        // |    — 固定分隔符
        // {}  — 普通占位符，按顺序填入后续参数
        println!("{:>2} | {}", i + 1, line);
    }
}
