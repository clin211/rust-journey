use colored::*;

fn main() {
    // 基本用法
    let n1: u8 = 20; // 显示类型
    let n2 = 20; // 自动推导类型为 i32
    println!("n1 is {}, n2 is {}", n1, n2);
    // 打印类型
    println!(
        "1、类型获取函数 type_name_of_val，n1 is {}, n2 is {}",
        std::any::type_name_of_val(&n1).green(),
        std::any::type_name_of_val(&n2).green()
    ); // n1 is u8, n2 is i32

    // 类型后缀
    let n3 = 100u16;
    let n4 = 100i16;
    println!(
        "2、在值前加上类型后缀 u16/i16，n3 is {}, n4 is {}",
        std::any::type_name_of_val(&n3),
        std::any::type_name_of_val(&n4)
    ); // n3 is u16, n4 is i16

    // 不同类型运算
    let a: i32 = 10;
    let b: u32 = 5;
    // let c = a + b; // ❌ 编译错误：mismatched types
    let c = a + b as i32; // ✅ 显式转换后才能运算
    println!("3、不同类型运算的时候，需要显式转换后才能运算，c is {}", c);

    // 整型溢出的四种处理方式
    let a = u8::MAX.wrapping_add(1);
    println!(
        "4、整型溢出的四种处理方式一：{}，a 的值为 {:?}, a 的二进制表示 {:08b}",
        "回绕".green(),
        a,
        a
    );
    let b = 200u8.checked_add(1);
    println!(
        "4、整型溢出的四种处理方式二：使用 checked_add 方法，b 的值为 {:?}, is_none = {:?}",
        b,
        b.is_none() // true 表示溢出
    );
    let c = 200u8.overflowing_add(1); // 返回一个元组, 第一个元素是结果，第二个元素是是否溢出
    println!(
        "4、整型溢出的四种处理方式三：使用 overflowing_add 方法，c 的值为 {:?}, is_overflow = {:?}",
        c.0,
        c.1 // true 表示溢出
    );
    let d = u8::MAX.saturating_add(1); // 该类型能表达的最大值
    let e = u8::MIN.saturating_sub(1); // 该类型能表达的最小值
    println!(
        "4、整型溢出的四种处理方式四：使用 saturating_add 方法，d 的值为 {:?}, d 的二进制表示 {:08b}, e 的值为 {:?}, e 的二进制表示 {:08b}",
        d, d, e, e
    );

    // 这里不只是有 _add 方法，在官方文档中还有不少方法在 https://doc.rust-lang.org/std/
}
