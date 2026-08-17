// fn main() {
//     greet("clin");
//     println!("Hello, world!");
// }

// fn greet(name: &str) {
//     println!("hello {}! welcome to rust world.", name);
// }

// fn main() {
//     let mut count = 0;

//     'continue_up: loop {
//         println!("count={count}");

//         let mut remaining = 10;

//         loop {
//             println!("remaining={remaining}");

//             if remaining == 9 {
//                 break;
//             }

//             if count == 2 {
//                 break 'continue_up;
//             }
//             remaining -= 1
//         }
//         count += 1;
//     }

//     println!("count={count}");
// }

// fn main() {
//     // use while condition
//     let mut x = 10;

//     while x != 0 {
//         println!("x={x}");
//         x -= 1;
//     }
//     println!("执行后的 x 的值为：{x}");
// }

fn main() {
    //     let array = [10, 20, 30, 40, 50];

    //     for element in array {
    //         println!("element is {element}");
    //     }

    //     for element in (1..5).rev() {
    //         println!("element: {element}");
    //     }

    //     // 获取索引和值的循环写法
    //     for (index, value) in array.iter().enumerate() {
    //         println!("index = {index}, value = {value}");
    //     }

    //     let fruits = vec!["Apple", "Banana", "Orange"];

    //     for (index, fruit) in fruits.iter().enumerate() {
    //         println!("index: {}, fruit: {}", index, fruit);
    //     }

    println!("前5项的Fibonacci数：{}", fibonacci(5)); // 1 1 2 3 5
}

// 相互转换摄氏与华氏温度。

// 生成第 n 个斐波那契数。斐波拉契（Fibonacci，又译斐波那契）通常指斐波那契数列及其在数学、自然和金融中的应用。它的前两项是 0 和 1，从第三项开始，每个数字等于前两个数字相加。
fn fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    // let mut i = 1;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    // while i <= n {
    //     let c: usize = a + b;
    //     a = b;
    //     b = c;

    //     i += 1;
    // }

    b
}
