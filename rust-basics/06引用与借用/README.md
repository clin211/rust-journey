# 引用与借用 (references & borrowing)

> 引用与借用（References & Borrowing）是 Rust 所有权系统的"访客机制"。
> 它让你在 **不转移所有权** 的前提下，临时访问数据——这是 Rust 几乎所有 API 的设计基础。

在上一章（05所有权）中，我们学到了一个让初学者头疼的问题：

```rust
fn calculate_length(s: String) -> usize {
    s.len()
}

fn main() {
    let text = String::from("hello");
    let len = calculate_length(text);
    // println!("{text}"); // ❌ text 已经被移动进函数，此处无法使用
}
```

所有权系统逼着你"用完即销"，但这显然不合理——你只是想读一下字符串的长度，凭什么要交出所有权？

这就是**引用**要解决的问题。本章你将真正把以下内容讲透：

- 引用的本质：对数据的临时访问权，不转移所有权
- `&T` 不可变引用：多个并存，只读，Copy
- `&mut T` 可变引用：独占，可写，限一个
- 借用规则：为什么这样设计，怎么用好它
- 非词法生命周期（NLL）：借用结束的真正时机
- 悬垂引用：Rust 如何在编译期彻底消灭它
- 切片（slice）：`&str` 和 `&[T]` 本质上也是借用
- 实战练习：`fn first_word(s: &str) -> &str`

---

## 示例文件

| 文件 | 主题 | 运行 |
|------|------|------|
| `examples/01_immutable_references.rs` | 不可变引用 `&T`：本质、多引用、Copy、解引用 | `cargo run --example 01_immutable_references` |
| `examples/02_mutable_references.rs` | 可变引用 `&mut T`：独占写、NLL 串行、实战 | `cargo run --example 02_mutable_references` |
| `examples/03_borrow_rules.rs` | 借用规则全景：规则一/二/三、NLL 详解、实践技巧 | `cargo run --example 03_borrow_rules` |
| `examples/04_dangling_references.rs` | 悬垂引用：编译器如何阻止、正确替代方案、生命周期初探 | `cargo run --example 04_dangling_references` |
| `examples/05_string_slice.rs` | 字符串切片 `&str`：胖指针、切片语法、UTF-8 边界 | `cargo run --example 05_string_slice` |
| `examples/06_array_slice.rs` | 数组/向量切片 `&[T]`：通用切片、常用方法、模式匹配 | `cargo run --example 06_array_slice` |
| `examples/07_first_word.rs` | 综合练习：`first_word` 的多种实现与扩展 | `cargo run --example 07_first_word` |

---

## 目录

- [引用与借用 (references \& borrowing)](#引用与借用-references--borrowing)
  - [示例文件](#示例文件)
  - [目录](#目录)
  - [一、为什么需要引用](#一为什么需要引用)
    - [1.1 所有权的代价](#11-所有权的代价)
    - [1.2 引用：借而不占](#12-引用借而不占)
    - [1.3 引用与指针的区别](#13-引用与指针的区别)
  - [二、不可变引用 `&T`](#二不可变引用-t)
    - [2.1 基础语法](#21-基础语法)
    - [2.2 多个不可变引用可以共存](#22-多个不可变引用可以共存)
    - [2.3 `&String` 和 `&str` 的区别](#23-string-和-str-的区别)
    - [2.4 解引用操作符 `*`](#24-解引用操作符-)
    - [2.5 引用实现了 Copy](#25-引用实现了-copy)
  - [三、可变引用 `&mut T`](#三可变引用-mut-t)
    - [3.1 基础语法](#31-基础语法)
    - [3.2 同一时刻只能有一个 `&mut T`](#32-同一时刻只能有一个-mut-t)
    - [3.3 用作用域串行（显式）](#33-用作用域串行显式)
    - [3.4 用 NLL 串行（隐式）](#34-用-nll-串行隐式)
  - [四、借用规则（borrow rules）](#四借用规则borrow-rules)
    - [4.1 三条规则](#41-三条规则)
    - [4.2 为什么这样设计——数据竞争](#42-为什么这样设计数据竞争)
    - [4.3 如何读懂借用报错](#43-如何读懂借用报错)
    - [4.4 不要把 `clone` 当成默认解法](#44-不要把-clone-当成默认解法)
  - [五、非词法生命周期（NLL）](#五非词法生命周期nll)
  - [六、悬垂引用（dangling reference）](#六悬垂引用dangling-reference)
    - [6.1 什么是悬垂引用](#61-什么是悬垂引用)
    - [6.2 Rust 如何在编译期阻止](#62-rust-如何在编译期阻止)
    - [6.3 正确的替代方案](#63-正确的替代方案)
    - [6.4 生命周期注解初探](#64-生命周期注解初探)
  - [七、切片（slice）](#七切片slice)
    - [7.1 切片的本质](#71-切片的本质)
    - [7.2 字符串切片 `&str`](#72-字符串切片-str)
    - [7.3 数组和向量切片 `&[T]`](#73-数组和向量切片-t)
    - [7.4 UTF-8 边界为什么要特别注意](#74-utf-8-边界为什么要特别注意)
  - [八、实战：`fn first_word(s: &str) -> &str`](#八实战fn-first_words-str---str)
  - [九、API 设计中的引用选择](#九api-设计中的引用选择)
  - [十、常见错误与易错点](#十常见错误与易错点)
  - [十一、综合练习](#十一综合练习)
  - [要点总结](#要点总结)

---

## 一、为什么需要引用

### 1.1 所有权的代价

所有权系统保证了内存安全，但也带来了一个问题：传值即失去控制。

```rust
fn print_length(s: String) {
    println!("长度: {}", s.len());
}

fn main() {
    let text = String::from("hello");
    print_length(text);
    // text 在这里已经无效了，所有权被移进函数后就结束了
}
```

更麻烦的做法是把所有权传进去再传出来：

```rust
fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len) // 把 s 再传回来，才能继续使用
}

fn main() {
    let text = String::from("hello");
    let (text, len) = calculate_length(text);
    println!("'{text}' 的长度是 {len}");
}
```

这显然很繁琐。大多数时候，函数只是需要"看一眼"数据，并不需要拥有它。

### 1.2 引用：借而不占

Rust 的解决方案是**引用（reference）**，也叫**借用（borrowing）**：

```rust
fn calculate_length(s: &String) -> usize { // &String：引用，不拿走所有权
    s.len()
}

fn main() {
    let text = String::from("hello");
    let len = calculate_length(&text); // &text：创建 text 的引用
    println!("'{text}' 的长度是 {len}"); // text 仍然有效
}
```

`&text` 的含义是：创建一个指向 `text` 所拥有数据的引用。

这里有一个关键直觉要建立：

> **引用让你拥有数据的地址，但不拥有数据本身。**
> 引用的生命周期结束时，它所指向的数据不会被 drop，因为引用并不"拥有"它。

### 1.3 引用与指针的区别

引用在底层确实是一个指针（内存地址），但 Rust 给引用加了编译期保证，使其根本不同于 C 中的裸指针：

| 特性 | Rust 引用 | C 裸指针 |
|------|-----------|----------|
| 是否可以为 null | 不能（永远非 null） | 可以 |
| 是否可能悬垂 | 不能（编译器阻止） | 可以 |
| 是否有类型信息 | 有，编译期检查 | 类型不安全 |
| 是否需要手动解引用 | 多数场景自动处理 | 必须显式 `*` |
| 是否有别名规则 | 有，借用规则静态保证 | 无保证 |

Rust 的引用可以理解为：**带编译期安全保证的指针**。

---

## 二、不可变引用 `&T`

### 2.1 基础语法

创建引用用 `&`，函数接收引用也用 `&`：

```rust
let x = 42;
let r = &x;       // 创建不可变引用
println!("{r}");  // 可以读取，Rust 自动解引用

let text = String::from("hello");
let r_text = &text;              // &String
println!("{}", r_text.len());   // 通过引用调用方法
```

使用引用传参时，函数不会取走所有权：

```rust
fn describe(s: &String) {
    println!("长度: {}, 内容: {s}", s.len());
}

fn main() {
    let name = String::from("Alice");
    describe(&name);        // 借用
    println!("{name}");     // name 仍然有效
}
```

### 2.2 多个不可变引用可以共存

这是不可变引用最重要的特性之一：

```rust
fn main() {
    let data = String::from("共享数据");

    let r1 = &data;
    let r2 = &data;
    let r3 = &data;

    // 三个引用可以同时存在，同时使用
    println!("r1={r1}, r2={r2}, r3={r3}");
}
```

原理很直观：多个只读访问不会产生冲突。就像多人可以同时阅读同一本书，因为没有人在修改它。

这也是"读-读不冲突，读-写才冲突"的核心思想。

### 2.3 `&String` 和 `&str` 的区别

这是初学者非常容易混淆的一对类型：

| 类型 | 含义 | 可以接受 |
|------|------|----------|
| `&String` | 对 `String` 的引用 | 只能接受 `&String` |
| `&str` | 字符串切片引用 | `&String`、字面量、切片 都可以 |

实际上，`String` 实现了 `Deref<Target = str>`，所以 `&String` 在需要 `&str` 的地方会自动转换：

```rust
fn greet(name: &str) {           // 参数写 &str，更通用
    println!("Hello, {name}!");
}

fn main() {
    let owned = String::from("Alice");
    greet(&owned);              // &String → &str（Deref 强制转换）
    greet("Bob");               // &str 字面量直接传
    greet(&owned[..3]);         // 字符串切片也是 &str
}
```

**经验法则：** 只读字符串参数，优先写 `&str`，而不是 `&String`。

### 2.4 解引用操作符 `*`

`*` 用于显式访问引用指向的值：

```rust
fn main() {
    let x = 42;
    let r = &x;

    println!("{}", *r);         // 显式解引用：42
    println!("{}", r);          // 隐式解引用：println! 自动处理

    let doubled = *r * 2;       // 先解引用得到值，再计算
    println!("{doubled}");      // 84
}
```

大多数情况下，Rust 会自动解引用（`.` 运算符、`println!`、比较运算等），不需要显式写 `*`。
需要显式写 `*` 的场景比较少见，主要出现在：

- 算术运算中获取引用后面的值
- 可变引用赋值：`*r = new_value;`
- 模式匹配中

### 2.5 引用实现了 Copy

不可变引用 `&T` 实现了 `Copy`，传递时会复制引用本身（只是一个指针，8 字节）：

```rust
fn print_it(s: &str) {
    println!("{s}");
}

fn main() {
    let text = String::from("Rust");
    let r = &text;

    print_it(r);    // 传入引用（Copy，r 仍然有效）
    print_it(r);    // 再次传入，仍然有效
    print_it(r);    // 第三次，依然有效

    let r2 = r;     // 复制引用（只复制了指针，text 未受影响）
    println!("{r}  {r2}");  // r 和 r2 都有效
}
```

注意：复制引用不会复制引用指向的数据——被引用的数据只有一份。

---

## 三、可变引用 `&mut T`

### 3.1 基础语法

要创建可变引用，有两个条件必须同时满足：

1. 被引用的变量本身必须声明为 `mut`
2. 创建引用时使用 `&mut`

```rust
fn add_suffix(s: &mut String) {
    s.push_str("!");   // 可以修改 s 指向的数据
}

fn main() {
    let mut text = String::from("hello"); // mut 是前提
    add_suffix(&mut text);               // &mut 创建可变引用
    println!("{text}");                  // "hello!"
}
```

常见错误：忘记 `mut` 之一：

```rust
// ❌ 原变量不是 mut
let text = String::from("hello");
// add_suffix(&mut text); // 编译错误：cannot borrow as mutable, as it is not declared as mutable

// ❌ 引用不写 &mut
let mut text = String::from("hello");
// add_suffix(&text);     // 编译错误：mismatched types, expected &mut String
```

### 3.2 同一时刻只能有一个 `&mut T`

这是 Rust 最重要的限制之一：

```rust
// ❌ 错误：两个 &mut 同时活跃
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;  // 编译错误：cannot borrow `s` as mutable more than once at a time
    println!("{r1}, {r2}");
}
```

编译器会明确告诉你：

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:4:14
  |
3 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
4 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
5 |     println!("{r1}, {r2}");
  |               ---- first borrow later used here
```

这个限制的根本原因是防止**数据竞争（data race）**，在下一节会详细解释。

### 3.3 用作用域串行（显式）

虽然同时只能有一个 `&mut`，但可以用作用域**串行**使用：

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("r1 使用完毕: {r1}");
    } // r1 的可变借用在这里结束

    {
        let r2 = &mut s;   // 前一个 &mut 已结束，可以开新的
        r2.push_str("!");
        println!("r2 使用完毕: {r2}");
    }

    println!("最终结果: {s}"); // "hello world!"
}
```

### 3.4 用 NLL 串行（隐式）

现代 Rust 有了 NLL（非词法生命周期），不一定需要手写 `{}`——只要两个 `&mut` 的**使用**不重叠就行：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push_str(" world");
    println!("r1: {r1}");
    // r1 最后一次使用在上面，借用在这里结束

    let r2 = &mut s;   // ✅ 此时 r1 已结束，可以借
    r2.push_str("!");
    println!("r2: {r2}");
}
```

---

## 四、借用规则（borrow rules）

### 4.1 三条规则

Rust 的借用规则可以总结为三条：

> **规则一：** 在任意时刻，可以有**任意数量**的不可变引用（`&T`）。
>
> **规则二：** 在任意时刻，只能有**唯一一个**可变引用（`&mut T`）。
>
> **规则三：** 不可变引用和可变引用**不能同时活跃**。

用表格总结：

| 情形 | 是否允许 |
|------|----------|
| 多个 `&T` 同时存在 | ✅ 允许 |
| 单独一个 `&mut T` | ✅ 允许 |
| `&T` 和 `&mut T` 同时活跃 | ❌ 不允许 |
| 两个 `&mut T` 同时活跃 | ❌ 不允许 |

"活跃"的意思是：引用从创建到最后一次使用的这个区间内。

### 4.2 为什么这样设计——数据竞争

这些规则是为了在编译期彻底消灭**数据竞争（data race）**。

数据竞争需要同时满足三个条件：

1. 两个或多个指针同时访问同一数据
2. 其中至少有一个指针在**写入**数据
3. 没有同步机制（比如锁）

Rust 的借用规则如何应对：

- 规则一允许多个 `&T`：多个**读**是安全的，不满足条件 2
- 规则二保证 `&mut T` 独占：写时没有其他访问，不满足条件 1
- 规则三禁止读写共存：直接消灭了"读 + 写"的可能

这就是为什么 Rust 可以在**没有运行时开销**的情况下做到无数据竞争——它在编译期就把所有可能的竞争情形都排除了。

### 4.3 如何读懂借用报错

Rust 的借用错误信息通常很详细，有几个关键词要认识：

**`cannot borrow as mutable because it is also borrowed as immutable`**

```
let r_read = &data;
let r_write = &mut data;  // ← 这里报错
println!("{r_read}");     // ← r_read 在这里还要用
```

意思是：你有只读借用还没用完，不能开始可变借用。
解法：先把只读借用用完，再进行可变借用。

**`cannot borrow as mutable more than once at a time`**

```
let r1 = &mut data;
let r2 = &mut data;   // ← 这里报错
```

意思是：可变借用不能同时存在两个。
解法：串行使用（用完 r1 再用 r2），或者用花括号隔离。

**`borrow later used here`**

这是 Rust 指出"引起冲突的后续使用点"在哪里。
先找到这行，再往上找借用的创建点，就能理清冲突逻辑。

### 4.4 不要把 `clone` 当成默认解法

初学者一遇到借用错误，第一反应往往是：

```rust
let copy = data.clone(); // 弄一份副本来规避冲突
```

这有时是合理的，但很多时候只是掩盖了设计问题。

遇到借用报错时，推荐按顺序思考：

1. **能不能用引用？** 函数是否只是"读一下"，不需要拥有数据？
2. **能不能缩小借用范围？** 借用是否比必要的存活时间长？
3. **能不能调整语句顺序？** 先用完只读借用，再做可变操作？
4. **参数类型是否合理？** 是否该用 `&str` 而非 `&String`？
5. **真的需要两份独立数据？** 如果是，再用 `clone()`。

---

## 五、非词法生命周期（NLL）

早期 Rust（2018 edition 之前）的借用有一个痛点：借用的范围是词法作用域（`{}`），导致很多直觉上合理的代码无法编译。

现代 Rust 使用 **NLL（Non-Lexical Lifetimes，非词法生命周期）**：

> 借用从创建引用的那行开始，在**最后一次使用**引用的那行结束，而不是在花括号结束时。

这让借用规则更灵活，避免了很多不必要的限制：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;                    // r1 借用开始
    let r2 = &s;                    // r2 借用开始
    println!("{r1} and {r2}");      // r1、r2 最后一次使用，借用在这里结束

    // NLL：r1 和 r2 的借用已经结束，虽然变量名还在"词法作用域"内
    let r3 = &mut s;                // ✅ 可以开始可变借用
    r3.push_str(" world");
    println!("{r3}");
}
```

NLL 的直觉：**编译器关注的是"这个引用最后用在哪里"，而不是"变量名到哪里才出作用域"。**

一个需要小心的场景——返回值持有借用：

```rust
fn main() {
    let mut map = std::collections::HashMap::new();
    map.insert("key", 1);

    // ❌ 这里 entry 持有对 map 的可变借用
    // let entry = map.entry("key").or_insert(0);
    // map.insert("other", 2); // 不能，entry 还活着
    // println!("{entry}");

    // ✅ 用完 entry 再操作 map
    {
        let count = map.entry("key").or_insert(0);
        *count += 1;
    } // count 的借用在这里结束
    map.insert("other", 2); // 现在可以了
}
```

---

## 六、悬垂引用（dangling reference）

### 6.1 什么是悬垂引用

**悬垂引用（Dangling Reference）** 是指：引用所指向的内存已经被释放，但引用本身还存在、还被使用。

在 C 语言中，这是一个臭名昭著的错误来源：

```c
// C 语言：返回局部变量的地址 → 悬垂指针 → 未定义行为
int* make_number() {
    int x = 42;
    return &x;  // x 在函数结束时释放，返回了无效地址
}
```

这类错误在运行时可能导致程序崩溃、数据损坏，甚至安全漏洞——而且往往难以复现。

### 6.2 Rust 如何在编译期阻止

Rust 的**借用检查器（Borrow Checker）**在编译期静态分析所有引用的有效期。它的核心原则是：

> **引用存活的时间不能超过它所指向的数据存活的时间。**

最常见的悬垂引用场景——返回局部变量的引用——在 Rust 中直接无法编译：

```rust
// ❌ 这段代码无法编译
fn dangle() -> &String {
    let s = String::from("hello");
    &s         // 返回 s 的引用
}              // s 在这里 drop，堆内存释放，但 &s 还在被返回
               // 编译错误：missing lifetime specifier
               //           this function's return type contains a borrowed value,
               //           but there is no value for it to be borrowed from
```

编译器拒绝了这段代码。它注意到：函数内部的 `s` 在函数结束时会被释放，但你想把指向它的引用返回出去，这就会创建一个悬垂引用。

### 6.3 正确的替代方案

**方案一：返回所有权，不返回引用**

如果数据是在函数内部创建的，就直接把所有权返回出去：

```rust
// ✅ 返回 String 本身，转移所有权给调用方
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 不是 &s，而是 s 本身——所有权转给调用方，数据不会被释放
}
```

**方案二：引用来自输入参数（最常见）**

如果需要返回引用，让引用指向的数据来自调用方（通过参数传入），而不是函数内部：

```rust
// ✅ 返回的引用来自输入参数 s，生命周期由调用方控制
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```

函数结束后，`s` 所指向的数据仍然由调用方持有，所以返回的引用依然有效。

### 6.4 生命周期注解初探

当函数有多个引用参数时，编译器可能无法自动判断返回的引用"来自哪个参数"，这时需要用**生命周期注解**明确说明：

```rust
// 'a 是生命周期参数：表示返回值的有效期 = s1 和 s2 中较短的那个
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}

fn main() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xy");
        result = longer(&s1, &s2);
        println!("更长的字符串: {result}"); // ✅ 在 s2 有效的范围内使用
    }
    // println!("{result}"); // ❌ s2 已释放，result 可能来自 s2，不能使用
}
```

生命周期是一个独立的深入主题（见 `12生命周期/`），这里只需建立直觉：

> **生命周期注解不改变引用的实际存活时间，只是告诉编译器引用之间的关联关系。**

---

## 七、切片（slice）

### 7.1 切片的本质

切片是对某段**连续内存**的引用。它在栈上是一个**胖指针（fat pointer）**：

```
&str / &[T] 在栈上的布局：
┌─────────────────────────────┐
│ ptr  → 数据起始字节的地址    │  8 bytes（指针）
│ len  → 切片包含的元素个数    │  8 bytes（长度）
└─────────────────────────────┘
```

切片**不拥有数据**，不负责释放——它只是"借了一个窗口"来查看已有数据的某个范围。

这也是为什么切片本质上也是**借用**：数据的 owner 依然是原来的变量（`String`、数组、`Vec`）。

### 7.2 字符串切片 `&str`

**切片语法：**

```rust
let s = String::from("hello world");
//                    0123456789...

let hello = &s[..5];    // 字节 0~4，等价于 &s[0..5]
let world = &s[6..];    // 字节 6 到末尾，等价于 &s[6..11]
let all   = &s[..];     // 整个字符串，等价于 &s[0..11]
let part  = &s[3..8];   // 字节 3~7
```

**字符串字面量就是 `&str`：**

```rust
let literal: &str = "I'm stored in the binary!"; // 'static 生命周期
// 字面量在编译时就被嵌入程序的二进制文件中（.rodata 段）
// 整个程序运行期间都有效，所以其生命周期是 'static
```

**`&str` 比 `&String` 通用：**

```rust
fn print_greeting(s: &str) {    // 接受 &str：最通用的选择
    println!("问候: {s}");
}

fn main() {
    let owned = String::from("Alice");
    print_greeting(&owned);           // &String → 自动 deref 成 &str
    print_greeting("Bob");            // 字面量本身就是 &str
    print_greeting(&owned[..3]);      // 切片也是 &str
}
```

**切片借用活跃时不能修改原字符串：**

```rust
fn main() {
    let mut title = String::from("rust slice");

    let first = &title[..4];   // first 借用了 title 的数据

    // ❌ 错误：切片 first 还活跃时，不能对 title 进行修改
    // title.push('!');         // push 可能触发内存重新分配，使 first 的 ptr 失效

    println!("first: {first}"); // first 最后一次使用，借用在这里结束

    title.push('!');            // ✅ 现在 first 已结束，可以修改
    println!("title: {title}");
}
```

### 7.3 数组和向量切片 `&[T]`

数组和 `Vec<T>` 也有对应的切片类型 `&[T]`：

```rust
fn main() {
    // 数组切片
    let arr = [10, 20, 30, 40, 50];
    let part: &[i32] = &arr[1..4];  // [20, 30, 40]
    println!("{:?}", part);

    // Vec 切片
    let v = vec![1, 2, 3, 4, 5];
    let mid: &[i32] = &v[1..4];    // [2, 3, 4]
    println!("{:?}", mid);
}
```

函数参数写 `&[T]` 比 `&Vec<T>` 更通用：

```rust
fn sum(nums: &[i32]) -> i32 {        // &[i32]：接受数组切片或 Vec 切片
    nums.iter().sum()
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let v = vec![10, 20, 30];

    println!("{}", sum(&arr));        // 数组引用 → &[i32]
    println!("{}", sum(&v));          // Vec 引用 → &[i32]（Deref 强制转换）
    println!("{}", sum(&v[1..]));     // Vec 的切片，也是 &[i32]
}
```

常用的切片方法：

```rust
let s = &[1, 2, 3, 4, 5][..];

s.len()             // 元素个数
s.is_empty()        // 是否为空
s.first()           // Option<&T>，第一个元素
s.last()            // Option<&T>，最后一个元素
s.get(2)            // Option<&T>，安全的索引访问（不会 panic）
s.contains(&3)      // 是否包含某个值
s.iter()            // 遍历迭代器
```

可变切片 `&mut [T]` 可以原地修改元素：

```rust
fn double_all(nums: &mut [i32]) {
    for n in nums.iter_mut() {
        *n *= 2;
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    double_all(&mut v);
    println!("{:?}", v); // [2, 4, 6, 8, 10]

    let mut arr = [3, 1, 4, 1, 5];
    arr.sort();          // sort() 接受的就是 &mut [T]
    println!("{:?}", arr); // [1, 1, 3, 4, 5]
}
```

### 7.4 UTF-8 边界为什么要特别注意

Rust 的字符串 `String` 和 `&str` 内部存储的是 **UTF-8 字节序列**。ASCII 字符每个占 1 字节，但 Unicode 字符（如汉字、emoji）通常占 3~4 字节。

字符串切片的索引是**字节位置**，不是字符序号：

```rust
let s = String::from("你好Rust");
// 字节布局：
// 你 → 字节 0,1,2（3字节）
// 好 → 字节 3,4,5（3字节）
// R  → 字节 6
// u  → 字节 7
// s  → 字节 8
// t  → 字节 9

// ❌ 危险：字节 0..1 不是完整字符边界
// let bad = &s[0..1];  // 运行时 panic：byte index 1 is not a char boundary

// ✅ 安全：使用 char_indices 找到字符边界
let prefix: &str = s.char_indices()
    .nth(2)                         // 第 3 个字符的起始字节索引
    .map(|(i, _)| &s[..i])         // 截取到该位置
    .unwrap_or(&s);
println!("{prefix}");              // "你好"
```

几种安全处理 Unicode 字符串的方式：

```rust
let s = "你好，世界！";

// 字符数（不是字节数）
println!("字符数: {}", s.chars().count());

// 按字符遍历
for c in s.chars() {
    print!("[{c}]");
}
println!();

// 字符 + 字节索引
for (i, c) in s.char_indices() {
    println!("字节{i}: {c}");
}

// 安全地取前 N 个字符组成的切片
fn char_prefix(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}
```

经验法则：

> **处理字符串切片索引时，永远用 `char_indices()` 找边界，不要手写字节偏移。**

---

## 八、实战：`fn first_word(s: &str) -> &str`

这个函数是 Rust 官方教程的经典练习，非常集中地体现了引用与借用的核心思想。

**版本一：字节迭代（经典实现）**

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();                   // 把 &str 看成字节序列

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {                       // 找到第一个空格
            return &s[..i];                     // 返回空格前的切片
        }
    }

    &s[..]                                      // 没有空格，整个字符串就是第一个词
}
```

这里体现的核心借用规则：

- 参数 `s: &str` 是借用，函数不拥有字符串
- 返回值 `&str` 也是借用，它指向 `s` 的一段数据
- 编译器自动推断：返回值的生命周期与参数 `s` 一致
- 等价于显式写法：`fn first_word<'a>(s: &'a str) -> &'a str`

**版本二：迭代器（惯用写法）**

```rust
fn first_word_v2(s: &str) -> &str {
    s.split_whitespace()        // 按空白字符切分，返回 &str 的迭代器
     .next()                    // 取第一个
     .unwrap_or("")             // 空字符串时返回 ""
}
```

**为什么不能在使用返回值期间修改原字符串？**

```rust
fn main() {
    let mut sentence = String::from("hello world rust");
    let word = first_word(&sentence);    // word 借用了 sentence 的数据

    // ❌ 错误：sentence 有不可变借用（word）活跃，不能进行修改
    // sentence.clear();                 // clear() 需要可变引用
    // sentence.push_str("!");           // 同样需要可变引用

    println!("第一个词: {word}");        // word 最后一次使用，借用结束

    sentence.push_str("!");             // ✅ 现在可以修改了
    println!("修改后: {sentence}");
}
```

**扩展练习：更多单词函数**

```rust
// 取最后一个单词
fn last_word(s: &str) -> &str {
    s.split_whitespace().last().unwrap_or("")
}

// 取第 n 个单词（从 0 开始）
fn nth_word(s: &str, n: usize) -> Option<&str> {
    s.split_whitespace().nth(n)
}

// 统计单词数
fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}
```

---

## 九、API 设计中的引用选择

学完本章后，你应该能理性地做出这些 API 设计决策：

**字符串参数：用 `&str`，不用 `&String`**

```rust
// ❌ 不够通用：只能接受 &String
fn print_name(name: &String) { println!("{name}"); }

// ✅ 更通用：&String、字面量、切片都能传
fn print_name(name: &str) { println!("{name}"); }
```

**集合参数：用 `&[T]`，不用 `&Vec<T>`**

```rust
// ❌ 不够通用：只能接受 &Vec<i32>
fn average(nums: &Vec<f64>) -> f64 { ... }

// ✅ 更通用：数组切片和 Vec 切片都能传
fn average(nums: &[f64]) -> f64 {
    nums.iter().sum::<f64>() / nums.len() as f64
}
```

**方法接收者：`self`、`&self`、`&mut self` 怎么选**

```rust
struct Counter {
    count: u32,
    name: String,
}

impl Counter {
    fn value(&self) -> u32 {          // &self：只读访问，最常用
        self.count
    }

    fn name(&self) -> &str {          // &self：返回内部数据的引用
        &self.name
    }

    fn increment(&mut self) {         // &mut self：需要修改自身
        self.count += 1;
    }

    fn into_name(self) -> String {    // self：消费整个实例，获取内部数据
        self.name                     // 调用后 self 失效
    }
}
```

| 接收者 | 语义 | 适用场景 |
|--------|------|----------|
| `&self` | 只读借用 | 读取状态、计算、返回引用 |
| `&mut self` | 可变借用 | 修改内部状态 |
| `self` | 消费所有权 | 将自身转换为别的类型 |

**传值 / 借用 / clone 的判断准则**

| 场景 | 推荐方式 |
|------|----------|
| 函数只是读取数据 | `&T` / `&str` / `&[T]` |
| 函数需要修改调用方的数据 | `&mut T` |
| 函数需要接管数据（保存进结构体等） | 传值（move） |
| 需要长期保留两份独立数据 | `clone()` |

---

## 十、常见错误与易错点

### 1. 忘记两个 `mut`

```rust
let text = String::from("hello");
// change(&mut text); // ❌ text 不是 mut，无法创建 &mut
```

修复：`let mut text = ...;`

### 2. 同时创建两个可变引用

```rust
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s; // ❌ r1 还活跃，不能创建第二个 &mut
```

修复：用花括号隔离，或调整语句顺序，利用 NLL。

### 3. 只读借用活跃时尝试可变借用

```rust
let mut s = String::from("hello");
let r = &s;
// s.push_str("!"); // ❌ r 还活跃
println!("{r}");    // r 最后使用在这里
s.push_str("!");   // ✅ 现在 r 已结束
```

修复：先用完只读借用，再进行可变操作。

### 4. 返回局部变量的引用

```rust
// fn bad() -> &String {
//     let s = String::from("hello");
//     &s  // ❌ s 在函数结束时被 drop
// }
```

修复：返回 `String`（所有权），而不是 `&String`（引用）。

### 5. 对 UTF-8 字符串按字节切片

```rust
let s = String::from("你好");
// let bad = &s[0..1]; // ❌ 运行时 panic：不是字符边界
```

修复：使用 `char_indices()` 找到合法的字节边界再切片。

### 6. 切片还活跃时修改原字符串

```rust
let mut s = String::from("hello world");
let word = &s[..5];         // word 借用了 s
// s.push_str("!");          // ❌ word 还活跃
println!("{word}");          // word 最后使用
s.push_str("!");             // ✅ 现在可以
```

修复：确保先用完切片，再修改原数据。

### 7. 把 `clone` 当成遇到借用错误的第一反应

```rust
let copy = data.clone(); // ⚠️ 有时合理，但常常只是在掩盖设计问题
```

先检查：
- 能不能用引用？
- 能不能缩小借用范围？
- 能不能调整代码顺序？

### 8. 混淆 `&String` 和 `&str`

```rust
fn greet(name: &String) { ... } // 参数写成 &String

greet("hello"); // ❌ 类型不匹配
```

修复：函数参数改成 `&str`，更通用。

### 9. 不理解 NLL，误以为借用要到花括号才结束

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];         // 借用开始
println!("{first}");       // 借用在这里结束（最后一次使用）
v.push(4);                 // ✅ NLL 下这里可以，first 已结束
```

---

## 十一、综合练习

完成下面的练习，把本章内容真正练熟。

### 练习 1：只读函数不拿走所有权

要求：写一个函数 `fn count_vowels(s: &str) -> usize`，统计字符串中元音字母（a/e/i/o/u）的数量。调用后原字符串仍可继续使用。

提示：`s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()`

### 练习 2：可变借用修改原数据

要求：写一个函数 `fn normalize(s: &mut String)`，把字符串转为小写并去除首尾空白。调用后原变量发生变化。

提示：用 `*s = s.trim().to_lowercase();`

### 练习 3：返回切片

要求：写一个函数 `fn longest_word(s: &str) -> &str`，找出字符串中最长的单词并返回其切片（不用 clone，直接返回引用）。

提示：用 `split_whitespace().max_by_key(|w| w.len()).unwrap_or("")`

### 练习 4：数组切片求最大值

要求：写一个函数 `fn max_value(nums: &[i32]) -> Option<i32>`，返回切片中的最大值（不拥有数据）。同时让它能接受 `[i32; N]` 和 `Vec<i32>`。

提示：`nums.iter().copied().max()`

### 练习 5：结构体方法设计

定义以下结构体，并分别实现三种方法接收者：

```rust
struct Note {
    title: String,
    content: String,
}

impl Note {
    // 实现 fn title(&self) -> &str              只读，返回 title 的引用
    // 实现 fn append(&mut self, text: &str)      可变借用，在 content 末尾追加
    // 实现 fn into_content(self) -> String       消费实例，返回 content 的所有权
}
```

思考：三种方法分别对应哪种借用/所有权语义？

### 练习 6：安全截取 Unicode 前缀

要求：写一个函数 `fn char_prefix<'a>(s: &'a str, n: usize) -> &'a str`，安全地截取字符串的前 `n` 个字符（按字符数，不是字节数），用 `char_indices` 实现。

测试用例：

- `char_prefix("hello", 3)` → `"hel"`
- `char_prefix("你好世界", 2)` → `"你好"`
- `char_prefix("Rust", 10)` → `"Rust"`（不足时返回整个字符串）

### 练习 7：借用规则综合排查

下面的代码有借用冲突，找出原因并修复（不允许用 `clone`）：

```rust
fn main() {
    let mut words = vec!["apple", "banana", "cherry"];
    let first = &words[0];
    words.push("date");
    println!("第一个: {first}");
}
```

提示：思考 `push` 操作可能导致什么，以及如何调整语句顺序。

---

## 要点总结

### 引用的核心概念

1. 引用 `&T` 是对数据的"临时访问权"，不转移所有权
2. 引用的本质是带安全保证的指针（非 null、不悬垂、有类型）
3. 借用的数据在引用存活期间不会被 drop
4. `&T` 是不可变引用，`&mut T` 是可变引用

### 借用规则（三条，必须记住）

1. 同一时刻，可以有任意数量的不可变引用 `&T`（多读不冲突）
2. 同一时刻，只能有一个可变引用 `&mut T`（写必须独占）
3. 不可变引用和可变引用不能同时活跃（读写不能并发）

### NLL 与借用范围

1. 借用从创建引用的那行开始
2. 借用在**最后一次使用**引用的那行结束（不是花括号结束时）
3. NLL 让借用规则更灵活，不需要频繁用 `{}` 隔离
4. 遇到借用冲突时，先尝试调整语句顺序，再考虑其他方案

### 悬垂引用

1. 悬垂引用 = 引用指向的内存已释放，引用仍存在
2. Rust 借用检查器在编译期静态消灭悬垂引用，零运行时开销
3. 局部变量的引用不能逃出其作用域
4. 函数内创建的数据，应返回所有权而非引用

### 切片

1. 切片是对连续内存的引用（胖指针：ptr + len）
2. `&str` 是字符串切片，`&[T]` 是数组/向量切片
3. 切片不拥有数据，本质上也是借用
4. 字符串切片索引是字节位置，非 ASCII 字符要用 `char_indices`

### API 设计最佳实践

1. 只读字符串参数优先用 `&str`，比 `&String` 更通用
2. 只读集合参数优先用 `&[T]`，比 `&Vec<T>` 更通用
3. 方法接收者：只读用 `&self`，修改用 `&mut self`，转换用 `self`
4. 遇到借用冲突，按顺序考虑：用引用 → 缩小范围 → 调整顺序 → `clone()`

### 最实用的判断准则

| 你的需求 | 推荐写法 |
|----------|----------|
| 只是读取数据 | `&T` / `&str` / `&[T]` |
| 需要修改原数据 | `&mut T` |
| 需要长期持有/保存数据 | 传值（move） |
| 需要两份独立副本 | `clone()` |
| 字符串只读参数 | `&str`（不是 `&String`） |
| 集合只读参数 | `&[T]`（不是 `&Vec<T>`） |

---

> **下一步：** 掌握引用与借用后，`07结构体/` 会大量用到 `&self`、`&mut self`、`&str` 这些模式。
> 等进入 `12生命周期/`，你会更深入理解为什么编译器有时需要你写 `'a` 标注。
> 这些都是在本章基础上的自然延伸。