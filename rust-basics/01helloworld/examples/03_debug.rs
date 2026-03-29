use colored::*;

/// 调试格式化输出
/// 1、{:?} — Debug 格式，用于查看数据结构的内部表示
/// 2、{:#?} — Pretty Debug，换行缩进，适合复杂结构
/// 注意：使用 {:?} 的类型必须实现 Debug trait（#[derive(Debug)]）
fn main() {
    // 1、基本类型的 Debug 输出
    println!("{}", "=== 基本类型 ===".green());
    let num = 42;
    let b = true;
    let ch = '中';
    let s = "hello";
    println!("数字：{:?}", num);
    println!("布尔：{:?}", b);
    println!("字符：{:?}", ch);
    println!("字符串：{:?}", s);
    println!();

    // 2、复合类型的 Debug 输出
    println!("{}", "=== 复合类型 ===".green());
    let tuple = (1, "hello", 3.14);
    let arr = [1, 2, 3, 4, 5];
    println!("元组：{:?}", tuple);
    println!("数组：{:?}", arr);
    println!();

    // 3、Option 和 Result 的 Debug 输出
    println!("{}", "=== Option / Result ===".green());
    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;
    let ok_val: Result<i32, &str> = Ok(100);
    let err_val: Result<i32, &str> = Err("出错了");
    println!("Some：{:?}", some_val);
    println!("None：{:?}", none_val);
    println!("Ok：{:?}", ok_val);
    println!("Err：{:?}", err_val);
    println!();

    // 4、{:?} vs {:#?} 对比
    println!("{}", "=== {:?} vs {:#?} ===".green());
    let data = vec![
        ("Alice", 90),
        ("Bob", 85),
        ("Charlie", 95),
    ];
    println!("单行：{:?}", data);
    println!("美化：\n{:#?}", data);
}
