# 数据类型

Rust 是**静态类型**语言，编译时必须知道所有变量的类型。编译器通常能自动推导类型，必要时需显式标注。

数据类型分为两大类：**标量类型（Scalar）** 和 **复合类型（Compound）**。

## 一、标量类型（Scalar）

标量类型代表单个值。

### 1. 整数类型

| 类型 | 大小（位） | 最小值 | 最大值 | 说明 |
|------|-----------|--------|--------|------|
| `i8` | 8 | -128 | 127 | 有符号 8 位 |
| `i16` | 16 | -32_768 | 32_767 | 有符号 16 位 |
| `i32` | 32 | -2^31 | 2^31 - 1 | **默认推导类型** |
| `i64` | 64 | -2^63 | 2^63 - 1 | 有符号 64 位 |
| `i128` | 128 | -2^127 | 2^127 - 1 | 有符号 128 位 |
| `isize` | arch* | — | — | 指针大小，64 位系统为 64 位 |
| `u8` | 8 | 0 | 255 | 无符号 8 位 |
| `u16` | 16 | 0 | 65_535 | 无符号 16 位 |
| `u32` | 32 | 0 | 2^32 - 1 | 无符号 32 位 |
| `u64` | 64 | 0 | 2^64 - 1 | 无符号 64 位 |
| `u128` | 128 | 0 | 2^128 - 1 | 无符号 128 位 |
| `usize` | arch* | 0 | — | 指针大小，常用于索引和长度 |

> **arch\*** 表示大小取决于计算机架构（32 位或 64 位）。

**整数字面量写法：**

| 字面量 | 示例 | 说明 |
|--------|------|------|
| 十进制 | `98_222` | 下划线提高可读性 |
| 十六进制 | `0xff` | `0x` 前缀 |
| 八进制 | `0o77` | `0o` 前缀 |
| 二进制 | `0b1111_0000` | `0b` 前缀 |
| 字节 | `b'A'` | 仅限 `u8` |

**类型后缀示例：** `57u8`、`1000i64`、`0xffu32`

#### 基本用法

```rust
// 显式类型标注 vs 自动推导
let n1: u8 = 20;  // 显式类型
let n2 = 20;       // 自动推导类型为 i32

println!("n1 is {}, n2 is {}", n1, n2);

// 打印变量的实际类型（使用 type_name_of_val 函数）
println!(
    "n1 的类型是 {}, n2 的类型是 {}",
    std::any::type_name_of_val(&n1),  // "u8"
    std::any::type_name_of_val(&n2)   // "i32"
);
```

```rust
// 在值后面加上类型后缀来指定类型
let n3 = 100u16;  // u16 类型
let n4 = 100i16;  // i16 类型
println!(
    "n3 的类型是 {}, n4 的类型是 {}",
    std::any::type_name_of_val(&n3),  // "u16"
    std::any::type_name_of_val(&n4)   // "i16"
);
```

#### 坑：不同整数类型不能直接运算

Rust 不做隐式转换，哪怕 `i32` 和 `u32` 都是 32 位整数。

```rust
let a: i32 = 10;
let b: u32 = 5;
// let c = a + b; // ❌ 编译错误：mismatched types
let c = a + b as i32; // ✅ 显式转换后才能运算
println!("c is {}", c); // 15

// ⚠️ 注意整数除法直接截断，不是四舍五入
let m: i32 = 5;
let n: i32 = 2;
println!("{}", m / n);               // 2（整数除法）
println!("{}", m as f64 / n as f64); // 2.5（浮点除法）
```

#### 坑：整数溢出 — Debug vs Release 行为不一致

整数溢出是最常见的隐蔽 bug：开发和生产环境行为不同。

```rust
// 以下每个示例独立演示各方法在 u8::MAX (255) 上的行为：

// 方法一：回绕（wrapping）— 溢出后从最小值重新开始
let a = 255u8.wrapping_add(1);       // 255 + 1 回绕为 0

// 方法二：checked — 溢出时返回 None
let b = 255u8.checked_add(1);        // None（不安全，拒绝计算）

// 方法三：saturating — 饱和在最大值或最小值
let c = 255u8.saturating_add(1);     // 255（不回绕，停在最大值）

// 方法四：overflowing — 返回一个元组 (结果, 是否溢出)
let d = 255u8.overflowing_add(1);    // (0, true)

// ⚠️ 绝不要用 x = x + 1 处理可能溢出的场景：
// Debug 模式：255u8 + 1 会 panic（arithmetic overflow）
// Release 模式：255u8 + 1 静默回绕为 0，不会报错！
// 行为取决于编译模式，非常危险！
```

**四个溢出处理方法对比：**

| 方法 | `255u8 + 1` 结果 | `200u8 + 1` 结果 | 说明 |
|------|-------------------|-------------------|------|
| `wrapping_add` | `0` | `201` | 回绕，始终安全 |
| `checked_add` | `None` | `Some(201)` | 返回 Option，安全检查 |
| `saturating_add` | `255` | `201` | 饱和在最大/最小值 |
| `overflowing_add` | `(0, true)` | `(201, false)` | 同时返回结果和溢出标志 |

**实际运行示例：**

```rust
// 方式一：回绕（wrapping）— 溢出后从最小值重新开始
let a = u8::MAX.wrapping_add(1); // u8::MAX = 255, 255 + 1 回绕为 0
println!("a 的值为 {:?}, 二进制表示 {:08b}", a, a); // 0, 00000000

// 方式二：checked — 溢出时返回 None
let b = 200u8.checked_add(1);
println!("b 的值为 {:?}, is_none = {:?}", b, b.is_none()); // Some(201), false

// 方式三：overflowing — 返回 (结果, 是否溢出)
let c = 200u8.overflowing_add(1);
println!("c 的值为 {:?}, is_overflow = {:?}", c.0, c.1); // 201, false

// 方式四：saturating — 饱和在该类型的最大值或最小值
let d = u8::MAX.saturating_add(1); // 255 + 1 = 255（饱和在最大值）
let e = u8::MIN.saturating_sub(1); // 0 - 1 = 0（饱和在最小值）
println!("d = {:?} ({:08b}), e = {:?} ({:08b})", d, d, e, e); // 255, 0
```

> 不仅有 `_add` 方法，官方文档还有更多方法：<https://doc.rust-lang.org/std/>

#### 坑：`isize` / `usize` 的可移植性

```rust
// ⚠️ usize 在 32 位系统上是 32 位，64 位系统上是 64 位
let huge: u64 = 5_000_000_000; // 约 50 亿
// let idx: usize = huge as usize; // 32 位系统上会截断！

// ✅ 安全做法：用 try_into
let idx: Result<usize, _> = huge.try_into();
match idx {
    Ok(v) => println!("索引：{}", v),
    Err(_) => println!("值太大，不适合当前架构的 usize"),
}
```

#### 坑：整数推导默认是 i32

```rust
let a = 2_147_483_647; // i32 最大值
// let b = a + 1; // ⚠️ Debug 模式 panic！需要更大类型应显式标注
let b: i64 = 2_147_483_648; // ✅ 用 i64
```

### 2. 浮点类型

| 类型 | 大小（位） | 精度 | 说明 |
|------|-----------|------|------|
| `f32` | 32 | ~6-7 位有效数字 | 单精度，IEEE-754 |
| `f64` | 64 | ~15-16 位有效数字 | **默认推导类型**，双精度 |

#### 基本用法

```rust
let x = 2.0;      // f64（默认）
let y: f32 = 3.0; // f32
```

#### 坑：浮点数不能直接用 `==` 比较

浮点数精度丢失是所有语言的通病，Rust 也不例外。

```rust
// ❌ 错误：0.1 + 0.2 != 0.3
let x: f64 = 0.1 + 0.2;
println!("0.1 + 0.2 == 0.3 {}, x 的实际值为 {}", x == 0.3, x);
// false！x 实际为 0.30000000000000004

// ❌ 错误：大数精度丢失
let a: f32 = 16_777_217.0; // 超出 f32 精度范围
let b: f32 = 16_777_216.0;
println!("大精度精度丢失：{}", a == b); // true！因为 f32 无法区分这两个数
```

**正确做法：**

```rust
// ✅ 正确：用容差比较
let eps = 1e-9; // 即 10 的 -9 次方，一个很小的浮点数
let equal = (x - 0.3).abs() < eps; // abs() 取绝对值，然后和 eps 比较
println!("用容差比较：{}", equal); // true

// ✅ 正确：需要精确计算时用整数
// 用"分"而非"元"表示金额：100 分 = 1.00 元
let price_in_cents: i64 = 1999; // 19.99 元
println!("price in cents: {}", price_in_cents);
```

#### 浮点数书写方式

```rust
let eps = 1e-9_f64;           // 推荐写法：科学计数法 + 类型后缀
let eps2: f64 = 0.000_000_001; // 也可以用下划线分隔的可读形式

println!("浮点数 eps = {}", eps);        // 输出: 1e-9
println!("浮点数 eps = {:.10}", eps);    // 输出更清晰的十进制形式
println!("浮点数 eps2 的值为 {}", eps2);  // 输出: 0.000000001
```

#### 特殊浮点值

```rust
let pos_inf = f64::INFINITY;     // 正无穷
let neg_inf = f64::NEG_INFINITY; // 负无穷
let nan = f64::NAN;              // Not a Number

println!("nan == nan {}", nan == nan);      // false！NaN 不等于任何值，包括自己
println!("使用 is_nan 函数检测是否是 NaN {}", nan.is_nan()); // true — 用 is_nan() 检测
println!("0.0 / 0.0 是 NaN {}", 0.0 / 0.0);   // NaN
println!("1.0 / 0.0 是正无穷大 {}", 1.0 / 0.0); // inf
```

### 3. 布尔类型

| 类型 | 大小 | 值 | 说明 |
|------|------|-----|------|
| `bool` | 1 字节 | `true` / `false` | 条件判断的基础类型 |

```rust
let t = true;
let f: bool = false;

// ⚠️ bool 不能直接当整数用
let x: i32 = t as i32; // 可以，值为 1
// let y: bool = 1;    // ❌ 编译错误：不能把整数当 bool
```

### 4. 字符类型

| 类型 | 大小 | 范围 | 说明 |
|------|------|------|------|
| `char` | 4 字节 | Unicode 标量值 | U+0000 ~ U+D7FF, U+E000 ~ U+10FFFF |

#### 基本用法

```rust
let c = 'z';
let emoji = '🦀';
let heart = '\u{2764}'; // ❤
```

> `char` 是 Unicode 标量值，不同于 `u8` 字节。String 内部是 UTF-8 编码的字节序列，不等于 `char` 数组。

#### 坑：`char` 不等于字符串，也不等于 `u8`

```rust
let c = '中';
println!("{}", c as u32); // 20013（Unicode 码点 U+4E2D）

// ⚠️ char as u8 可以编译，但会静默截断（只保留低 8 位）！
let b: u8 = '中' as u8;
println!("{}", b); // 45，Unicode 码点 U+4E2D 的低 8 位是 0x2D = 45，数据丢失！

// ❌ 中文字符占 3 个字节（UTF-8），不是 1 个
let s = "中";
println!("{}", s.len());           // 3（字节数），不是 1！
println!("{}", s.chars().count()); // 1（字符数）

// ❌ 混淆字节和字符
let s = "hello🦀";
println!("{}", s.len());           // 9（5 + 4），不是 6
println!("{}", s.chars().count()); // 6（字符数）
println!("{}", s.bytes().count()); // 9（字节数）
```

### 5. Never 类型（`!`）

`!` 是一种特殊的标量类型，表示**永远不会返回值**。也称为"never type"。

| 特性 | 说明 |
|------|------|
| 类型表示 | `!` |
| 值 | **不存在任何值** |
| 大小 | 0 字节（不可能被实例化） |
| 用途 | 表示发散（divergent）的控制流 |

#### 基本用法

`!` 类型的表达式永远不会正常结束执行，因此 `!` 可以**强制转换为任意类型**：

```rust
// 这些表达式返回类型是 !（永远不会返回）
panic!("程序崩溃了");     // 永远不会返回
std::process::exit(1);    // 永远不会返回
loop {};                  // 永远不会返回
```

#### `!` 可以转为任意类型

因为 `!` 类型的表达式永远不会执行到返回值那一步，所以它可以被当作任何类型使用：

```rust
let x: i32 = if true { 42 } else { panic!("不会执行") };
// else 分支类型是 !，可以转为 i32，编译通过

let y: &str = match Some("hello") {
    Some(s) => s,
    None => panic!("None!"), // panic! 返回 !，可以转为 &str
};
```

#### 常见产生 `!` 类型的表达式

| 表达式 | 说明 |
|--------|------|
| `panic!("...")` | 立即终止程序 |
| `std::process::exit(code)` | 退出进程 |
| `loop {}` | 无限循环（不带 break） |
| `unimplemented!()` | 占位宏，调用即 panic |
| `todo!()` | 类似 `unimplemented!()`，表示待实现 |

#### 坑：`!` 类型目前仍是不完整特性（unstable）

```rust
// ❌ 不能显式声明变量为 ! 类型
// let x: ! = panic!("oops"); // 编译错误：`!` 类型不能直接使用

// ✅ 但在类型推导中，! 会自动被推导出来
fn always_panic() -> ! {
    panic!("永远会 panic");
}

// ✅ 函数返回 ! 时，调用者可以在任何需要类型的地方使用它
fn get_or_panic(opt: Option<i32>) -> i32 {
    match opt {
        Some(v) => v,
        None => always_panic(), // 返回 !，自动转为 i32
    }
}
```

### 6. 单元类型（`()`）

| 特性 | 说明 |
|------|------|
| 类型 | `()` |
| 值 | 只有一个值：`()` |
| 大小 | 0 字节 |
| 用途 | 表示"没有有意义的值" |

#### 基本用法

```rust
let unit = ();                // 单元类型的唯一值
println!("{}", unit == ());   // true，因为 () 只有一个值

// 不写返回值的函数，实际返回的就是 ()
fn say_hello() {              // 等价于 fn say_hello() -> ()
    println!("hello");
}

let result = say_hello();
// result 的类型是 ()，值为 ()
println!("result 的类型是 {}", std::any::type_name_of_val(&result)); // "()"
```

#### 单元类型的常见场景

```rust
// 1. 函数没有显式返回值时，默认返回 ()
fn print_sum(a: i32, b: i32) {  // 隐式返回 ()
    println!("{}", a + b);
}

// 2. if/else 表达式中不需要返回值时，各分支返回 ()
if true {
    println!("yes");
} else {
    println!("no");
}
// 整个 if 表达式的类型是 ()

// 3. 用 () 作为 Vec 的元素类型，表示"不需要值，只关心数量"
let actions: Vec<()> = vec![(); 5];
println!("actions 数量：{}", actions.len()); // 5

// 4. 元组解构时忽略不需要的部分
let (name, _) = ("Alice", 42);  // _ 是通配符，忽略第二个值（不绑定，也不触发 Drop）
```

## 二、复合类型（Compound）

复合类型将多个值组合成一个类型。

### 1. 元组（Tuple）

| 特性 | 说明 |
|------|------|
| 长度 | **固定**，声明后不可变 |
| 类型 | 各元素可以是**不同类型** |
| 访问方式 | 解构或索引（`.0`、`.1`） |
| 内存 | 栈上分配 |

#### 基本用法

```rust
let tup: (i32, f64, bool) = (500, 6.4, true);
let (x, y, z) = tup;         // 解构
let first = tup.0;            // 索引访问
let unit = ();                // 单元类型，详见"单元类型"章节
```

#### 坑：元组解构时类型和数量必须完全匹配

```rust
let tup: (i32, f64, &str) = (42, 3.14, "hello");

// ❌ 数量不匹配
// let (a, b) = tup; // 编译错误：expected 3 elements, found 2

// ✅ 用 _ 忽略不需要的元素
let (a, _, c) = tup;

// ❌ 类型不匹配
// let (a, b, c): (i32, i32, i32) = tup; // 编译错误：类型不匹配

// ⚠️ 单元素元组必须有逗号
let single = (5);     // 这不是元组！类型是 i32
let tuple = (5,);     // 这才是元组，类型是 (i32,)
println!("{}", single * 2); // 10 — 说明 single 是 i32，不是元组
```

### 2. 数组（Array）

| 特性 | 说明 |
|------|------|
| 长度 | **固定**，编译时确定 |
| 类型 | 所有元素必须是**相同类型** |
| 内存 | 栈上分配 |
| 访问方式 | 索引（`arr[0]`） |

#### 基本用法

```rust
let arr = [1, 2, 3, 4, 5];           // 类型：[i32; 5]
let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 显式标注
let arr = [3; 5];                     // [3, 3, 3, 3, 3]，初始化为相同值
let first = arr[0];                   // 索引访问
```

#### 坑：数组越界 — 编译通过但运行时 panic

索引必须是 `usize` 类型，且越界会导致程序崩溃。

```rust
let arr = [1, 2, 3];

// ❌ 索引越界：编译通过，运行时 panic
// let x = arr[5]; // panic: index out of bounds: the len is 3 but the index is 5

// ❌ 非法索引类型
// let idx: i32 = 0;
// let x = arr[idx]; // 编译错误：usize expected, got i32

// ✅ 安全做法：用 .get() 返回 Option
match arr.get(5) {
    Some(v) => println!("值为：{}", v),
    None => println!("索引越界"), // 走这里
}

// ✅ 简写：unwrap_or 提供默认值
let val = arr.get(5).copied().unwrap_or(-1); // -1
```

### 3. 切片（Slice）

切片是对一个连续元素序列的**引用**，不拥有所有权。

| 特性 | 说明 |
|------|------|
| 长度 | **动态**，运行时确定 |
| 类型 | 所有元素必须是**相同类型** |
| 内存 | 引用 underlying 数据，不分配新内存 |
| 访问方式 | 索引（`s[0]`） |
| 类型写法 | `&[T]`（不可变切片）、`&mut [T]`（可变切片） |

#### 基本用法

```rust
// 从数组创建切片
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr;          // 完整切片
let slice: &[i32] = &arr[1..3];    // [2, 3]，左闭右开
let slice: &[i32] = &arr[2..];     // [3, 4, 5]，从索引 2 到末尾
let slice: &[i32] = &arr[..3];     // [1, 2, 3]，从开头到索引 3（不含）

println!("第一个元素：{}", slice[0]); // 索引访问

// 字符串切片 &str 本质上就是对 [u8] 的封装
let s: &str = "hello";
let bytes: &[u8] = s.as_bytes();   // 获取底层字节切片
```

#### 数组 vs 切片

| 特性 | 数组 `[T; N]` | 切片 `&[T]` |
|------|---------------|-------------|
| 长度 | 编译时固定 | 运行时确定 |
| 大小 | 编译时已知 | 胖指针（指针 + 长度） |
| 所有权 | 拥有数据 | 借用数据 |
| 函数参数 | 限固定长度 | 通用，推荐 |

```rust
// ✅ 用切片做函数参数，可以接受数组和 Vec 的引用
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

let arr = [1, 2, 3];
let vec = vec![4, 5, 6];
println!("{}", sum(&arr));  // 6
println!("{}", sum(&vec));  // 15
```

#### 坑：切片索引越界同样会 panic

```rust
let arr = [1, 2, 3];
let slice = &arr[0..2]; // [1, 2]

// ❌ 索引越界：运行时 panic
// let x = slice[5]; // panic: index out of bounds

// ✅ 安全做法：用 .get() 返回 Option
match slice.get(5) {
    Some(v) => println!("值为：{}", v),
    None => println!("索引越界"),
}
```

### 4. 字符串类型

Rust 中的字符串比大多数语言复杂，核心在于区分**所有权**和**编码方式**。

Rust 标准库中有多种字符串相关类型，日常主要使用 `&str` 和 `String`：

| 类型 | 所有权 | 编码 | 可变性 | 用途 |
|------|--------|------|--------|------|
| `&str` | 借用 | UTF-8 | 不可变 | 函数参数、字面量、只读访问 |
| `String` | 拥有 | UTF-8 | 可变 | 构建、修改、拥有字符串 |
| `OsStr` / `OsString` | — | 平台相关 | — | 操作系统 API 交互 |
| `CStr` / `CString` | — | C 兼容（NUL 终止） | — | FFI / C 语言交互 |
| `Path` / `PathBuf` | — | 平台相关 | — | 文件路径操作 |

> `OsStr/OsString`、`CStr/CString`、`Path/PathBuf` 仅做了解，后续章节详解。本节重点讲解 `&str` 和 `String`。

#### 4.1 `&str` vs `String`

| 特性 | `&str`（字符串切片） | `String`（堆分配字符串） |
|------|---------------------|------------------------|
| 所有权 | 借用（不拥有数据） | 拥有数据 |
| 内存位置 | 可以在栈上、堆上或静态区 | 堆上 |
| 可变性 | 不可变（`&mut str` 极少使用） | 可变 |
| 大小 | 胖指针（指针 + 长度），16 字节 | 胖指针（指针 + 长度 + 容量），24 字节 |
| 创建方式 | `"hello"` 字面量 | `String::from()`、`.to_string()` 等 |
| 典型用途 | 函数参数、只读访问 | 需要拥有/修改/构建字符串 |

**内部表示：**

```sh
&str（胖指针，16 字节）         String（Vec<u8> 封装，24 字节）
┌─────────┬──────┐             ┌─────────┬──────┬──────────┐
│ ptr     │ len  │             │ ptr     │ len  │ capacity │
│ (8字节) │(8字节)│             │ (8字节)  │(8字节)│ (8字节)  │
└────┬────┴──────┘             └────┬────┴──────┴──────────┘
     │                              │
     ▼                              ▼
   [h][e][l][l][o]               [h][e][l][l][o][ ][ ][...]  ← 堆上可能多分配
   （可能在静态区/栈上/堆上）       capacity ≥ len，预留扩容空间
```

```rust
let s: &str = "hello";
println!("&str 大小：{} 字节", std::mem::size_of_val(&s)); // 16

let s = String::from("hello");
println!("String 大小：{} 字节", std::mem::size_of_val(&s)); // 24
println!("String 容量：{}", s.capacity()); // 至少 5
```

#### 4.2 字符串字面量（`&str`）

```rust
// 普通字符串字面量：类型是 &'static str，编译时嵌入二进制文件
let s1 = "hello";                    // &'static str
let s2: &'static str = "world";     // 显式标注

// 多行字符串字面量：换行和前导空格都会保留
let s3 = "第一行
第二行
    第三行（有缩进）";

// 原始字符串（Raw String）：用 r"..." 包裹，反斜杠不转义
let s4 = r"C:\Users\forest\file.txt";      // 不需要双写反斜杠
let s5 = r#"内容包含"双引号"也没关系"#;      // 用 # 号界定边界
let s6 = r##"内容包含"#也能正常工作"##;      // 多层 # 号匹配

// 字节字符串：类型是 &[u8; N]，不是 &str
let bytes = b"hello";                // 类型：&[u8; 5]
println!("{}", bytes.len());         // 5
// ⚠️ 字节字符串只能包含 ASCII，不能用中文
// let bad = b"你好";                // ❌ 编译错误
```

#### 4.3 String 创建

```rust
// 从 &str 创建（堆分配，复制数据）
let s1 = String::from("hello");      // 方法一：From trait
let s2 = "hello".to_string();        // 方法二：Display trait 的 to_string
let s3 = "hello".to_owned();         // 方法三：ToOwned trait
let s4: String = "hello".into();     // 方法四：Into trait

// 创建空字符串
let s5 = String::new();              // 空字符串，""，容量 0
let s6 = String::with_capacity(100); // 预分配 100 字节，减少重新分配

// 从 char 创建
let s7 = String::from('🦀');         // "🦀"
let s8 = "abc".repeat(3);            // "abcabcabc"

// 从字节数组创建（必须是合法 UTF-8）
let bytes = vec![104, 101, 108, 108, 111];
let s9 = String::from_utf8(bytes).unwrap(); // "hello"

let bytes = vec![0xff, 0xfe];        // 不是合法 UTF-8
let result = String::from_utf8(bytes);
println!("{:?}", result);            // Err(FromUtf8Error { ... })

// 从 UTF-8 字节数组创建（忽略有效性检查，unsafe）
// let s10 = unsafe { String::from_utf8_unchecked(vec![0xff]) }; // ⚠️ 危险
```

#### 4.4 String 修改

```rust
let mut s = String::from("hello");

// 追加（在末尾添加）
s.push_str(" world");        // 追加 &str
s.push('!');                  // 追加 char
println!("{}", s);            // "hello world!"

// 插入（在指定字节位置，必须是合法 UTF-8 边界）
s.insert(5, ',');             // 在字节位置 5 插入 ','
println!("{}", s);            // "hello, world!"

// 替换
let s2 = "I like Rust".replace("like", "love"); // "I love Rust"
let s3 = "aaa".replacen('a', "b", 2);           // "bba"（只替换前 2 个）

// 删除
s.remove(5);                  // 删除字节位置 5 的字符（char），返回该 char
let popped = s.pop();         // 删除并返回最后一个 char，返回 Option<char>
s.truncate(5);                // 截断到前 5 个字节
s.clear();                    // 清空字符串，变成 ""

// ⚠️ 所有修改操作中，insert/remove 的位置必须是合法 UTF-8 字符边界
// let mut s = String::from("你好");
// s.remove(1);               // ❌ panic: byte index 1 is not a char boundary
```

#### 4.5 字符串拼接

```rust
// 方法一：+ 运算符（注意所有权移动）
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = s1 + &s2;            // s1 被移动，之后不可用！s2 仍可用
// println!("{}", s1);        // ❌ 编译错误：s1 已被移动
println!("{}", s2);            // ✅ s2 是引用，没被移动

// 方法二：format! 宏（推荐，不移动所有权）
let s4 = format!("{}-{}-{}", "a", "b", "c"); // "a-b-c"

// 方法三：push_str / push（追加到已有 String）
let mut s5 = String::from("hello");
s5.push_str(" world");

// 方法四：join（用分隔符连接字符串切片）
let parts = vec!["hello", "world", "rust"];
let s6 = parts.join(", ");    // "hello, world, rust"

// 方法五：concat（直接连接）
let s7 = ["a", "b", "c"].concat(); // "abc"

// ⚠️ 连续拼接时优先用 push_str，比 + 或 format! 性能更好
let mut result = String::with_capacity(100);
for word in ["hello", " ", "world", "!"] {
    result.push_str(word);    // ✅ 每次只追加，不重新分配
}
```

#### 4.6 字符串遍历与访问

```rust
let s = "你好Rust";

// ⚠️ 字符串不能用索引访问
// let c = s[0];               // ❌ 编译错误：String/&str 不支持索引

// ✅ 按字符遍历（char，Unicode 标量值）
for c in s.chars() {
    print!("[{}] ", c);         // [你] [好] [R] [u] [s] [t]
}
println!();
println!("字符数：{}", s.chars().count()); // 6

// ✅ 按字节遍历
for b in s.bytes() {
    print!("{:02x} ", b);       // e4 bd a0 e5 a5 bd 52 75 73 74
}
println!();
println!("字节数：{}", s.len());          // 12（2×3 + 4×1）

// ✅ 按 (字节位置, 字符) 遍历（处理中文时非常有用）
for (pos, c) in s.char_indices() {
    println!("字节 {} 处是 '{}'", pos, c);
}
// 字节 0 处是 '你'
// 字节 3 处是 '好'
// 字节 6 处是 'R'
// ...

// ✅ 按行遍历
let text = "第一行\n第二行\n第三行";
for line in text.lines() {
    println!("{}", line);
}

// ✅ 按分隔符拆分
let parts: Vec<&str> = "a,b,c".split(',').collect();       // ["a", "b", "c"]
let words: Vec<&str> = "hello  world".split_whitespace().collect(); // ["hello", "world"]
```

#### 4.7 字符串切片

```rust
let s = "你好Rust";

// ⚠️ 字符串切片的索引是**字节位置**，必须在合法 UTF-8 字符边界
let s1 = &s[0..3];              // ✅ "你"（中文字符占 3 字节）
let s2 = &s[6..10];             // ✅ "Rust"（ASCII 各 1 字节）
// let s3 = &s[0..2];           // ❌ panic: byte index 2 is not a char boundary
// let s4 = &s[1..4];           // ❌ panic: 在"你"的中间切割

// ✅ 安全做法：用 char_indices 获取正确边界
let s = "hello你好";
let mut boundaries: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
boundaries.push(s.len());       // 加上末尾边界
// boundaries = [0, 1, 2, 3, 4, 5, 8]
// 可以安全地在这些位置切片

// ✅ 用 get 切片，越界不会 panic
match s.get(0..3) {
    Some(slice) => println!("{}", slice),
    None => println!("切片失败"),
}
```

#### 4.8 `&str` 与 `String` 互转

```rust
// String -> &str（零成本借用，不复制数据）
let s = String::from("hello");
let slice1: &str = &s;          // 方式一：Deref 自动转换
let slice2: &str = s.as_str();  // 方式二：显式调用

// &str -> String（需要堆分配，复制数据）
let s1 = "hello".to_string();   // 方式一
let s2 = String::from("hello"); // 方式二
let s3 = "hello".to_owned();    // 方式三
let s4: String = "hello".into();// 方式四

// ⚠️ 函数参数优先用 &str（更通用）
fn greet(name: &str) {          // ✅ 可以接受 &str 和 &String
    println!("Hello, {}!", name);
}
greet("Alice");                 // 传 &str 字面量
greet(&String::from("Bob"));    // 传 &String，Deref 自动转 &str

// ⚠️ 返回值用 String（拥有数据，生命周期不受限）
fn make_greeting(name: &str) -> String {
    format!("Hello, {}!", name) // String 拥有数据，可以安全返回
}
```

#### 4.9 字符串常用操作

```rust
// 查找
let s = "Hello, Rust!";
println!("{}", s.contains("Rust"));     // true
println!("{}", s.starts_with("Hello")); // true
println!("{}", s.ends_with('!'));       // true
println!("{:?}", s.find(','));          // Some(5) — 返回字节位置
println!("{:?}", s.match_indices("l")); // 所有匹配位置

// 截取与修剪
let s = "  hello world  ";
println!("{}", s.trim());               // "hello world"（去两端空白）
println!("{}", s.trim_start());         // "hello world  "（去左边）
println!("{}", s.trim_end());           // "  hello world"（去右边）

// 大小写转换（对 ASCII 有效，Unicode 可能部分有效）
println!("{}", "hello".to_uppercase()); // "HELLO"
println!("{}", "HELLO".to_lowercase()); // "hello"

// 重复
println!("{}", "abc".repeat(3));        // "abcabcabc"

// 反转（按字符反转，不是字节）
println!("{}", "hello".chars().rev().collect::<String>()); // "olleh"

// 判断
println!("{}", s.is_empty());           // false
println!("{}", "".is_empty());          // true
println!("{}", "   ".is_empty());       // false（空格不是空）
println!("{}", "abc".is_ascii());       // true
println!("{}", "你好".is_ascii());      // false
```

#### 4.10 字符串与所有权

```rust
// ⚠️ + 运算符会移动左侧 String 的所有权
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = s1 + &s2;
// s1 已被移动，不能再使用
// println!("{}", s1);                  // ❌ 编译错误
println!("{}", s2);                      // ✅ s2 是借用，没被移动
println!("{}", s3);                      // "hello world"

// ⚠️ String 不能同时存在可变引用和不可变引用
let mut s = String::from("hello");
let r1 = &s;                              // 不可变借用
// s.push_str(" world");                  // ❌ 编译错误：有不可变借用时不能修改
println!("{}", r1);

// ✅ 借用结束后可以修改
s.push_str(" world");                     // ✅ r1 已不再使用
```

#### 4.11 坑：字符串长度 ≠ 字符数

```rust
let s = "你好世界";

// ❌ len() 返回字节数，不是字符数
println!("{}", s.len());           // 12（每个中文 3 字节 × 4）
println!("{}", s.chars().count()); // 4（实际字符数）

// 不同字符占不同字节数（UTF-8 变长编码）
// ASCII 字符：1 字节
// 拉丁字母（é, ñ）：2 字节
// 中文/日文/韩文：3 字节
// emoji（🦀）：4 字节
let s = "aé中🦀";
println!("{}", s.len());           // 1 + 2 + 3 + 4 = 10
println!("{}", s.chars().count()); // 4

// ⚠️ emoji 可能是多个 char（组合字符）
let s = "e\u{301}";               // "é" 由 'e' + 组合重音符号组成
println!("{}", s.chars().count()); // 2（两个 char，但视觉上是一个字符）
```

#### 4.12 其他字符串类型速览

```rust
use std::ffi::{OsStr, OsString, CStr, CString};
use std::path::{Path, PathBuf};

// OsStr / OsString：与操作系统交互时使用
// 在 Unix 上是 UTF-8，在 Windows 上是 UTF-16
// 文件路径、环境变量、命令行参数等场景
let os: OsString = OsString::from("file.txt");

// CStr / CString：与 C 语言 FFI 交互时使用
// C 字符串以 \0 结尾，Rust 字符串不以 \0 结尾
let c = CString::new("hello").unwrap(); // 内部追加 \0

// Path / PathBuf：文件路径操作
// 等价于 &str / String 的关系，但专门处理路径分隔符
let path = PathBuf::from("/usr/local/bin");
println!("{}", path.display());          // /usr/local/bin
```

## 三、类型转换

Rust **不支持隐式数值类型转换**，数值类型之间必须使用 `as` 关键字显式转换。
但存在一些非数值的自动转换，如 Deref coercion（`&String` → `&str`）、never type coercion（`!` → 任意类型）等。

### 基本用法

```rust
let x: i32 = 42;
let y: f64 = x as f64;        // i32 -> f64
let z: u8 = 42u8;
```

#### 坑：`as` 类型转换会静默截断

`as` 转换不会报错，但可能丢数据，且行为因方向而异。

```rust
// ❌ 大类型 -> 小类型：截断高位
let x: u16 = 300;         // 二进制：0000 0001 0010 1100
let y: u8 = x as u8;      // 截断为：0010 1100 = 44
println!("300 as u8 = {}", y); // 44，不是 300！

// ❌ 有符号 -> 无符号：负数变成很大的正数
let a: i8 = -1;           // 二进制：1111 1111
let b: u8 = a as u8;      // 255
println!("-1i8 as u8 = {}", b); // 255

// ❌ 无符号 -> 有符号：大正数变成负数
let c: u8 = 200;
let d: i8 = c as i8;      // -56
println!("200u8 as i8 = {}", d); // -56

// ❌ f64 -> f32：精度丢失
let f: f64 = 1.23456789012345;
let g: f32 = f as f32;
println!("{}", g); // 1.2345679，后面几位没了

// ❌ 浮点 -> 整数：向零截断（不是四舍五入！）
let pi: f64 = 3.99;
let n: i32 = pi as i32;
println!("3.99 as i32 = {}", n); // 3，不是 4！

// ❌ 整数和浮点之间不能直接运算
let x: i32 = 5;
let y: f64 = 2.0;
// let z = x / y; // 编译错误：cannot divide i32 by f64
let z = x as f64 / y; // ✅ 先转换再运算
```

#### 安全转换：`From` / `Into` / `try_into`

`as` 转换的问题在于静默截断，Rust 提供了更安全的转换 trait：

**`From` 和 `Into`（不会丢数据的转换）：**

```rust
// From 已在 prelude 中自动导入，无需手动 use

// From：定义如何从 A 类型创建 B 类型
let x: i32 = 42;
let y: i64 = i64::from(x);    // i32 -> i64，安全，不会丢数据
let z: i64 = x.into();        // 等价写法（Into 自动实现）

// ⚠️ 反方向不行（i64 -> i32 可能丢数据）
// let a: i32 = i32::from(999999999999i64); // ❌ 编译错误：没有实现 From<i64>

// String 和 &str 的转换也是通过 From/Into
let s = String::from("hello");
let s2: String = "world".into();
```

**`try_into`（可能失败的转换）：**

```rust
let big: i64 = 5_000_000_000;
// let small: i32 = big as i32; // ❌ as 会静默截断

// ✅ 用 try_into 安全转换，失败返回 Err
let result: Result<i32, _> = big.try_into();
match result {
    Ok(v) => println!("转换成功：{}", v),
    Err(e) => println!("转换失败：{}", e), // 走这里，值超出 i32 范围
}

let small: i32 = 100i64.try_into().unwrap(); // 100，安全
```

**转换方式对比：**

| 方式 | 安全性 | 失败时 | 适用场景 |
|------|--------|--------|----------|
| `as` | 不安全 | 静默截断 | 确定不会丢数据的场景 |
| `From/Into` | 安全 | 编译拒绝 | 类型之间有无损转换 |
| `try_into` | 安全 | 返回 `Result` | 可能丢数据的转换 |

## 四、类型别名（Type Alias）

用 `type` 关键字为现有类型创建一个新名字，**不会创建新类型**，只是别名。

### 基本用法

```rust
// 简化复杂类型签名
type Kilometers = i32;       // Kilometers 就是 i32 的别名
type Point = (f64, f64);     // 二维坐标
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // 通用错误类型

let distance: Kilometers = 100;
let x: i32 = distance;       // ✅ Kilometers 和 i32 完全等价，可以直接赋值
println!("距离：{}km", distance);
```

### 常见使用场景

```rust
// 1. 减少重复书写
type Record = (String, i32, bool);
let r: Record = ("Alice".to_string(), 30, true);

// 2. 简化函数签名
type IntList = Vec<i32>;
fn sum(list: &IntList) -> i32 {
    list.iter().sum()
}

// 3. 泛型约束简化
type Callback = Box<dyn Fn(i32) -> i32>;
fn apply(f: &Callback, x: i32) -> i32 {
    f(x)
}

// 4. 标准库中的类型别名
// std::io::Result<T> = Result<T, std::io::Error>
// std::BoxError = Box<dyn std::error::Error>
```

### 坑：类型别名不是新类型

```rust
type Celsius = f64;
type Fahrenheit = f64;

let c: Celsius = 36.5;
let f: Fahrenheit = 97.7;

// ❌ 别名只是名字不同，底层是同一个类型，可以混用！
let sum = c + f; // ✅ 编译通过，但语义上是错的（摄氏度 + 华氏度）
println!("{}", sum); // 134.2，无意义的值

// ✅ 如果需要类型安全，应该用 newtype 模式（后续章节）
struct Celsius(f64);
struct Fahrenheit(f64);
// 现在 Celsius + Fahrenheit 会编译错误
```

## 五、类型推导与标注

```rust
let x = 5;            // 推导为 i32
let y = 2.0;          // 推导为 f64
let z: u8 = 5;        // 显式标注为 u8

// 打印变量的实际类型
println!("{}", std::any::type_name_of_val(&x)); // "i32"
```

### 坑：类型推导不够时的编译错误

```rust
// ❌ 编译器无法推导类型（缺少上下文）
// let x = "42".parse().unwrap(); // 编译错误：type annotations needed

// ✅ 必须标注类型
let x: i32 = "42".parse().unwrap();
let y: u64 = "42".parse().unwrap();

// ❌ 空集合无法推导元素类型
// let v = vec![]; // 编译错误：type annotations needed

// ✅ 标注元素类型
let v: Vec<i32> = vec![];
```

## 六、完整速查表

| 分类 | 类型 | 大小 | 默认推导 | 典型用途 |
|------|------|------|----------|----------|
| 整数 | `i8` `i16` `i32` `i64` `i128` `isize` | 8~128 位 | `i32` | 计数、运算 |
| 整数 | `u8` `u16` `u32` `u64` `u128` `usize` | 8~128 位 | — | 位运算、索引 |
| 浮点 | `f32` `f64` | 32/64 位 | `f64` | 小数运算 |
| 布尔 | `bool` | 1 字节 | — | 条件判断 |
| 字符 | `char` | 4 字节 | — | Unicode 字符 |
| Never | `!` | 0 字节 | — | 发散控制流（panic/loop） |
| 单元 | `()` | 0 字节 | — | 无返回值 |
| 元组 | `(T1, T2, ...)` | 各元素之和 | — | 多值返回 |
| 数组 | `[T; N]` | T 大小 × N | — | 固定长度集合 |
| 切片 | `&[T]` | 胖指针（16 字节） | — | 动态长度序列引用 |
| 字符串 | `&str` | 胖指针（16 字节） | — | 字符串切片（借用） |
| 字符串 | `String` | 24 字节 | — | 堆分配可变字符串 |
