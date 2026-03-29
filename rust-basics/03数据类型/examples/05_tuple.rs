fn main() {
    // 元组的基本用法
    {
        let tup: (i32, f64, bool) = (500, 6.4, true);
        let (x, y, z) = tup; // 解构
        let first = tup.0; // 索引访问
        let unit = (); // 单元类型

        println!("x = {}, y = {}, z = {}", x, y, z);
        println!("first = {}, unit = {:?}", first, unit);
    }

    // 元组解构注意事项
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
}
