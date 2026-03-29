fn main() {
    // Never 类型 (!)：永不返回的表达式
    {
        let x: i32 = if true { 42 } else { panic!("不会执行") };
        // else 分支类型是 !，可以转为 i32，编译通过

        let y: &str = match Some("hello") {
            Some(s) => s,
            None => panic!("None!"), // panic! 返回 !，可以转为 &str
        };
        println!("x = {}, y = {}", x, y);
    }

    // 单元类型 ()
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
}
