# 所有权 (ownership)

> 所有权（Ownership）是 Rust 最核心、最有辨识度的特性。
> 它让 Rust 在 **没有垃圾回收（GC）** 的情况下，依然能保证 **内存安全**、**线程安全** 和 **资源自动释放**。

在 C/C++ 中，程序员需要手动管理内存；在 Java / Go / Python 中，通常依赖垃圾回收器；而 Rust 选择了第三条路：

- 在 **编译期** 用严格规则管理资源
- 在 **运行期** 几乎不额外付出 GC 成本
- 在保证性能的同时，尽量消灭悬垂指针、重复释放、数据竞争等问题

这一章不只是“认识所有权”这么简单，而是要把下面这些真正讲透：

- 什么是所有权，Rust 为什么需要它
- `String` 为什么会 move，`clone()` 和 `Copy` 到底差在哪
- 为什么函数签名其实就是“资源契约”
- 什么是引用（reference）与借用（borrowing）
- 为什么可变借用必须独占
- `&String`、`&str`、`String` 到底该怎么选
- 切片（slice）为什么本质上也是借用
- Rust 如何在编译期阻止悬垂引用（dangling reference）
- 写代码时什么时候该传值、什么时候该借用、什么时候才该 `clone()`

---

## 示例文件

| 文件 | 主题 | 运行 |
|------|------|------|
| `examples/01_ownership_basics.rs` | 所有权基础、作用域、资源释放 | `cargo run --example 01_ownership_basics` |
| `examples/02_move_and_clone.rs` | move、clone、所有权转移场景 | `cargo run --example 02_move_and_clone` |
| `examples/03_copy_types.rs` | Copy 类型、引用为什么也是 Copy | `cargo run --example 03_copy_types` |
| `examples/04_functions_and_ownership.rs` | 函数参数、返回值、传值 / 借用 / 可变借用 | `cargo run --example 04_functions_and_ownership` |
| `examples/05_borrowing.rs` | 不可变借用、多个引用、`&String` vs `&str` | `cargo run --example 05_borrowing` |
| `examples/06_mutable_references.rs` | 可变借用、独占访问、借用结束时机 | `cargo run --example 06_mutable_references` |
| `examples/07_slice.rs` | `&str`、`&[T]`、UTF-8 边界、切片实践 | `cargo run --example 07_slice` |
| `examples/08_dangling_references.rs` | 悬垂引用与正确替代方案 | `cargo run --example 08_dangling_references` |
| `examples/09_struct_and_vec_ownership.rs` | 结构体字段、`Vec<String>`、拥有资源的类型 | `cargo run --example 09_struct_and_vec_ownership` |
| `examples/10_string_vs_str.rs` | `String`、`&String`、`&str` 的区别与 API 设计 | `cargo run --example 10_string_vs_str` |
| `examples/11_borrowing_lifetimes_intuition.rs` | 借用存活范围、NLL 直觉 | `cargo run --example 11_borrowing_lifetimes_intuition` |
| `examples/12_method_receivers_and_api_design.rs` | `self` / `&self` / `&mut self` 与方法设计 | `cargo run --example 12_method_receivers_and_api_design` |

---

## 目录

- [所有权 (ownership)](#所有权-ownership)
  - [示例文件](#示例文件)
  - [目录](#目录)
  - [一、为什么需要所有权](#一为什么需要所有权)
    - [1.1 栈（stack）与堆（heap）](#11-栈stack与堆heap)
    - [1.2 Rust 到底想避免什么问题](#12-rust-到底想避免什么问题)
    - [1.3 Rust、GC、手动管理的区别](#13-rustgc手动管理的区别)
  - [二、所有权的三条核心规则](#二所有权的三条核心规则)
  - [三、作用域（scope）与资源释放](#三作用域scope与资源释放)
  - [四、`String` 与 move 语义](#四string-与-move-语义)
    - [4.1 为什么 `String` 会 move](#41-为什么-string-会-move)
    - [4.2 `clone()`：显式深拷贝](#42-clone显式深拷贝)
    - [4.3 move 不只发生在赋值](#43-move-不只发生在赋值)
  - [五、`Copy` 类型](#五copy-类型)
  - [六、函数中的所有权](#六函数中的所有权)
    - [6.1 函数签名就是资源契约](#61-函数签名就是资源契约)
    - [6.2 返回值也参与所有权转移](#62-返回值也参与所有权转移)
    - [6.3 怎么决定用 `T`、`&T` 还是 `&mut T`](#63-怎么决定用-tt-还是-mut-t)
  - [七、引用（reference）与借用（borrowing）](#七引用reference与借用borrowing)
    - [7.1 不可变引用](#71-不可变引用)
    - [7.2 多个不可变引用为什么可以同时存在](#72-多个不可变引用为什么可以同时存在)
    - [7.3 `&String` 和 `&str` 的区别](#73-string-和-str-的区别)
  - [八、可变引用（`&mut`）](#八可变引用mut)
  - [九、借用规则（borrow rules）](#九借用规则borrow-rules)
    - [9.1 规则总结表](#91-规则总结表)
    - [9.2 如何读 Rust 的借用报错](#92-如何读-rust-的借用报错)
    - [9.3 非词法生命周期（NLL）直觉](#93-非词法生命周期nll直觉)
    - [9.4 不要把 clone 当成默认解法](#94-不要把-clone-当成默认解法)
  - [十、切片（slice）](#十切片slice)
    - [10.1 字符串切片 `&str`](#101-字符串切片-str)
    - [10.2 数组 / 向量切片 `&[T]`](#102-数组--向量切片-t)
    - [10.3 UTF-8 边界为什么要特别小心](#103-utf-8-边界为什么要特别小心)
  - [十一、悬垂引用（dangling reference）](#十一悬垂引用dangling-reference)
  - [十二、实战中的 API 设计](#十二实战中的-api-设计)
    - [12.1 字符串参数优先考虑 `&str`](#121-字符串参数优先考虑-str)
    - [12.2 集合参数优先考虑切片](#122-集合参数优先考虑切片)
    - [12.3 方法接收者：`self`、`&self`、`&mut self`](#123-方法接收者selfselfmut-self)
  - [十三、常见错误与易错点](#十三常见错误与易错点)
  - [十四、综合练习](#十四综合练习)
  - [要点总结](#要点总结)

---

## 一、为什么需要所有权

### 1.1 栈（stack）与堆（heap）

理解所有权之前，先理解 Rust 里经常出现的 **栈** 和 **堆**。

| 项目 | 栈（stack） | 堆（heap） |
|---|---|---|
| 分配速度 | 快 | 相对慢 |
| 数据大小 | 通常固定、已知 | 通常可变、运行期确定 |
| 管理方式 | 编译器自动管理 | 需要记录分配与释放 |
| 典型数据 | `i32`、`bool`、`char`、固定大小元组 | `String`、`Vec<T>`、`Box<T>` |

例如：

```rust
let x = 42;
```

`x` 的大小固定，通常放在栈上。

再看：

```rust
let s = String::from("hello");
```

`String` 本身有一部分元数据在栈上：

- 指向堆内存的指针
- 长度（length）
- 容量（capacity）

真正的字符串内容通常在堆上。

于是问题来了：

> 堆上的那块内存，到底该由谁负责释放？

如果没人管，会 **内存泄漏**；如果两个人都管，会 **重复释放（double free）**；如果释放后还继续访问，会产生 **悬垂指针（dangling pointer）**。

Rust 的答案就是：**所有权**。

### 1.2 Rust 到底想避免什么问题

你可以把所有权系统理解成：Rust 在编译期提前拦截一批非常危险、而且很常见的资源错误。

| 问题 | 典型风险 | Rust 主要靠什么避免 |
|---|---|---|
| 重复释放（double free） | 同一块资源被释放两次 | move + 单一 owner |
| 悬垂引用（dangling reference） | 资源已经失效，但还有引用指向它 | 借用检查器 |
| 释放后继续使用（use after free） | 读取或修改已失效内存 | 所有权 + 生命周期分析 |
| 数据竞争（data race） | 多线程同时读写同一数据 | 借用规则 + 类型系统 |

换句话说，Rust 的“限制”不是为了折磨你，而是为了把很多运行期才会炸掉的问题，直接提前到编译期发现。

### 1.3 Rust、GC、手动管理的区别

Rust 既不是纯手动内存管理，也不是依赖垃圾回收。

| 方案 | 优点 | 缺点 |
|---|---|---|
| 手动管理（C/C++ 常见风格） | 灵活、零运行时回收开销 | 容易忘记释放、重复释放、悬垂指针 |
| 垃圾回收（Java / Go / Python 常见思路） | 写起来省心 | 运行时有回收成本，暂停时间或开销不完全可控 |
| Rust 所有权 | 编译期保证资源使用合法，运行期成本低 | 需要学习所有权和借用规则 |

Rust 的核心思路是：

- 在编译期检查资源使用是否合法
- 在作用域结束时自动释放资源
- 让“谁拥有资源”这件事始终足够明确

---

## 二、所有权的三条核心规则

Rust 所有权系统可以先记住三条最重要的规则：

1. **Rust 中的每一个值，都有一个所有者（owner）**
2. **一个值在同一时刻只能有一个所有者**
3. **当所有者离开作用域（scope）时，这个值会被自动丢弃（drop）**

看起来很简单，但这三条规则几乎解释了本章全部内容。

```rust
fn main() {
    let s = String::from("hello");
    println!("{s}");
} // 这里 s 离开作用域，String 自动被 drop
```

这里可以先建立一个非常重要的直觉：

- **变量名** 是绑定（binding）
- **值** 是那份真正的数据
- **owner** 是“谁对这份数据负责”

初学者最容易误解的一点是：

> “一个值只能有一个 owner” 不等于 “一个值永远只能出现一个名字”。

比如 move 之后，原来的名字失效了，但值本身还在，只是 ownership 已经转移给了新的绑定。

---

## 三、作用域（scope）与资源释放

作用域就是变量有效的那段范围，通常由 `{}` 决定。

```rust
fn main() {
    {
        let name = String::from("Rust");
        println!("进入作用域: {name}");
    } // name 在这里离开作用域，被自动释放

    // println!("{name}"); // ❌ 编译错误：name 已经失效
}
```

对简单的栈上数据，这种行为不太容易被感知；
但对 `String`、`Vec<T>` 这样的堆数据，这就非常关键。

Rust 的资源释放不只针对内存，也包括：

- 文件句柄
- socket
- 锁
- 数据库连接
- 任何实现了 `Drop` 的资源

所以“所有权”本质上不是只管内存，而是 Rust 的 **资源管理模型（RAII）**。

你也可以利用作用域主动控制资源释放时机：

```rust
fn main() {
    let data = String::from("important");

    {
        let borrowed = &data;
        println!("borrowed = {borrowed}");
    } // borrowed 在这里结束

    println!("data 仍然可用: {data}");
}
```

内层作用域经常被用来：

- 提前结束借用
- 提前释放资源
- 让后面的代码重新获得可变访问权

---

## 四、`String` 与 move 语义

### 4.1 为什么 `String` 会 move

先看代码：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {s2}");
    // println!("s1 = {s1}"); // ❌ 编译错误
}
```

很多初学者会问：

> “不就是把 `s1` 复制给 `s2` 吗？为什么 `s1` 不能用了？”

关键在于：`String` 管理的是堆内存。

可以把它粗略理解成下面这样：

| 变量 | 栈上保存的内容 | 堆上保存的内容 |
|---|---|---|
| `s1` | 指针、长度、容量 | `"hello"` |
| `let s2 = s1` 之后 | `s2` 接管这组元数据 | 还是同一份堆数据 |

如果 Rust 只是简单把这组“指针、长度、容量”复制一份：

- `s1` 和 `s2` 会同时指向同一块堆内存
- 作用域结束时就会出现 **重复释放**

为了避免这个问题，Rust 规定：

- `let s2 = s1;` 对 `String` 来说不是“普通复制”
- 而是 **所有权转移（move）**
- 转移之后，`s1` 立即失效，只有 `s2` 是合法 owner

所以更准确的理解不是“复制了一份 `String`”，而是：

> “把 `s1` 的所有权交给 `s2`。”

### 4.2 `clone()`：显式深拷贝

如果你确实想保留两份独立数据，就要显式调用 `.clone()`：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}");
    println!("s2 = {s2}");
}
```

这里发生的是：

- `s1` 保留原来的堆数据
- `s2` 分配新的堆空间，并复制内容
- 两者互不影响

`clone()` 的要点要记住两条：

1. 它是 **显式的** —— Rust 要你清楚知道“现在我要复制一份完整数据”
2. 它是 **有成本的** —— 尤其是字符串、向量、较大结构体，复制的是真实数据，不只是改个名字

一个很重要的经验法则：

| 场景 | 推荐 |
|---|---|
| 函数只是读取数据 | 优先借用 |
| 函数要修改调用者的数据 | 用 `&mut` |
| 函数确实要接管数据 | 传值 |
| 需要长期保留两份独立数据 | `clone()` 可能合理 |

### 4.3 move 不只发生在赋值

很多初学者以为 move 只会发生在：

```rust
let s2 = s1;
```

其实不是。move 还会出现在：

#### 1. 函数传参

```rust
fn take(s: String) {
    println!("{s}");
}

fn main() {
    let text = String::from("hello");
    take(text);
    // println!("{text}"); // ❌ 已经 move 进函数
}
```

#### 2. 放进容器

```rust
fn main() {
    let skill = String::from("Rust");
    let mut skills = Vec::new();

    skills.push(skill);
    // println!("{skill}"); // ❌ skill 已经 move 进 Vec
}
```

#### 3. 移动进结构体字段

```rust
struct User {
    name: String,
}

fn main() {
    let name = String::from("Alice");
    let user = User { name };

    println!("{}", user.name);
    // println!("{name}"); // ❌ name 已经 move 进 user
}
```

所以你应该建立一个更完整的直觉：

> 只要某个操作需要“接管拥有资源的值”，就可能发生 move。

---

## 五、`Copy` 类型

那为什么下面这种代码又可以？

```rust
fn main() {
    let x = 10;
    let y = x;

    println!("x = {x}");
    println!("y = {y}");
}
```

因为 `i32` 是 **Copy 类型**。

所谓 `Copy`，可以先简单理解为：

> 赋值、传参、返回时，直接按位复制一份，原值仍然有效。

常见 `Copy` 类型包括：

- 所有整数类型：`i32`、`u64` 等
- 浮点数：`f32`、`f64`
- 布尔：`bool`
- 字符：`char`
- 只包含 `Copy` 成员的元组
- 引用：`&T`

而这些通常 **不是** `Copy`：

- `String`
- `Vec<T>`
- `Box<T>`
- 大多数拥有堆数据、需要释放逻辑的类型

可以先记住这个判断：

> **固定大小、复制成本低、又不需要特殊释放的简单值，通常是 `Copy`。**

再补一个很容易忽略的点：

### 为什么引用 `&T` 也是 `Copy`

比如：

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = r1;
```

这里被复制的是 **引用本身**（也就是一个地址 / 借用关系），不是底层的 `String` 数据。

所以：

- `r1` 和 `r2` 都只是“只读借用”
- 真正拥有 `String` 的仍然是 `s`
- `s` 才是 owner，`r1` / `r2` 只是 reference

---

## 六、函数中的所有权

函数参数本质上也和赋值一样，会发生 move 或 copy。

### 6.1 函数签名就是资源契约

你可以把函数签名理解成：

> “调用这个函数时，数据的所有权和访问权限会怎么变化？”

最常见的三种签名：

| 签名 | 含义 | 调用后原值还能用吗 |
|---|---|---|
| `fn foo(s: String)` | 接管所有权 | 不能 |
| `fn foo(s: &str)` | 只读借用 | 能 |
| `fn foo(s: &mut String)` | 可变借用 | 能，但借用结束前不能冲突访问 |

例如：

```rust
fn consume(text: String) {
    println!("consume: {text}");
}

fn read(text: &str) {
    println!("read: {text}");
}

fn append(text: &mut String) {
    text.push('!');
}
```

这三个函数看起来只是参数写法不同，但语义完全不同：

- `consume`：函数拿走数据
- `read`：函数只看，不拿走
- `append`：函数可以改，但不接管所有权

### 6.2 返回值也参与所有权转移

返回值同样会影响 ownership：

```rust
fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(s: String) -> String {
    s
}
```

调用时：

```rust
fn main() {
    let s1 = gives_ownership();
    let s2 = takes_and_gives_back(s1);

    println!("s2 = {s2}");
}
```

这里发生的事情是：

- `gives_ownership()` 产生一个新的 `String`，把所有权交给调用者
- `takes_and_gives_back(s1)` 先接管 `s1`，再把所有权转交给 `s2`

所以：

- 赋值会 move / copy
- 函数传参会 move / copy
- 函数返回值也会 move / copy

### 6.3 怎么决定用 `T`、`&T` 还是 `&mut T`

这是初学 Rust 最常见、也最实用的判断题。

| 需求 | 推荐签名 |
|---|---|
| 只读字符串 | `&str` |
| 只读一段序列 | `&[T]` |
| 修改调用者数据 | `&mut T` |
| 接管并持有数据 | `T` |
| 需要返回新拥有的数据 | 返回 `T` |

一个非常实用的经验法则：

1. **默认优先借用**
2. **需要修改时用 `&mut`**
3. **确实要接管时才传值**
4. **确实需要两份独立数据时才 `clone()`**

---

## 七、引用（reference）与借用（borrowing）

如果每次把值传给函数都会失去所有权，那写程序会很麻烦。

Rust 的解决方案是：**借用（borrowing）**。

借用的核心思想是：

> 我可以临时使用你的数据，但我不把它拿走。

### 7.1 不可变引用

用 `&` 创建引用：

```rust
fn calculate_length(s: &str) -> usize {
    s.len()
}

fn main() {
    let text = String::from("hello");
    let len = calculate_length(&text);

    println!("'{text}' 的长度是 {len}");
}
```

这里：

- `text` 仍然拥有字符串
- `calculate_length` 只是借用它
- 函数结束后，借用失效，但原值仍然存在

所以：

- `text` 是 owner
- `&text` 是 reference
- 这个过程叫 borrowing

### 7.2 多个不可变引用为什么可以同时存在

下面这种写法是允许的：

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("r1 = {r1}, r2 = {r2}");
}
```

原因很简单：

- `r1` 只读
- `r2` 也只读
- 两个只读操作不会互相破坏数据

你可以把它理解成：

> “多人围观一本书”没问题，前提是大家都只是看，不去改。

### 7.3 `&String` 和 `&str` 的区别

这是新手非常容易卡住的一点。

| 类型 | 含义 | 适合当函数参数吗 |
|---|---|---|
| `String` | 拥有整段字符串 | 只有在函数需要接管数据时 |
| `&String` | 借用一个 `String` | 偶尔可以，但通常不够通用 |
| `&str` | 借用一段字符串切片 | **最常见、最推荐** |

为什么函数参数通常优先写 `&str`？

因为它更通用：

```rust
fn print_text(text: &str) {
    println!("{text}");
}

fn main() {
    let owned = String::from("hello");
    let literal = "rust";

    print_text(&owned);
    print_text(literal);
}
```

`&str` 可以接收：

- `String` 的借用
- 字符串字面量
- 某个字符串的切片

而 `&String` 只能接收“一个 `String` 的借用”，范围更窄。

所以经验上可以先记住：

> **只读字符串参数，优先写 `&str`。**

---

## 八、可变引用（`&mut`）

如果不仅想看，还想修改数据，就要用 **可变引用**：

```rust
fn change(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut text = String::from("hello");
    change(&mut text);
    println!("{text}"); // hello world
}
```

这里有两个关键点：

1. 原变量必须是 `mut`
2. 借用时也要写 `&mut`

也就是说：

- `let mut text = ...` 表示“绑定本身允许被修改”
- `&mut text` 表示“现在把它以独占、可写的方式借出去”

可变借用为什么要求独占？

因为 Rust 要保证：

> 当有人在修改一份数据时，不能同时还有别的地方以冲突的方式读取或修改它。

你可以把它理解成：

- 多个人同时只读，问题不大
- 一个人独占读写，也没问题
- 但“有人在看、另一个人在改”就容易出事

---

## 九、借用规则（borrow rules）

借用规则是所有权系统的重点，也是 Rust 初学者最常遇到编译错误的地方。

### 9.1 规则总结表

在任意给定时刻，下面两种状态只能选一种：

1. **可以有任意多个不可变引用**
2. **或者只能有一个可变引用**

总结成表格就是：

| 状态 | 是否允许 |
|---|---|
| 多个 `&T` 同时存在 | 允许 |
| 一个 `&mut T` 单独存在 | 允许 |
| `&T` 和 `&mut T` 同时活跃 | 不允许 |
| 两个 `&mut T` 同时活跃 | 不允许 |

### 9.2 如何读 Rust 的借用报错

Rust 的报错看起来吓人，但信息通常非常具体。常见关键词有：

#### 1. `moved here`

表示：

> 这个值的所有权已经在这里被转走了。

通常你应该检查：

- 是不是传值进了函数？
- 是不是赋值给了新变量？
- 是不是 push 进了容器？

#### 2. `borrow later used here`

表示：

> 某个借用在后面还要继续使用，所以现在还不能进行冲突操作。

通常你应该检查：

- 能不能把最后一次使用提前？
- 能不能缩小借用范围？
- 能不能先读后改，而不是交错进行？

#### 3. `cannot borrow as mutable because it is also borrowed as immutable`

表示：

> 你已经有只读借用了，但又想在借用仍然活跃时进行可变借用。

这时一般不是立刻去 `clone()`，而是先考虑：

- 调整语句顺序
- 提前结束只读借用
- 把逻辑拆成更清楚的几个步骤

### 9.3 非词法生命周期（NLL）直觉

早期学习时，很多人会以为借用一定要等到整个 `{}` 作用域结束才算结束。

Rust 现在的分析更聪明一些。可以先建立这个直觉：

> 编译器更关注“这个借用最后一次被使用在什么时候”，而不只是变量名还在不在。

例如：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    println!("r1 = {r1}"); // r1 在这里最后一次使用

    let r2 = &mut s;        // ✅ 现在允许可变借用
    r2.push_str(" world");

    println!("s = {s}");
}
```

虽然 `r1` 这个名字看起来还在作用域里，但它已经不再被使用，所以借用关系可以视为结束。

这就是为什么有些代码你“感觉应该报错”，但其实 Rust 能接受。

### 9.4 不要把 clone 当成默认解法

这是非常重要的一条经验。

很多新手一看到借用冲突，就会写：

```rust
let backup = data.clone();
```

有时候这样确实能过编译，但未必是好解法。因为你可能只是：

- 没有缩小借用范围
- 参数类型写得太窄
- 语句顺序不合理

正确思路通常是按顺序排查：

1. 这个函数真的需要拿走所有权吗？
2. 能不能改成借用？
3. 能不能改成 `&str` / `&[T]` 这种更通用参数？
4. 能不能把借用的最后一次使用提前？
5. 如果真的需要两份独立数据，再 `clone()`

---

## 十、切片（slice）

切片（slice）本质上也是一种 **借用**。

它表示：

> 我不拥有整个集合，但我借用其中一段。

### 10.1 字符串切片 `&str`

例如：

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello = {hello}");
    println!("world = {world}");
}
```

这里的 `hello` 和 `world` 都不是新的字符串拥有者，而是对 `s` 某一段内容的借用。

更常见的写法是把函数参数写成 `&str`：

```rust
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

它有两个重要优点：

- 能接受 `String`
- 也能接受字符串字面量 `&str`

### 10.2 数组 / 向量切片 `&[T]`

切片不只存在于字符串。

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let part = &numbers[1..4];

    println!("{:?}", part); // [20, 30, 40]
}
```

所以：

- `&str` 是字符串切片
- `&[i32]` 是一段整数切片
- 切片的本质都是“借用一段连续数据”

写函数时，通常也更推荐：

```rust
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}
```

而不是：

```rust
fn sum(nums: &Vec<i32>) -> i32 {
    nums.iter().sum()
}
```

原因是 `&[i32]` 更通用：

- 可以接收数组切片
- 可以接收 `Vec<i32>` 的切片
- 可以接收整个数组或整个向量的借用视图

### 10.3 UTF-8 边界为什么要特别小心

Rust 的 `String` 是 **UTF-8 编码**。

这意味着：

- `"hello"` 这种纯 ASCII 字符串，一个字符通常占 1 字节
- `"你"`、`"好"` 这样的中文字符，往往占多个字节

所以这种写法要特别小心：

```rust
let s = String::from("你好");
// let first = &s[0..1]; // ❌ 不是合法的 UTF-8 边界
```

为什么不行？

因为 `[0..1]` 只截取了一个字节，而一个中文字符通常要多个字节才能组成合法 UTF-8。

所以面对自然语言文本时，通常应该考虑：

- `chars()`
- `char_indices()`
- 或更高层的字符串处理方式

一个很实用的经验是：

> **对 ASCII 文本按字节切片通常没问题；对 Unicode 文本，要先想“字符边界”而不是“字节边界”。**

---

## 十一、悬垂引用（dangling reference）

悬垂引用就是：

> 一个引用指向了一块已经被释放的内存。

在很多语言里，这会成为严重 bug；在 Rust 里，编译器会直接禁止它出现。

错误示例：

```rust
// ❌ 不能这样写
fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // s 在这里被释放，但返回了它的引用
```

为什么不行？

- `s` 是局部变量
- 函数结束时它就会被 drop
- 返回 `&s` 相当于把一个已经失效的地址交给外部

正确方式通常有两类：

### 1. 直接返回所有权

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

### 2. 从输入中借用，再把借用返回出去

```rust
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

这里返回的引用不是指向一个局部新建值，而是指向调用者传进来的那份数据的一部分。

你现在可以先建立这个直觉：

> 想返回引用，必须保证它引用的数据在函数返回后依然有效。

至于更系统的“生命周期标注语法”，通常会在后续专门章节展开。

---

## 十二、实战中的 API 设计

学所有权不只是为了看懂报错，更重要的是学会设计合理的函数和方法签名。

### 12.1 字符串参数优先考虑 `&str`

如果函数只是读字符串，通常优先写：

```rust
fn print_name(name: &str) {
    println!("{name}");
}
```

而不是：

```rust
fn print_name(name: String) {
    println!("{name}");
}
```

也不是：

```rust
fn print_name(name: &String) {
    println!("{name}");
}
```

推荐 `&str` 的原因：

- 不拿走所有权
- 比 `&String` 更通用
- 调用方可以传字面量、`String`、切片

### 12.2 集合参数优先考虑切片

如果函数只读一段数据，通常优先：

```rust
fn average(nums: &[i32]) -> f64 {
    let sum: i32 = nums.iter().sum();
    sum as f64 / nums.len() as f64
}
```

而不是：

```rust
fn average(nums: &Vec<i32>) -> f64 {
    let sum: i32 = nums.iter().sum();
    sum as f64 / nums.len() as f64
}
```

理由和 `&str` 类似：`&[T]` 更通用。

### 12.3 方法接收者：`self`、`&self`、`&mut self`

在结构体方法里，所有权设计常常体现在接收者上：

| 写法 | 含义 |
|---|---|
| `self` | 消费对象，接管整个实例 |
| `&self` | 只读借用 |
| `&mut self` | 可变借用，可以修改实例 |

例如：

```rust
struct User {
    name: String,
}

impl User {
    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    fn into_name(self) -> String {
        self.name
    }
}
```

这三个方法就分别代表三种完全不同的契约：

- `name(&self)`：只读，不修改，也不接管
- `rename(&mut self, ...)`：可写，但对象还归调用者所有
- `into_name(self)`：消费整个对象，把内部的 `String` 拿出来

---

## 十三、常见错误与易错点

### 1. move 之后继续使用原变量

```rust
let s1 = String::from("hello");
let s2 = s1;
// println!("{s1}"); // ❌ 已经 move 了
```

要问自己：是不是所有权已经转给了别的变量、函数、容器或结构体？

### 2. 把 `clone()` 当成默认补丁

```rust
let backup = data.clone();
```

有时这是合理的，但很多时候只是掩盖了：

- 参数类型设计不对
- 借用范围太大
- 语句顺序不合理

### 3. 误把 `&String` 当成最佳参数类型

如果函数只是读字符串，通常 `&str` 更好。

### 4. 不理解借用结束时机

有些人以为只要变量名还在，借用就一定还活跃。其实 Rust 更关注“最后一次使用点”。

### 5. 同时创建多个可变引用

```rust
let r1 = &mut s;
// let r2 = &mut s; // ❌
```

可变借用要求独占。

### 6. 已有不可变借用时，又尝试可变借用

```rust
let r1 = &s;
// let r2 = &mut s; // ❌
```

先结束只读借用，再进行可变借用。

### 7. 对 UTF-8 字符串按字节乱切

```rust
let s = String::from("你好");
// let first = &s[0..1]; // ❌
```

处理 Unicode 文本时，请优先考虑字符边界。

### 8. 返回局部变量的引用

```rust
// fn bad() -> &String {
//     let s = String::from("hello");
//     &s
// }
```

局部变量在函数结束时就失效，不能把它的引用返回出去。

### 9. 不知道什么时候该传值

如果函数的职责就是“接管并保存”数据，那传值反而是正确设计，不要一味想用借用。

---

## 十四、综合练习

你可以尝试自己完成下面这些练习，真正把所有权和借用练熟。

### 练习 1：计算字符串长度

要求：

- 写一个函数 `calculate_length`
- 参数不能拿走字符串所有权
- 返回字符串长度

提示：优先用 `&str`

### 练习 2：追加后缀

要求：

- 写一个函数 `append_suffix(name: &mut String, suffix: &str)`
- 在原字符串末尾追加后缀
- 调用后原变量仍可继续使用

### 练习 3：求和函数

要求：

- 写一个函数 `sum(nums: &[i32]) -> i32`
- 同时让它能接收数组切片和 `Vec<i32>` 切片

### 练习 4：结构体方法与 ownership

定义：

```rust
struct User {
    name: String,
}
```

然后分别实现：

- `fn name(&self) -> &str`
- `fn rename(&mut self, new_name: &str)`
- `fn into_name(self) -> String`

思考三者分别属于：

- 只读借用
- 可变借用
- 消费所有权

### 练习 5：判断该借用、传值还是 clone

分别思考下面这些场景的最佳选择：

1. 只是打印一段文本
2. 需要修改一段已有文本
3. 需要把文本保存进结构体长期持有
4. 需要在两个地方同时长期保留独立副本

### 练习 6：安全处理 UTF-8 字符串

要求：

- 不直接写 `&s[0..n]`
- 尝试使用 `char_indices()` 找到前几个字符的边界
- 截取一个中文字符串的前 2 个字符

这题的重点不是写得多短，而是理解：

> 字符串切片按的是字节边界，但自然语言里的“字符”并不总是一字节。

---

## 要点总结

### 所有权核心

1. 每个值都有一个 owner
2. 同一时刻只能有一个 owner
3. owner 离开作用域时，值会被自动 drop
4. 所有权管理的不只是内存，而是更广义的资源生命周期

### move / clone / Copy

1. `String`、`Vec<T>` 这类拥有资源的类型，赋值时通常发生 move
2. move 是“所有权转移”，不是“底层数据复制两份”
3. `clone()` 是显式深拷贝，有真实成本
4. `Copy` 类型赋值后原值仍然有效
5. 引用 `&T` 本身通常是 `Copy`，但被引用的数据并没有被复制

### 借用与借用规则

1. `&T` 表示只读借用
2. `&mut T` 表示可变借用
3. 多个只读借用可以同时存在
4. 可变借用必须独占
5. Rust 更关注借用最后一次被使用的位置，而不只是花括号作用域

### slice 与 API 设计

1. `&str` 是字符串切片，本质上也是借用
2. `&[T]` 是一段连续元素的借用视图
3. 只读字符串参数优先考虑 `&str`
4. 只读序列参数优先考虑 `&[T]`
5. UTF-8 字符串切片要特别注意字符边界

### 最实用的判断准则

1. **默认优先借用**
2. **需要修改时用 `&mut`**
3. **需要接管数据时才传值**
4. **真的需要两份独立数据时再 `clone()`**
5. **看到借用错误，先调整设计和顺序，不要第一反应就复制一份**

---

> **下一步：** 学完所有权之后，你会发现 Rust 后续很多语法和限制都突然变得合理。
> 结构体、枚举、模式匹配、生命周期、trait、并发，都会建立在这一章的基础之上。