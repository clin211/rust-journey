fn main() {
    // 变量遮蔽（shadowing）：用 let 重新声明同名变量，后面的覆盖前面的
    let num3 = 300;
    let num3 = 400;
    println!("num3 -> {}", num3); // num3 -> 400

    // 遮蔽可以改变类型
    let spaces = "   "; // &str
    let spaces = spaces.len(); // usize
    println!("spaces -> {}", spaces); // spaces -> 3

    // 对比：mut 不能改变类型
    // let mut x = 1;
    // x = "hello"; // ❌ 编译错误：类型不匹配
}
