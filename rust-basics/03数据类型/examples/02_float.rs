fn main() {
    // ❌ 错误：0.1 + 0.2 != 0.3
    let x: f64 = 0.1 + 0.2;
    println!(" 0.1 + 0.2 == 0.3 {}, x 的实际值为 {}", x == 0.3, x); // false！x 实际为 0.30000000000000004

    // ❌ 错误：大数精度丢失
    let a: f32 = 16_777_217.0; // 超出 f32 精度范围
    let b: f32 = 16_777_216.0;
    println!("大精度进度丢失：{}", a == b); // true！因为 f32 无法区分这两个数

    // ✅ 正确：用容差比较
    let eps = 1e-9; // 即 10 的 -9 次方，一个很小的浮点数，1e-9 = 0.000000001
    let equal = (x - 0.3).abs() < eps; // abs() 表示取绝对值, 然后和 eps 比较
    println!("用容差比较：{}", equal);

    // ✅ 正确：需要精确计算时用整数
    // 用"分"而非"元"表示金额：100 分 = 1.00 元
    let price_in_cents: i64 = 1999; // 19.99 元
    println!("price in cents: {}", price_in_cents);

    let eps = 1e-9_f64; // 推荐写法
    let eps2: f64 = 0.000_000_001; // 也可以用下划线分隔的可读形式

    println!("浮点数 eps = {}", eps); // 输出: 1e-9
    println!("浮点数 eps = {:.10}", eps); // 输出更清晰的十进制形式
    println!("浮点数 eps2 的值为 {}", eps2);

    // let pos_inf = f64::INFINITY; // 正无穷
    // let neg_inf = f64::NEG_INFINITY; // 负无穷
    let nan = f64::NAN; // Not a Number

    println!("nan == nan {}", nan == nan); // false！NaN 不等于任何值，包括自己
    println!("使用 is_nan 函数检测是否是 NaN {}", nan.is_nan()); // true — 用 is_nan() 检测
    println!("0.0 / 0.0 是 NaN  {}", 0.0 / 0.0); // NaN
    println!(" 1.0 / 0.0 是 正无穷大 {}", 1.0 / 0.0); // inf
}
