fn main() {
    // 语法：let 变量名: 类型 = 值;
    let a: i32 = 10;
    println!("a is {}", a);

    let num = 200;
    println!("num is {}", num);

    let num1 = 200.1;
    println!("num2 is {}", num1);

    let name = "rust";
    println!("name is {}", name);

    let is_true = true;
    println!("is_true is {}", is_true);

    let _age = 18; // 如果下文不使用，编译时会报错，可以通过在变量名前面加下划线来消除警告

    // let v: i32;
    // println!("{}", v); // 使用未初始化的变量会报错

    let v;
    v = 200;
    println!("{}", v); // 先声明后初始化，容易使用未初始化的变量，不推荐！！！

    let mut num2 = 200; // 要加上 mut 这个变量才能改变
    println!("before {}", num2);
    num2 = 201;
    println!("after {}", num2);

    // 变量解构
    let (a, b) = (200, true);
    println!("a -> {}, b -> {}", a, b); // a -> 200, b -> true
}
