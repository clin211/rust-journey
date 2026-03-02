use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
struct Car {
    name: String,
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "汽车品牌名称 {}", self.name)
    }
}

fn main() {
    println!("Hello, world!");

    let s = "I am a superman.".to_string();
    // 使用 for 循环遍历 1 到 10 的数字, 使用 _i 表示不使用循环变量
    for _i in 1..10 {
        // 使用 &s 创建一个临时的引用
        let tmp_s = &s;
        println!("s is {}", tmp_s);
    }

    // 格式化输出占位符
    println!("{} is a {}", "Rust", "language"); // 使用{}占位符
    println!("{:?} is a {}", "Rust", "language"); // {:?}格式化输出占位符，需要实现Debug trait

    let car = Car {
        name: "Benz".to_string(),
    };

    println!("{}", car);
    println!("{:#?}", car);
}
