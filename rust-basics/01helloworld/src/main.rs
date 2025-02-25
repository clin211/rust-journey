fn main() {
    println!("Hello, world!");

    let s = "I am a superman.".to_string();
    // 使用 for 循环遍历 1 到 10 的数字, 使用 _i 表示不使用循环变量
    for _i in 1..10 {
        // 使用 &s 创建一个临时的引用
        let tmp_s = &s;
        println!("s is {}", tmp_s);
    }
}
