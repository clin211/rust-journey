use colored::*;

fn main() {
    // 1. 嵌套循环中的 break 默认作用于最内层
    println!("{}", "=== 嵌套循环中的 break ===".green());
    let mut count = 0;
    'outer: loop {
        println!("--- 外层循环，count = {count} ---");
        let mut remaining = 10;
        loop {
            println!("  内层循环，remaining = {remaining}");

            if remaining == 9 {
                break; // 默认退出最内层循环（'inner）
            }
            if count == 2 {
                break 'outer; // 退出指定标签的循环（'outer）
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("1、最终 count = {count}"); // 1、最终 count = 2

    // 2. continue 也可以用标签
    println!("\n{}", "=== continue + 标签 ===".green());
    println!("2、九九乘法表（跳过包含 5 的行）:");
    'rows: for i in 1..=9 {
        if i == 5 {
            continue 'rows; // 跳过 i=5 的整行
        }
        for j in 1..=i {
            print!("{}×{}={:<3} ", j, i, i * j);
        }
        println!();
    }

    // 3. 综合练习：查找目标值
    println!("\n{}", "=== 综合练习：嵌套查找 ===".green());
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let target = 5;

    'search: for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &val) in row.iter().enumerate() {
            if val == target {
                println!("3、找到 {target}，位置: [{row_idx}][{col_idx}]",);
                break 'search;
            }
        }
    }
}
