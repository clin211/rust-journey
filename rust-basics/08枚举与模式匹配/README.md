# 枚举与模式匹配 (enums & pattern matching)

> 枚举（Enum）是 Rust 把"一组互斥的可能性"装进同一种类型的核心手段。
> 它和上一章的结构体（Struct）一起，构成了 Rust 类型系统的两根支柱：
>
> - **结构体表达 AND**：所有字段同时存在
> - **枚举表达 OR**：变体里只有一个会成立
>
> 而 `match` / `if let` / `while let` / `let else` 这一组**模式匹配**语法，
> 是消费枚举（以及更广泛的复合数据）最自然、最安全的方式。

在上一章 `07结构体` 中，我们看到了"散装变量 → 结构化数据"的进化：

```rust
fn create_user() -> (u64, String, String, bool, u32) { /* ... */ }   // ❌ 散装
struct User { id: u64, name: String, email: String, active: bool, age: u32 }  // ✅ 收编
```

但结构体只能表达"全字段同时存在"。一旦你想说"只有这几种情况之一"，就立刻碰到墙：

```rust
// ❌ 用 struct 模拟"OR" — 字段约定靠人记，编译器无能为力
struct Response {
    is_ok: bool,
    body: Option<String>,    // 只在 is_ok = true 时有意义
    err_code: Option<i32>,   // 只在 is_ok = false 时有意义
}
// 你可以构造出 { is_ok: true, body: None, err_code: Some(500) } 这种自相矛盾的值
// 编译器不会拦截 ⚠️
```

枚举从根上解决这个问题：

```rust
enum Response {
    Ok(String),
    Err(i32),
}
// "成功 + 数据" 和 "失败 + 错误码" 永远不会同时存在或同时缺席
// 编译器全程保护
```

本章你将彻底掌握：

- 三种变体形态：单元 / 元组 / 结构体
- 为枚举实现方法、关联函数、派生宏
- `match` 表达式与穷尽性检查
- 完整的模式语法：字面量 / 范围 / 多模式 / 守卫 / `@` 绑定 / 解构 / 嵌套
- `if let` / `let else` / `while let` 的现代写法
- **`Option<T>`**：替代 null 的标准答案 + 完整组合子 API
- **`Result<T, E>`**：可恢复错误处理的基石 + `?` 运算符
- **状态机 + 命令模式**：用 enum 建模业务流程的实战套路
- **递归枚举**：链表 / JSON Value / 表达式树的标准写法
- **内存布局**：discriminant / Niche / NPO，证明 Rust 的"零成本抽象"
- 实战：用 enum + match 实现计算器（加减乘除 + 错误处理）

---

## 示例文件

| 文件 | 主题 | 运行 |
|------|------|------|
| `examples/01_enum_basics.rs` | 单元变体、命名空间、enum vs struct（OR vs AND） | `cargo run --example 01_enum_basics` |
| `examples/02_enum_with_data.rs` | 三种变体形态、Message 经典例子、嵌套 enum/Vec | `cargo run --example 02_enum_with_data` |
| `examples/03_enum_methods.rs` | impl 块、关联函数、`#[default]`、Display 等派生 | `cargo run --example 03_enum_methods` |
| `examples/04_match_basics.rs` | match 表达式、穷尽性、`_` / 命名兜底、表达式赋值 | `cargo run --example 04_match_basics` |
| `examples/05_match_patterns.rs` | 字面量 / 范围 / `\|` / `..` / 守卫 / `@` / `ref` | `cargo run --example 05_match_patterns` |
| `examples/06_destructuring.rs` | 解构 enum / struct / tuple / 嵌套 / 数组与切片 | `cargo run --example 06_destructuring` |
| `examples/07_if_let_while_let.rs` | if let / if let 链 / let else / while let | `cargo run --example 07_if_let_while_let` |
| `examples/08_option.rs` | Option<T> 全 API：map / and_then / unwrap_or / take / `?` | `cargo run --example 08_option` |
| `examples/09_result_intro.rs` | Result<T,E>：组合子、`?`、与 Option 互转 | `cargo run --example 09_result_intro` |
| `examples/10_state_machine.rs` | 状态机：红绿灯 / 订单（事件驱动）/ Type-State | `cargo run --example 10_state_machine` |
| `examples/11_recursive_enum.rs` | 递归枚举：Cons 链表、JSON Value、AST + 求值 | `cargo run --example 11_recursive_enum` |
| `examples/12_calculator.rs` | 综合练习：计算器（加减乘除 + 错误处理 + 单元测试） | `cargo run --example 12_calculator` / `cargo test --example 12_calculator` |
| `examples/13_command_pattern.rs` | 命令模式 / Action / Route：reducer + 穷尽 match | `cargo run --example 13_command_pattern` |
| `examples/14_memory_layout.rs` | discriminant、最大变体、Niche / NPO、`#[repr]` | `cargo run --example 14_memory_layout` |
| `examples/15_advanced_patterns.rs` | 可反驳性、深嵌套、matches!、`ref mut`、unreachable | `cargo run --example 15_advanced_patterns` |

`src/main.rs` 用一个"餐厅订单系统"把全章关键概念串成一个最小完整流程。

---

## 目录

- [枚举与模式匹配 (enums \& pattern matching)](#枚举与模式匹配-enums--pattern-matching)
  - [示例文件](#示例文件)
  - [目录](#目录)
  - [一、为什么需要枚举](#一为什么需要枚举)
    - [1.1 用结构体表达"或"的痛苦](#11-用结构体表达或的痛苦)
    - [1.2 枚举：一组互斥的可能性](#12-枚举一组互斥的可能性)
    - [1.3 枚举 vs 其它语言](#13-枚举-vs-其它语言)
    - [1.4 enum 与 struct 的取舍](#14-enum-与-struct-的取舍)
  - [二、定义与变体](#二定义与变体)
    - [2.1 最简枚举：单元变体](#21-最简枚举单元变体)
    - [2.2 变体的命名空间](#22-变体的命名空间)
    - [2.3 显式 discriminant 与 `as` 转换](#23-显式-discriminant-与-as-转换)
  - [三、带数据的枚举](#三带数据的枚举)
    - [3.1 三种变体形态](#31-三种变体形态)
    - [3.2 经典例子：Message](#32-经典例子message)
    - [3.3 嵌套：变体里放 enum / Vec / Box](#33-嵌套变体里放-enum--vec--box)
    - [3.4 大小取决于"最大变体"](#34-大小取决于最大变体)
  - [四、为枚举实现方法](#四为枚举实现方法)
    - [4.1 `impl` 块](#41-impl-块)
    - [4.2 关联函数（构造器、parse 风格）](#42-关联函数构造器parse-风格)
    - [4.3 多个 `impl` 块按主题分组](#43-多个-impl-块按主题分组)
    - [4.4 派生宏总览](#44-派生宏总览)
    - [4.5 给 enum 一个默认变体（`#[default]`）](#45-给-enum-一个默认变体default)
  - [五、match 表达式](#五match-表达式)
    - [5.1 match 是表达式](#51-match-是表达式)
    - [5.2 穷尽性检查（exhaustiveness）](#52-穷尽性检查exhaustiveness)
    - [5.3 `_` 通配符 vs 命名兜底](#53-_-通配符-vs-命名兜底)
    - [5.4 多语句臂](#54-多语句臂)
    - [5.5 match vs if/else if 链](#55-match-vs-ifelse-if-链)
  - [六、模式语法全集](#六模式语法全集)
    - [6.1 字面量模式](#61-字面量模式)
    - [6.2 多模式 `\|`](#62-多模式-)
    - [6.3 范围 `..=` 与 `..`](#63-范围--与-)
    - [6.4 通配 `_` 与忽略 `..`](#64-通配-_-与忽略-)
    - [6.5 命名变量绑定（小心 shadowing）](#65-命名变量绑定小心-shadowing)
    - [6.6 守卫 `if` 子句](#66-守卫-if-子句)
    - [6.7 `@` 绑定](#67--绑定)
    - [6.8 `ref` / `ref mut`（与现代 `&` 语法对照）](#68-ref--ref-mut与现代--语法对照)
    - [6.9 解构 enum / struct / tuple / 嵌套](#69-解构-enum--struct--tuple--嵌套)
  - [七、可反驳性 vs 不可反驳性](#七可反驳性-vs-不可反驳性)
  - [八、模式可用的位置](#八模式可用的位置)
  - [九、`if let` / `let else` / `while let`](#九if-let--let-else--while-let)
    - [9.1 `if let`：只关心一个变体](#91-if-let只关心一个变体)
    - [9.2 `if let ... else`：再加一条 fallback](#92-if-let--else再加一条-fallback)
    - [9.3 `if let` 链（Rust 1.88+）](#93-if-let-链rust-188)
    - [9.4 `let else`：早返回 + 把值"摊"出来](#94-let-else早返回--把值摊出来)
    - [9.5 `while let`：循环消费](#95-while-let循环消费)
    - [9.6 `if let` 与 `match` 的取舍](#96-if-let-与-match-的取舍)
  - [十、`Option<T>`：替代 null 的标准答案](#十optiont替代-null-的标准答案)
    - [10.1 为什么 Rust 没有 null](#101-为什么-rust-没有-null)
    - [10.2 创建 Option](#102-创建-option)
    - [10.3 取值：unwrap 系列](#103-取值unwrap-系列)
    - [10.4 组合子：map / and\_then / filter / or](#104-组合子map--and_then--filter--or)
    - [10.5 引用转换：as\_ref / as\_mut / cloned / copied](#105-引用转换as_ref--as_mut--cloned--copied)
    - [10.6 就地变换：take / replace / get\_or\_insert](#106-就地变换take--replace--get_or_insert)
    - [10.7 `?` 运算符（Option 版本）](#107--运算符option-版本)
  - [十一、`Result<T, E>` 简介](#十一resultt-e-简介)
    - [11.1 与 Option 的关系](#111-与-option-的关系)
    - [11.2 常用 API](#112-常用-api)
    - [11.3 `?` 运算符 + `From` 自动转换](#113--运算符--from-自动转换)
    - [11.4 Option ↔ Result 互转](#114-option--result-互转)
  - [十二、enum 与状态机](#十二enum-与状态机)
    - [12.1 自动状态机：红绿灯](#121-自动状态机红绿灯)
    - [12.2 事件驱动：订单状态](#122-事件驱动订单状态)
    - [12.3 带数据的状态机：网络连接](#123-带数据的状态机网络连接)
    - [12.4 Type-State 风格：编译期拒绝非法转换](#124-type-state-风格编译期拒绝非法转换)
  - [十三、递归枚举](#十三递归枚举)
    - [13.1 为什么需要 `Box`](#131-为什么需要-box)
    - [13.2 链表](#132-链表)
    - [13.3 JSON Value](#133-json-value)
    - [13.4 表达式树（AST）+ 递归求值](#134-表达式树ast--递归求值)
  - [十四、命令模式 / Action / Route](#十四命令模式--action--route)
    - [14.1 state 用 struct，command 用 enum](#141-state-用-structcommand-用-enum)
    - [14.2 reducer 与穷尽 match](#142-reducer-与穷尽-match)
    - [14.3 路由（Subcommand）](#143-路由subcommand)
  - [十五、实战：用 enum + match 实现一个计算器](#十五实战用-enum--match-实现一个计算器)
  - [十六、枚举内存布局](#十六枚举内存布局)
    - [16.1 discriminant：每个实例都自带一个 tag](#161-discriminant每个实例都自带一个-tag)
    - [16.2 enum 的总大小 = max(变体) + tag + padding](#162-enum-的总大小--maxvariant--tag--padding)
    - [16.3 Niche / Null Pointer Optimization (NPO)](#163-niche--null-pointer-optimization-npo)
    - [16.4 `#[repr]` 全景](#164-repr-全景)
  - [十七、enum 在真实世界](#十七enum-在真实世界)
    - [17.1 标准库里的高频 enum](#171-标准库里的高频-enum)
    - [17.2 生态库里的典型例子](#172-生态库里的典型例子)
    - [17.3 enum 的八种典型角色](#173-enum-的八种典型角色)
  - [十八、常见错误与易错点](#十八常见错误与易错点)
  - [十九、API 设计准则](#十九api-设计准则)
  - [二十、综合练习](#二十综合练习)
  - [要点总结](#要点总结)

---

## 一、为什么需要枚举

### 1.1 用结构体表达"或"的痛苦

假设你要表达 HTTP 响应：要么成功（带 body），要么失败（带错误码）。
用结构体写出来是这样：

```rust
struct Response {
    is_ok: bool,
    body: Option<String>,    // 只在 is_ok=true 时有意义
    err_code: Option<i32>,   // 只在 is_ok=false 时有意义
}
```

痛点：

- **字段约束靠人记**：编译器不知道 `is_ok=true` 时 `err_code` 应该是 `None`
- **可以构造矛盾值**：`{ is_ok: true, body: None, err_code: Some(500) }` 完全合法
- **每次访问都要 unwrap**：调用方要写 `if r.is_ok { r.body.unwrap() }`
- **新增情况要改全家**：将来加一个 `Pending` 状态，就得加新字段并到处改

直观对比：

```text
  用 struct 模拟"OR"（不舒服）:        用 enum 表达"OR"（舒服）:
  ┌─────────────────────────────┐      ┌────────────────────────┐
  │ struct Response {           │      │ enum Response {        │
  │   is_ok:    bool,           │      │   Ok(String),          │
  │   body:     Option<String>, │      │   Err(i32),            │
  │   err_code: Option<i32>,    │      │ }                      │
  │ }                           │      └────────────────────────┘
  └─────────────────────────────┘             ↑
   ↑                                          每个变体都"自带数据",
   字段之间的约束靠人记                          编译器全程保护

  非法状态(矛盾值)               非法状态(根本不可能存在)
   { is_ok: true,                 — 编译器不让你构造 Response 的
     body: None,                    "既 Ok 又 Err"或"既不是 Ok 也不是 Err"
     err_code: Some(500) }          的实例
```

### 1.2 枚举：一组互斥的可能性

枚举用一种类型把"一组互斥的可能性"装进来：

```rust
enum Response {
    Ok(String),
    Err(i32),
}

fn handle(r: Response) -> String {
    match r {
        Response::Ok(body) => format!("ok: {body}"),
        Response::Err(code) => format!("err: {code}"),
    }
    // 编译器强制处理两种情况，将来加 Pending 变体时所有 match 都会编译报错
    // —— "悄悄遗漏"在 Rust 里不存在
}
```

这就是 enum 在 Rust 里被称为 **sum type / 代数数据类型 (ADT)** 的原因：
"sum" 指的是"OR 关系"——一个值的所有可能性是各变体之和。

### 1.3 枚举 vs 其它语言

| 语言 | 类比概念 | 与 Rust enum 最大不同 |
|------|----------|----------------------|
| C | `enum` | 只能是一组整数常量；不能携带不同形状的数据 |
| C | `union` | 没有 tag，运行时不知道当前是哪个分支，类型不安全 |
| TypeScript | union types | 运行时仍是动态的，没有完整的穷尽性检查 |
| Java | sealed class / class hierarchy | 通过抽象类 + 子类模拟，繁琐且打开扩展性 |
| Go | `interface{}` + 类型断言 | 运行时分发，没有编译期穷尽检查 |
| Swift | `enum` (associated values) | 几乎一一对应，是 Swift 学 Rust 学得最像的部分 |
| Haskell / OCaml | `data` / `type` | 一脉相承，Rust enum 在底层和 ADT 完全等价 |

Rust 的 enum 站在了 ADT 这一脉上，又把内存布局做到 C 一样紧凑（详见第十六章）。

### 1.4 enum 与 struct 的取舍

最实用的判断准则：

| 你的需求 | 推荐 |
|---------|------|
| 一组字段**同时存在** | `struct` |
| 一组**互斥的可能性** | `enum` |
| 既要互斥的种类，又要同时存在的字段 | enum 套 struct（或 struct 里嵌 enum） |
| 状态机的"状态" | enum |
| 状态机的"事件" | enum |
| 配置 / DTO（字段都是确定有的） | struct |
| 表达 "可能没有" | `Option<T>`（enum） |
| 表达 "可能失败" | `Result<T, E>`（enum） |

**记住**：

> struct 表达 **AND**，enum 表达 **OR**；
> 两者搭配，足以建模绝大多数业务场景。

完整的运行实例见 `examples/01_enum_basics.rs`。

---

## 二、定义与变体

### 2.1 最简枚举：单元变体

最朴素的形态：只列举几种可能性，每个变体本身不带数据。

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

它的用途等同于"有限的常量集合"，但比一堆 `const` 更安全：

- **类型唯一**：`Direction` 不会和 `i32` / `Status` 互相误用
- **穷尽性**：写 match 时编译器会强制检查每个变体
- **可扩展**：将来给 `Direction` 加 `UpLeft`，所有 match 都会编译报错来提醒你

构造与使用：

```rust
let d = Direction::Up;

let opposite = match d {
    Direction::Up => Direction::Down,
    Direction::Down => Direction::Up,
    Direction::Left => Direction::Right,
    Direction::Right => Direction::Left,
};
```

### 2.2 变体的命名空间

每个变体都"住"在它的 enum 命名空间里——必须写 `Direction::Up`，不能裸写 `Up`：

```rust
enum UserStatus { Active, Inactive }
enum ServerStatus { Active, Inactive, Crashed }

let u = UserStatus::Active;
let s = ServerStatus::Active;
// u 和 s 看上去都是 "Active"，但它们是两种独立类型，不能互相赋值
```

如果你确实想偷懒，可以在用之前 `use`：

```rust
use Direction::*;
let d = Up;     // 等价于 Direction::Up
```

但**生产代码不建议**——`Direction::Up` 一眼就能看出"这个 Up 来自哪个枚举"，
裸写 `Up` 会让阅读者要回查作用域。

### 2.3 显式 discriminant 与 `as` 转换

不带数据的枚举可以像 C 一样指定整数值，方便和外部协议 / FFI 对齐：

```rust
#[repr(u16)]                  // 强制 tag 占 u16
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
    Unknown = 0,
}

fn http_code(s: HttpStatus) -> u16 {
    s as u16                   // 单元变体可以用 `as` 直接转成数字
}

assert_eq!(http_code(HttpStatus::Ok), 200);
assert_eq!(http_code(HttpStatus::NotFound), 404);
```

⚠️ 注意：

- `as` 只对**没有数据的变体**有效；带数据的变体不能直接 `as`
- `#[repr(u8/u16/u32/i32/...)]` 用来精确控制 tag 的存储宽度（FFI / 协议必备）

更多内存布局细节见第十六章。

完整运行实例见 `examples/01_enum_basics.rs`、`examples/14_memory_layout.rs`。

---

## 三、带数据的枚举

### 3.1 三种变体形态

变体有三种形态，可以在同一个 enum 里混搭：

```rust
enum Message {
    Quit,                                 // 单元变体：不带数据
    Echo(String),                         // 元组变体：一个或多个匿名字段
    Move { x: i32, y: i32 },              // 结构体变体：具名字段
    ChangeColor(u8, u8, u8),              // 元组变体：RGB
}
```

| 形态 | 语法 | 解构方式 |
|------|------|---------|
| 单元 | `Quit` | `Message::Quit => ...` |
| 元组 | `Echo(String)` | `Message::Echo(s) => ...` |
| 结构体 | `Move { x, y }` | `Message::Move { x, y } => ...` |

### 3.2 经典例子：Message

来自《The Rust Programming Language》的经典例子：

```rust
fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("用户退出"),
        Message::Echo(text) => println!("回声: {text}"),
        Message::Move { x, y } => println!("移动到 ({x}, {y})"),
        Message::ChangeColor(r, g, b) => println!("颜色 #{r:02x}{g:02x}{b:02x}"),
    }
}
```

如果不用 enum，要表达同样的"互斥 + 各带各的字段"，就得搞一个肿胀的 struct：

```rust
struct MessageStruct {
    kind: u8,                 // 用一个 tag 区分到底是哪种
    echo_text: Option<String>,
    x: Option<i32>,
    y: Option<i32>,
    r: Option<u8>, g: Option<u8>, b: Option<u8>,
}
```

缺点：

- 每种 message 只用到部分字段，剩下全是 `None`
- "kind=Move 时 x,y 必须 Some" 这个约束编译器无法保证
- match 时要写一堆 `if let Some(...)` 检查

enum 把这些痛苦从源头消除。

### 3.3 嵌套：变体里放 enum / Vec / Box

变体里能放任何类型——包括其它 enum、`Vec<T>`、`HashMap`、甚至 `Box<Self>`（递归枚举，见 §13）。

```rust
enum LogLevel { Info, Warn, Error }

enum AppEvent {
    UserSignedIn { user_id: u64, ip: IpAddr },
    UserSignedOut(u64),
    ConfigChanged {
        key: String,
        old: Option<String>,    // Option<String> 也是个 enum
        new: Option<String>,
    },
    Log(LogLevel, String),       // 嵌套另一个 enum
    Errors(Vec<String>),         // 一次报告一组错误
}
```

这种"事件 / 命令 / 消息"建模在工程里出镜率极高（见 §14 命令模式）。

### 3.4 大小取决于"最大变体"

一个 enum 实例运行时只持有"它实际成立的那个变体"的数据。
但 enum 在编译时已经决定了整体大小——取决于**最大变体**：

```text
                                          discriminant
                                          (告诉运行时是哪个变体)
                                                │
                                                ▼
   Message 实例的内存:    [tag][      payload (按最大变体对齐)      ]

   Quit                   [ 0 ][          (空)                    ]
   Echo(String)           [ 1 ][ ptr | len | cap                  ]   ← String 是 24B
   Move { x, y }          [ 2 ][ x: i32 | y: i32                  ]   ← 8B
   ChangeColor(u,u,u)     [ 3 ][ r | g | b                        ]   ← 3B

   所有变体共享同一片 payload 区域，大小由"最大变体" + 对齐填充决定。
```

实战经验：

- 如果某个变体远大于其它（比如 `Big([u8; 1024])`），考虑改用 `Box<...>` 把大数据放在堆上：
  `enum X { Tiny, Big(Box<[u8; 1024]>) }` —— 整个 enum 立刻变小到一个指针 + tag
- 详细的内存布局分析见 §16

完整运行实例见 `examples/02_enum_with_data.rs`。

---

## 四、为枚举实现方法

### 4.1 `impl` 块

和结构体一样，枚举可以通过 `impl` 挂方法。方法接收者三种全能用：

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle { w: f64, h: f64 },
}

impl Shape {
    /// &self：只读，最常见
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Square(s) => s * s,
            Shape::Rectangle { w, h } => w * h,
        }
    }

    /// 判定方法
    fn is_round(&self) -> bool {
        matches!(self, Shape::Circle(_))
    }

    /// self：消费自身、返回新值
    fn scale(self, k: f64) -> Self {
        match self {
            Shape::Circle(r) => Shape::Circle(r * k),
            Shape::Square(s) => Shape::Square(s * k),
            Shape::Rectangle { w, h } => Shape::Rectangle { w: w * k, h: h * k },
        }
    }
}
```

接收者选择跟结构体一致：

| 接收者 | 语义 | 典型用途 |
|--------|------|----------|
| `&self` | 只读借用 | 计算派生值、判定、比较 |
| `&mut self` | 可变借用 | 原地修改、切换变体 |
| `self` | 消费自身 | 类型转换、合并、终结 |

### 4.2 关联函数（构造器、parse 风格）

第一个参数不是 self 的就是关联函数，用 `Type::func()` 调用。
最常见的就是各种构造器：

```rust
impl Shape {
    fn unit_circle() -> Self { Shape::Circle(1.0) }

    fn from_size((w, h): (f64, f64)) -> Self {
        if (w - h).abs() < f64::EPSILON {
            Shape::Square(w)
        } else {
            Shape::Rectangle { w, h }
        }
    }
}

let s1 = Shape::unit_circle();
let s2 = Shape::from_size((4.0, 5.0));
```

`parse` 风格的"从字符串构造"也极常见：

```rust
impl LogLevel {
    fn parse(s: &str) -> Option<Self> {
        Some(match s.to_ascii_lowercase().as_str() {
            "trace" => Self::Trace,
            "debug" => Self::Debug,
            "info" => Self::Info,
            "warn" | "warning" => Self::Warn,
            "error" => Self::Error,
            _ => return None,
        })
    }
}
```

### 4.3 多个 `impl` 块按主题分组

和 struct 一样，可以为同一个 enum 写多个 `impl` 块：

```rust
impl Shape {
    fn perimeter(&self) -> f64 { /* ... */ }      // 几何计算分一组
}

impl fmt::Display for Shape {                     // trait 实现单独成块
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Circle(r) => write!(f, "Circle(r={r})"),
            Shape::Square(s) => write!(f, "Square({s}x{s})"),
            Shape::Rectangle { w, h } => write!(f, "Rect({w}x{h})"),
        }
    }
}
```

### 4.4 派生宏总览

和 struct 一样，常用派生覆盖几乎所有日常需求：

| 派生 | 必要条件 | 典型用途 |
|------|---------|----------|
| `Debug` | — | `{:?}` 打印，日志、调试必备 |
| `Clone` / `Copy` | 所有内部字段也 Clone/Copy | 复制、按值传递 |
| `PartialEq` / `Eq` | 内部字段也 PartialEq/Eq | `==` 比较 |
| `Hash` | 所有字段 Hash + Eq | `HashMap` / `HashSet` 的 key |
| `PartialOrd` / `Ord` | 内部也支持比较 | 排序 |
| `Default` | 所有字段 Default + 标 `#[default]` 变体 | `Type::default()` |

⚠️ **特别注意**：

- 含 `f64` / `f32` 的 enum **不能** derive `Eq`（NaN ≠ NaN，违反"完全相等"语义）
- 含 `String` / `Vec<T>` 等堆上类型的 enum **不能** derive `Copy`，只能 `Clone`

最常用的组合：

```rust
#[derive(Debug, Clone, PartialEq)]                          // 普通业务 enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]                // 纯值 enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]                // 当 HashMap key
```

### 4.5 给 enum 一个默认变体（`#[default]`）

Rust 1.62+ 起 `#[derive(Default)]` 可以直接用在 enum 上，但你得标注哪个变体是默认的：

```rust
#[derive(Debug, Clone, Copy, Default)]
enum LogLevel {
    Trace,
    Debug,
    #[default]                    // ← 默认变体
    Info,
    Warn,
    Error,
}

let lv = LogLevel::default();    // → LogLevel::Info
```

完整运行实例见 `examples/03_enum_methods.rs`。

---

## 五、match 表达式

### 5.1 match 是表达式

在 Rust 里，`match` 不是"控制流语句"，而是一个**表达式**。
这意味着每个臂的最后一个表达式就是它的值，整个 match 可以直接赋给变量、作为返回值、嵌入其它表达式：

```rust
let cents = match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
};

fn http_text(code: u16) -> &'static str {
    match code {                       // 整个 match 直接作为返回值
        200 => "OK",
        404 => "Not Found",
        500..=599 => "Server Error",
        _ => "Unknown",
    }
}
```

这一点和 Java/Go 的 `switch` 不同——它们都不能"返回值"。

### 5.2 穷尽性检查（exhaustiveness）

这是 Rust match 最让人感动的特性之一：

> 编译器会检查 match 是否覆盖了所有变体；只要漏一个，就编译失败。

```rust
enum Coin { Penny, Nickel, Dime, Quarter }

fn cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 漏写 Coin::Quarter → 编译错误：
        // error[E0004]: non-exhaustive patterns: `Coin::Quarter` not covered
    }
}
```

这意味着：未来你给 `Coin` 新增一个变体（比如 `HalfDollar`），所有写过 `match` 的地方都会**编译报错**——你不会"安静地遗漏"任何分支。

```text
  ┌─ 新增 Coin::HalfDollar ─┐
  │                          │
  ▼                          ▼
  enum Coin {
      Penny,
      Nickel,
      Dime,
      Quarter,
      HalfDollar,    ← 新增
  }

         ┌────────────────────────────────────────────────┐
         │ 立刻在所有 match 处编译报错:                      │
         │   error[E0004]: non-exhaustive patterns         │
         │     `Coin::HalfDollar` not covered              │
         │                                                 │
         │ 你被迫去每一个 match 加一臂,                       │
         │ 不会有任何 "悄悄遗漏"                             │
         └────────────────────────────────────────────────┘
```

### 5.3 `_` 通配符 vs 命名兜底

当你确实不关心剩下的变体时，可以兜底：

```rust
// _ 通配：不绑定值
fn coin_class(c: Coin) -> &'static str {
    match c {
        Coin::Penny | Coin::Nickel => "小面额",
        Coin::Dime => "中等面额",
        _ => "大面额",                  // 一次性吃掉所有剩余
    }
}

// 命名兜底：捕获那个值，本臂内可用
fn describe_number(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        other => format!("number {other}"),  // ← `other` 是个绑定变量
    }
}
```

⚠️ **慎用 `_`**：它会让编译器在新增变体时**沉默**，失去穷尽性检查的保护。
业务代码应优先列出所有变体，只在变体数量极多且语义合理时才用兜底。

### 5.4 多语句臂

每一臂可以是单个表达式，也可以是一个代码块 `{ ... }`，最后一个表达式作为值：

```rust
match c {
    Coin::Penny => {
        println!("[log] 找到 1 分钱");
        1                           // 块的最后一个表达式 = 整臂的值
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}
```

### 5.5 match vs if/else if 链

写一个等价的 if/else 版本，对比能看出 match 的优势：

```rust
// match 版（紧凑、安全）
fn cents(c: &Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// if/else 版（啰嗦、不安全）
fn cents_via_if(c: &Coin) -> u32 {
    if matches!(c, Coin::Penny) { 1 }
    else if matches!(c, Coin::Nickel) { 5 }
    else if matches!(c, Coin::Dime) { 10 }
    else { 25 }    // ⚠️ 漏处理变体编译器不会提醒
}
```

| 维度 | match | if/else if |
|------|-------|-----------|
| 紧凑度 | 高 | 低 |
| 穷尽性检查 | ✅ 编译器拦截 | ❌ 没有 |
| 模式语法 | 强大（解构、范围、守卫） | 只能 `==` |
| 表达式能力 | 强（直接赋值） | 弱（需要 if let / 临时变量） |

完整运行实例见 `examples/04_match_basics.rs`。

---

## 六、模式语法全集

模式不是 match 专属——它在 `let`、函数参数、`for`、`if let`、`while let`、`let else` 里都能出现。
本节系统过一遍模式语法。

### 6.1 字面量模式

最朴素：用具体的字面量当作模式。

```rust
fn classify_int(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        -1 => "minus one",
        _ => "other",
    }
}

fn translate_yes_no(s: &str) -> &'static str {
    match s {
        "yes" | "y" | "true" | "1" => "肯定",
        "no" | "n" | "false" | "0" => "否定",
        _ => "未知",
    }
}
```

字面量模式可用的类型：整数、`char`、`bool`、`&str`。
**`f32` / `f64` 不能用作模式**（NaN 等导致排序模糊）。

### 6.2 多模式 `|`

把多个模式用 `|` 串起来，命中任何一个都进入此臂：

```rust
match c {
    'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
    '0'..='9' => "digit",
    _ => "other",
}
```

### 6.3 范围 `..=` 与 `..`

```rust
fn grade(score: u32) -> &'static str {
    match score {
        0..=59 => "F",
        60..=69 => "D",
        70..=79 => "C",
        80..=89 => "B",
        90..=100 => "A",
        _ => "无效成绩",
    }
}
```

| 形式 | 语义 | 注意 |
|------|------|------|
| `m..=n` | 闭区间 `[m, n]` | 模式里**必须**用闭区间或半开区间，不能用 `..` 单独表达 |
| `m..n` | 半开区间 `[m, n)` | Rust 1.66+ 在模式里可用 |
| `..` | 在元组/结构体里"忽略其余字段" | 这是另一回事，见 §6.4 |

范围模式只能用在**整数**和 **`char`** 上。

### 6.4 通配 `_` 与忽略 `..`

这两个看着像，工作层级不同：

```rust
// `_` 占据一个位置，不绑定
match (1, 2, 3) {
    (_, 2, _) => "中间是 2",
    _ => "其它",
}

// `..` 在元组/结构体里"省略其余字段"（一次性吃掉多个）
fn first_two((a, b, ..): (i32, i32, i32, i32, i32)) -> (i32, i32) {
    (a, b)
}

struct Point3 { x: i32, y: i32, z: i32 }
fn xy_only(p: Point3) -> (i32, i32) {
    let Point3 { x, y, .. } = p;       // 跳过 z
    (x, y)
}
```

### 6.5 命名变量绑定（小心 shadowing）

在模式里写一个标识符 `x` 时，它会被解释成"绑定一个新变量"，而**不是**匹配外部变量 `x` 的值：

```rust
let x = 5;
let y = 10;

let result = match x {
    1 => "one",
    // ❌ 你也许想说"如果 x 等于 y"——但这里 `y` 是个新绑定！
    //    它会匹配任何值，并把那个值绑定为本臂内的 y，覆盖外面的 y=10
    y => "match anything (and bind to y)",   // ← 永远命中
    _ => "other",                             // 永远到不了
};
```

```text
  你以为的语义                           实际语义
  ─────────────────                     ──────────────
  match x {                             match x {
      1     => "one",                       1     => "one",
      y     => "外面的 y == x",      ❌      y     => "新绑定 y, 永远命中",
      _     => "other",                     _     => "永远不可达 ⚠️",
  }                                     }
                                              ↓
  想表达"x 等于变量 y"用守卫:           编译器会发出
  ─────────────────                     unreachable_patterns 警告
  match x {
      1                  => "one",
      v if v == y        => "x == y",   ← ✅ 正确
      _                  => "other",
  }
```

**这是初学者最容易踩的陷阱**。想"匹配某个变量值"应使用守卫 `if`，见 §6.6。

### 6.6 守卫 `if` 子句

模式后面可以加 `if 条件`，只有模式 + 条件都成立时才进入此臂：

```rust
fn match_with_guard(pair: (i32, i32)) -> &'static str {
    match pair {
        (0, _) | (_, 0) => "至少一个零",
        (a, b) if a == b => "相等",
        (a, b) if a.abs() == b.abs() => "绝对值相等",
        _ => "其它",
    }
}
```

守卫常用来：

- 跨字段比较：`(a, b) if a == b`
- 引入运行期条件：`Some(n) if n > 0`
- 字符串前缀判断：`x if x.starts_with("set ")`

### 6.7 `@` 绑定

当你既想"匹配某个范围"，又想"把那个具体值取出来用"时，用 `名字 @ 模式`：

```rust
fn classify_age(age: u32) -> String {
    match age {
        n @ 0..=12   => format!("孩子，年龄 {n}"),
        n @ 13..=19  => format!("青少年，年龄 {n}"),
        n @ 20..=64  => format!("成年人，年龄 {n}"),
        n            => format!("长者，年龄 {n}"),
    }
}
```

`@` 也能配合嵌套结构体：

```rust
struct Person { id: u32, name: String }

match p {
    Person { id: id @ 1..=99, name }     => /* 内部用户 */ ,
    Person { id: id @ 100..=999, name }  => /* 付费用户 */ ,
    Person { id, name }                   => /* 游客 */ ,
}
```

### 6.8 `ref` / `ref mut`（与现代 `&` 语法对照）

现代 Rust 几乎不再写 `ref`，但你会在老代码里见到，要认识：

```text
  老写法                                   现代等价写法
  ─────────────────────                   ──────────────────────
  match value {                           match &value {
      x => ...                                x => ...    // x: &T
  }                                       }

  match value {                           match &mut value {
      ref x     => ...        x: &T          x => ...    // x: &mut T
      ref mut x => ...        x: &mut T  }
  }

  现代代码: 优先 match &value / match &mut value，让编译器自动绑定借用形态
```

借用式 match 的好处：函数返回后原值仍然属于调用方，无 move：

```rust
fn describe(item: &Item) -> String {
    match item {
        Item::Single(s) => format!("一个: {s}"),    // s 是 &String
        Item::Many(v)   => format!("一组: {v:?}"),  // v 是 &Vec<String>
    }
    // 调用方还能继续使用 item
}
```

### 6.9 解构 enum / struct / tuple / 嵌套

模式可以一直嵌套，跨多层结构一次拆开：

```rust
struct Profile { name: String, address: Address }
struct Address { country: String, city: String }

enum Account {
    Anon,
    Member { user_id: u64, profile: Profile },
    Admin(Profile),
}

fn city_of(a: &Account) -> Option<&str> {
    match a {
        Account::Anon => None,
        Account::Member {
            profile: Profile {
                address: Address { city, .. },
                ..
            },
            ..
        } => Some(city.as_str()),
        Account::Admin(Profile {
            address: Address { city, .. },
            ..
        }) => Some(city.as_str()),
    }
}
```

更多解构示例见 `examples/06_destructuring.rs` 与 `examples/15_advanced_patterns.rs`。

---

## 七、可反驳性 vs 不可反驳性

模式分两种：

| 类型 | 含义 | 出现在 |
|------|------|-------|
| **不可反驳（irrefutable）** | 永远能匹配上 | `let` / 函数参数 / `for` |
| **可反驳（refutable）** | 可能匹配失败 | `if let` / `while let` / `match` 的某些臂 / `let else` |

```rust
// ✅ 不可反驳：永远成功
let (a, b) = (1, 2);
let pair @ (x, y) = (10, 20);

// ❌ refutable pattern in local binding
// let Some(v) = opt;
//     ^^^^^^^   None 不会被覆盖，let 不接受可反驳模式

// ✅ 可反驳模式必须用 if let / let else / match
if let Some(v) = opt { /* ... */ }
let Some(v) = opt else { return };
match opt {
    Some(v) => { /* ... */ }
    None => { /* ... */ }
}
```

```text
  位置             接受模式类型            示例
  ─────────────    ─────────────────       ──────────────────────────
  let              不可反驳                 let (a, b) = pair
  fn 参数          不可反驳                 fn f((x, y): (i32, i32))
  for              不可反驳                 for (k, v) in &map
  match 整个       任意（按穷尽性约束）       match x { Some(v) => .., None => .. }
  if let           可反驳                   if let Some(v) = opt
  while let        可反驳                   while let Some(x) = stack.pop()
  let else         可反驳（else 必须发散）    let Some(v) = opt else { return };
```

完整对照见 `examples/15_advanced_patterns.rs`。

---

## 八、模式可用的位置

总览一下"模式"能在哪些语法里出现：

```rust
// 1) let
let (a, b) = (1, 2);

// 2) 函数参数
fn print_pair((x, y): (i32, i32)) { /* ... */ }

// 3) for 循环
for (k, v) in &map { /* ... */ }

// 4) match
match x {
    1 => /* ... */,
    2..=10 => /* ... */,
    _ => /* ... */,
}

// 5) if let
if let Some(v) = opt { /* ... */ }

// 6) while let
while let Some(top) = stack.pop() { /* ... */ }

// 7) let else (Rust 1.65+)
let Some(v) = opt else { return };

// 8) 闭包参数
let add_pair = |(a, b): (i32, i32)| a + b;
```

**直觉**：哪里需要"从一个值里取东西出来"，哪里就能用模式。

---

## 九、`if let` / `let else` / `while let`

### 9.1 `if let`：只关心一个变体

`match` 必须穷尽，但有时你只想"如果匹配上某种情况就做点事，否则什么都不做"。
完整 `match` 太啰嗦：

```rust
match opt {
    Some(x) => println!("{x}"),
    None => {}                          // 啥都不干，纯凑数
}
```

这种场景就用 `if let`：

```rust
if let Some(x) = opt {
    println!("{x}");
}
```

### 9.2 `if let ... else`：再加一条 fallback

想在"没匹配上"时也走一段逻辑？加 `else`：

```rust
fn describe_opt(opt: Option<&str>) -> String {
    if let Some(name) = opt {
        format!("欢迎 {name}!")
    } else {
        "请先登录".to_string()
    }
}
```

### 9.3 `if let` 链（Rust 1.88+）

把多个 `if let` 用 `&&` 串起来——多个模式匹配 + 任意布尔条件混着写。
早期版本里这种写法要靠嵌套 `if let` 或 `match`，现在直接一行搞定：

```rust
fn welcome_vip(user: Option<&User>) {
    // 三个条件同时成立才欢迎：
    if let Some(u) = user
        && let Role::Member { vip } = &u.role
        && *vip
    {
        println!("尊贵的 VIP {}！", u.name);
    } else {
        println!("普通通道，欢迎光临");
    }
}
```

### 9.4 `let else`：早返回 + 把值"摊"出来

`let else` 用来在"必须匹配上某个模式，否则就立刻退出"的场景里去掉嵌套：

```rust
// 老写法（嵌套缩进）
fn validate_old(input: &str) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(age) if age >= 18 => Ok(age),
        Ok(_) => Err("未满 18 岁".into()),
        Err(_) => Err(format!("'{input}' 不是合法的 u8")),
    }
}

// 新写法（平直清爽）
fn validate(input: &str) -> Result<u8, String> {
    let Ok(age) = input.parse::<u8>() else {
        return Err(format!("'{input}' 不是合法的 u8"));
    };
    if age < 18 {
        return Err("未满 18 岁".into());
    }
    Ok(age)
}
```

⚠️ `let else` 的 **`else` 块必须发散**（`return` / `break` / `continue` / `panic!` / `unreachable!`），否则编译失败——这条规则保证了"被绑定的值在后面一定合法"。

### 9.5 `while let`：循环消费

`while let 模式 = 表达式 { ... }` 只要表达式匹配上模式就继续：

```rust
// 经典：从栈反复 pop 直到为空
let mut stack = vec![1, 2, 3, 4, 5];
while let Some(top) = stack.pop() {
    println!("{top}");
}

// 配合迭代器
let mut iter = text.lines();
while let Some(line) = iter.next() {
    /* ... */
}

// 事件队列：遇到 Quit 就退出
while let Some(ev) = queue.pop() {
    match ev {
        Event::Quit => return,
        other => process(other),
    }
}
```

### 9.6 `if let` 与 `match` 的取舍

| 场景 | 推荐 |
|------|------|
| 只关心 1 个变体，其它一概忽略 | `if let` |
| 只关心 1 个变体，但失败时也要做事 | `if let` / `else` |
| 多个变体都要分别处理 | `match` |
| 复杂的守卫 / 嵌套 | `match` |
| 想顺便确保"不漏变体" | `match`（利用穷尽性检查） |

⚠️ 当你"为了避免一行 match 而写 `if let`，但其实关心多个分支"时，**反而会丢掉穷尽性检查**——别贪图短而牺牲安全。

完整对照见 `examples/07_if_let_while_let.rs`。

---

## 十、`Option<T>`：替代 null 的标准答案

### 10.1 为什么 Rust 没有 null

在 C / Java / JS / Python 里，"null"（或 `None` / `nil` / `undefined`）是导致最多线上 bug 的特性之一。Tony Hoare 自己都说："null 是我十亿美元级的错误"。

问题在于：

- 任何引用类型都可能为 null
- 编译器不区分"可能为 null"和"绝对不为 null"
- 解引用 null 是运行时崩溃

Rust 的解决方案：完全不要 null，用 enum 在类型层面表达"可能没有"：

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

这样：

- 一个值的类型是 `T` → 它**一定**有效
- 一个值的类型是 `Option<T>` → 你**必须**先处理 `None`

编译器全程帮你审查"我有没有处理 None"，避免空指针异常。

```text
   Java / Go / JS                          Rust
   ─────────────────────────               ──────────────────────────
   String name = ...;                      let name: String = ...;
   name.length()                           name.len()
   ❗ 可能 NullPointerException             ✅ 编译器保证 name 一定有值

   String name = nullable();               let name: Option<String> = nullable();
   name.length()                           name.len()  ❌ 编译错误
   ❗ 编译器不会拦截                         ✅ 编译器逼你处理 None

                                           match name {
                                               Some(n) => n.len(),
                                               None => 0,
                                           }
```

### 10.2 创建 Option

```rust
let a: Option<i32> = Some(42);
let b: Option<i32> = None;

let c: Option<i32> = "42".parse().ok();      // 把 Result 转成 Option
let d: Option<i32> = "abc".parse().ok();     // None
```

### 10.3 取值：unwrap 系列

把 `Option<T>` 转回 `T` 的方法，从最危险到最安全：

| 方法 | 行为 | 用途 |
|------|------|------|
| `.unwrap()` | None 时 panic | 测试 / demo / 已知必为 Some |
| `.expect("msg")` | None 时 panic 并打印自定义信息 | 调试时优于 unwrap |
| `.unwrap_or(默认)` | None 时返回默认值（**已经算好**） | 简单默认 |
| `.unwrap_or_else(\|\| ..)` | None 时**懒求值**默认值 | 默认值代价大时 |
| `.unwrap_or_default()` | None 时用 `T::default()` | T 实现了 Default |

```rust
let some_v: Option<i32> = Some(7);
let none_v: Option<i32> = None;

println!("{}", some_v.unwrap_or(0));         // 7
println!("{}", none_v.unwrap_or(0));         // 0
println!("{}", none_v.unwrap_or_else(|| {
    println!("[懒求值闭包被执行]");
    99
}));                                          // 99，并打印日志
```

### 10.4 组合子：map / and\_then / filter / or

这一组是 `Option` 的"组合子"，让你像写 Excel 公式一样把变换串起来，不用写一堆 `match` 分支。

```rust
// map: Some(x) → Some(f(x)); None → None
Some(10).map(|n| n * 2)                  // Some(20)
None::<i32>.map(|n| n * 2)               // None

// and_then (flat_map)：闭包返回 Option，自动扁平化
fn parse(s: &str) -> Option<i32> { s.parse().ok() }
fn positive(n: i32) -> Option<i32> { if n > 0 { Some(n) } else { None } }

Some("5").and_then(parse).and_then(positive)   // Some(5)
Some("0").and_then(parse).and_then(positive)   // None
Some("abc").and_then(parse).and_then(positive) // None

// filter: 满足谓词才保留
Some(42).filter(|&n| n > 100)            // None

// or / or_else: Some 优先，否则尝试备选
None.or(Some(5))                         // Some(5)
Some(5).or(None)                         // Some(5)
```

```text
   方法链可视化:

   Some("5") ──parse──▶ Some(5) ──positive──▶ Some(5)
   Some("0") ──parse──▶ Some(0) ──positive──▶ None      ← 中途变 None
   Some("x") ──parse──▶ None    ──positive──▶ None      ← None 短路传播
   None      ──parse──▶ None    ──positive──▶ None      ← None 一路 None
```

### 10.5 引用转换：as\_ref / as\_mut / cloned / copied

| 方法 | 签名 | 用途 |
|------|------|------|
| `.as_ref()` | `Option<T>` → `Option<&T>` | 不消耗原值，借用内部 |
| `.as_mut()` | `Option<T>` → `Option<&mut T>` | 借用并就地修改 |
| `.cloned()` | `Option<&T>` → `Option<T>` (T: Clone) | 把借用版本变回拥有版本 |
| `.copied()` | `Option<&T>` → `Option<T>` (T: Copy) | 同上但用 `Copy` |

```rust
let owned: Option<String> = Some("hello".to_string());
let r: Option<&String> = owned.as_ref();      // owned 还能继续用
let len: Option<usize> = r.map(|s| s.len());
```

### 10.6 就地变换：take / replace / get\_or\_insert

| 方法 | 行为 |
|------|------|
| `.take()` | 把内部值取出来，自身变 `None` |
| `.replace(v)` | 用 `v` 替换内部，返回旧值 |
| `.get_or_insert(v)` | 是 None 时塞入 `v`，返回 `&mut T` |
| `.get_or_insert_with(\|\| ..)` | 上面的懒求值版本 |

```rust
let mut a = Some(10);
let v = a.take();                     // a 变成 None，v = Some(10)

let mut b = Some(1);
let old = b.replace(2);               // b = Some(2)，old = Some(1)

let mut c: Option<Vec<i32>> = None;
c.get_or_insert_with(Vec::new).push(7);  // c = Some(vec![7])
```

### 10.7 `?` 运算符（Option 版本）

`?` 让你写"短路链式":

- 如果当前是 `Some(x)`，自动解出 x 继续往下
- 如果当前是 `None`，函数立刻返回 `None`

要求：**函数返回值类型也是 `Option<...>`**。

```rust
struct User { profile: Option<Profile> }
struct Profile { contact: Option<Contact> }
struct Contact { email: Option<String> }

// 用 ? 一气呵成
fn user_email(u: &User) -> Option<&String> {
    let profile = u.profile.as_ref()?;       // None ? → return None
    let contact = profile.contact.as_ref()?;
    contact.email.as_ref()
}

// 不用 ? 的等价（嵌套地狱）
fn user_email_old(u: &User) -> Option<&String> {
    match u.profile.as_ref() {
        Some(p) => match p.contact.as_ref() {
            Some(c) => c.email.as_ref(),
            None => None,
        },
        None => None,
    }
}
```

完整运行实例见 `examples/08_option.rs`。

---

## 十一、`Result<T, E>` 简介

### 11.1 与 Option 的关系

标准库的定义只有几行：

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `Option<T>` —— 表达"可能没值"
- `Result<T, E>` —— 表达"可能失败"，并附带 `E` 描述失败原因

凡是函数"可能失败"的场景都返回 `Result`：`parse` / `read_file` / `connect` / 几乎所有 I/O 函数。

```rust
fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("除数不能为 0".to_string()) }
    else { Ok(a / b) }
}

match safe_div(10, 0) {
    Ok(v) => println!("结果 {v}"),
    Err(e) => println!("失败: {e}"),
}
```

### 11.2 常用 API

和 `Option` 同名 / 类似，但要多处理 `Err` 这一边：

| 方法 | 行为 |
|------|------|
| `.is_ok()` / `.is_err()` | 判定 |
| `.unwrap()` / `.expect("msg")` | Err 时 panic |
| `.unwrap_or(默认)` / `.unwrap_or_else(\|e\| ..)` | Err 时用默认 |
| `.map(\|x\| ..)` | 变换 Ok 这一侧 |
| `.map_err(\|e\| ..)` | 变换 Err 这一侧 |
| `.and_then(\|x\| -> Result<...>)` | Ok 时继续，Err 短路 |
| `.or_else(\|e\| -> Result<...>)` | Err 时尝试备选 |
| `.ok()` / `.err()` | 拆成两半的 Option |

### 11.3 `?` 运算符 + `From` 自动转换

```rust
fn parse_and_sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;        // Err 自动 return
    let y: i32 = b.parse()?;
    Ok(x + y)
}
```

`?` 等价于：

```rust
let x = match expr {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),    // ← 注意：自动 From 转换
};
```

不同步骤的错误类型不一样时，统一用一个 `Error` 类型 + `impl From`：

```rust
#[derive(Debug)]
enum MyError {
    ParseError(ParseIntError),
    Logic(String),
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::ParseError(e)
    }
}

fn pipeline(a: &str, b: &str) -> Result<i32, MyError> {
    let x: i32 = a.parse()?;        // ParseIntError → MyError，靠 From
    let y: i32 = b.parse()?;
    if y == 0 { return Err(MyError::Logic("除数不能为 0".into())); }
    Ok(x / y)
}
```

更完整的错误处理（`thiserror` / `anyhow` / 自定义 Error）会放到第 10 章。

### 11.4 Option ↔ Result 互转

```rust
// Option → Result
let r1 = some_v.ok_or("缺值");                     // Some → Ok, None → Err("缺值")
let r2 = some_v.ok_or_else(|| format!("缺 {}", "key"));  // 懒求值

// Result → Option
let o1 = res.ok();                                  // Ok(v) → Some(v), Err → None
let o2 = res.err();                                 // 反向
```

完整运行实例见 `examples/09_result_intro.rs`。

---

## 十二、enum 与状态机

状态机是 enum 在工程里出镜率最高的场景之一：

> "状态 = enum 变体" + "方法表达状态转移"

### 12.1 自动状态机：红绿灯

```rust
#[derive(Debug, Clone, Copy)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn next(self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}
```

```text
       +------+      30s       +-------+      3s        +--------+      30s
       | Red  |  ─────────▶    | Green |  ─────────▶    | Yellow |  ─────────▶  Red
       +------+                +-------+                +--------+
```

### 12.2 事件驱动：订单状态

现实业务里，状态转移往往是"事件触发"的，而不是自动 tick：

```rust
enum OrderState { Created, Paid, Shipped, Delivered, Cancelled, Refunded }

enum OrderEvent { Pay, Ship, Deliver, Cancel, Refund }

#[derive(Debug)]
enum TransitionError {
    InvalidTransition { from: OrderState, by: OrderEvent },
}

impl Order {
    fn apply(&mut self, event: OrderEvent) -> Result<(), TransitionError> {
        let new_state = match (&self.state, &event) {
            (OrderState::Created, OrderEvent::Pay) => OrderState::Paid,
            (OrderState::Created, OrderEvent::Cancel) => OrderState::Cancelled,
            (OrderState::Paid, OrderEvent::Ship) => OrderState::Shipped,
            (OrderState::Paid, OrderEvent::Cancel) => OrderState::Cancelled,
            (OrderState::Shipped, OrderEvent::Deliver) => OrderState::Delivered,
            (OrderState::Shipped, OrderEvent::Refund) => OrderState::Refunded,
            // 其它所有 (state, event) 组合都不合法
            (from, _) => return Err(TransitionError::InvalidTransition {
                from: *from, by: event,
            }),
        };
        self.state = new_state;
        Ok(())
    }
}
```

`match (state, event)` 是事件驱动状态机的标准写法。

### 12.3 带数据的状态机：网络连接

不同状态可以"携带"不同的数据：

```rust
enum Connection {
    Disconnected,
    Connecting { host: String, port: u16 },
    Connected { host: String, port: u16, retries: u32 },
    Failed { host: String, reason: String },
}

impl Connection {
    fn complete(self) -> Self {
        match self {
            Connection::Connecting { host, port } =>
                Connection::Connected { host, port, retries: 0 },
            other => other,         // 其它状态不允许 complete
        }
    }
}
```

### 12.4 Type-State 风格：编译期拒绝非法转换

更激进的版本是：把每种状态做成**独立的类型**，让编译器在编译期就拒绝错误调用：

```rust
struct PaidOrder { id: u64, amount_cents: u32 }
struct ShippedOrder { id: u64, amount_cents: u32, carrier: String }
struct DeliveredOrder { /* ... */ }

impl PaidOrder {
    fn ship(self, carrier: impl Into<String>) -> ShippedOrder { /* ... */ }
}

impl ShippedOrder {
    fn deliver(self) -> DeliveredOrder { /* ... */ }
    // ⚠️ 没有 ship 方法 —— shipped.ship("DHL") 直接编译报错
}
```

| 方案 | 错误检查时机 | 适用 |
|------|-------------|------|
| 单 enum + match | 运行期返回 Err | 90% 业务场景 |
| Type-State（独立类型） | **编译期** | 关键流程、零容错 |

完整运行实例见 `examples/10_state_machine.rs`。

---

## 十三、递归枚举

### 13.1 为什么需要 `Box`

直觉上你可能会写：

```rust
enum List {
    Cons(i32, List),       // ❌ 编译报错
    Nil,
}
```

编译器拒绝它的理由是：`List` 直接包含 `List`，导致大小无限大：

```text
   List 大小 = max(Cons 的大小, Nil 的大小)
   Cons 大小 = i32 + List 大小
                       ▲
                       │ 又依赖 List 大小……
                       │
                  无限递归 → 编译器无法决定 size
```

只要在递归位置加一层"间接"（指针），就能打破循环：

```rust
enum List {
    Cons(i32, Box<List>),  // ✅ Box 是固定大小（指针），递归被打破
    Nil,
}
```

`Box<T>` 在堆上分配 `T`，自身在栈上只占一个指针（64 位平台 8 字节）。

### 13.2 链表

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn len(&self) -> usize {
        match self {
            List::Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    fn from_slice(items: &[i32]) -> Self {
        let mut node = List::Nil;
        for &x in items.iter().rev() {
            node = List::Cons(x, Box::new(node));
        }
        node
    }
}
```

### 13.3 JSON Value

`serde_json::Value` 的简化版本——一个 JSON 值要么是基本类型，要么是嵌套结构：

```rust
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
```

> 注意：Vec 和 HashMap 自带"间接层"，所以不需要再套 Box。

### 13.4 表达式树（AST）+ 递归求值

把"语法结构"用一个递归 enum 描述出来，是 enum 的另一个杀手级用法：

```rust
enum Expr {
    Num(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

enum EvalError {
    UnboundVariable(String),
    DivByZero,
}

impl Expr {
    fn eval(&self, env: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        match self {
            Expr::Num(n) => Ok(*n),
            Expr::Var(name) => env.get(name.as_str()).copied()
                .ok_or(EvalError::UnboundVariable(name.clone())),
            Expr::Add(a, b) => Ok(a.eval(env)? + b.eval(env)?),
            Expr::Sub(a, b) => Ok(a.eval(env)? - b.eval(env)?),
            Expr::Mul(a, b) => Ok(a.eval(env)? * b.eval(env)?),
            Expr::Div(a, b) => {
                let bv = b.eval(env)?;
                if bv == 0.0 { return Err(EvalError::DivByZero); }
                Ok(a.eval(env)? / bv)
            }
            Expr::Neg(a) => Ok(-a.eval(env)?),
        }
    }
}
```

完整运行实例（含构造帮手与多种错误路径）见 `examples/11_recursive_enum.rs`。

---

## 十四、命令模式 / Action / Route

### 14.1 state 用 struct，command 用 enum

这是 enum 在工程里出镜率最高的场景之一——CLI / Web 路由的 subcommand、Redux/Flux 的 Action、事件溯源的命令日志、工作流引擎、UI 状态更新。

```rust
// state（多个字段同时存在）→ struct
struct AppState {
    todos: HashMap<u64, Todo>,
    next_id: u64,
}

// command（一组互斥的"要做什么"）→ enum
enum Command {
    Add { title: String },
    Complete { id: u64 },
    Toggle { id: u64 },
    Rename { id: u64, title: String },
    Remove { id: u64 },
    ClearCompleted,
    Batch(Vec<Command>),     // 递归：一个命令可以由多个子命令组成
}
```

### 14.2 reducer 与穷尽 match

reducer 把"当前状态 + 命令"变成"新状态"。这就是 Redux / Elm 架构的核心：

```rust
impl AppState {
    fn dispatch(&mut self, cmd: Command) -> Result<(), CommandError> {
        match cmd {
            Command::Add { title } => self.add(title),
            Command::Complete { id } => self.set_done(id, true),
            Command::Toggle { id } => self.toggle(id),
            Command::Rename { id, title } => self.rename(id, title),
            Command::Remove { id } => self.remove(id),
            Command::ClearCompleted => self.clear_completed(),
            Command::Batch(cmds) => {
                for c in cmds { self.dispatch(c)?; }
                Ok(())
            }
        }
    }
}
```

reducer 是个**穷尽 match**，新增命令时编译器逼你来这里加分支。这是命令模式的核心安全保障。

### 14.3 路由（Subcommand）

`clap 4.x`、`actix-web`、`axum` 等真实框架里，一个 `#[derive(Subcommand)]` 的 enum 就是 CLI 子命令的本体。简化版：

```rust
enum Route {
    Index,
    UserDetail { id: u64 },
    UserEdit { id: u64 },
    Search { keyword: String, page: u32 },
    NotFound(String),
}

fn handle_route(route: Route) -> String {
    match route {
        Route::Index => "GET /".into(),
        Route::UserDetail { id } => format!("GET /users/{id}"),
        Route::UserEdit { id } => format!("PATCH /users/{id}"),
        Route::Search { keyword, page } => format!("GET /search?q={keyword}&page={page}"),
        Route::NotFound(path) => format!("404 - {path}"),
    }
}
```

完整运行实例（含单元测试）见 `examples/13_command_pattern.rs`。

---

## 十五、实战：用 enum + match 实现一个计算器

`examples/12_calculator.rs` 是 README 计划里点名要做的练习：用 enum + match 实现简单计算器（加减乘除 + 错误处理）。

**设计要点**：

1. 用 enum 同时表达"运算"和"错误"：

   ```rust
   enum Op { Add, Sub, Mul, Div }

   enum CalcError {
       DivByZero,
       Overflow,
       InvalidNumber(String),
       InvalidOperator(String),
       InvalidSyntax(String),
   }
   ```

2. `calc` 返回 `Result<i64, CalcError>`，用 `checked_xxx` 系列处理整数溢出：

   ```rust
   fn calc(a: i64, op: Op, b: i64) -> Result<i64, CalcError> {
       match op {
           Op::Add => a.checked_add(b).ok_or(CalcError::Overflow),
           Op::Sub => a.checked_sub(b).ok_or(CalcError::Overflow),
           Op::Mul => a.checked_mul(b).ok_or(CalcError::Overflow),
           Op::Div => {
               if b == 0 { Err(CalcError::DivByZero) }
               else { a.checked_div(b).ok_or(CalcError::Overflow) }
           }
       }
   }
   ```

3. 字符串解析 + 错误传播 + 端到端 API：

   ```rust
   fn evaluate(input: &str) -> Result<i64, CalcError> {
       let (a, op, b) = parse_binary(input)?;
       calc(a, op, b)
   }
   ```

4. **完整的单元测试**：覆盖 happy path + 各种错误路径（除零、溢出、非法数字、非法语法）。

```bash
# 运行 demo
cargo run --example 12_calculator

# 跑测试
cargo test --example 12_calculator
```

完整代码与所有测试用例见 `examples/12_calculator.rs`。

---

## 十六、枚举内存布局

很多人听到 "Rust 是零成本抽象" 时会质疑："这只是个口号吗？" 本节用 `std::mem::size_of` 给出硬证据。

### 16.1 discriminant：每个实例都自带一个 tag

每个 enum 实例运行时都需要某种方式表达"我现在是哪个变体"。Rust 给每个变体分配一个整数判别式（discriminant），缺省从 0 开始递增：

```rust
enum Direction { Up = 0, Down = 1, Left = 2, Right = 3 }
```

编译器生成的内存布局（默认）大致是：

```text
   Direction:        [tag: u8 ]    (1 字节，因为变体数量 <= 256)

   多变体 enum + payload:
   ┌────┬───────────────────────────────────────┐
   │tag │  payload (按"最大变体"对齐的存储空间)   │
   └────┴───────────────────────────────────────┘
```

不带数据的枚举可以像 C 一样指定数值：

```rust
#[repr(u16)]                  // 强制 tag 占 u16
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
    Unknown = 0,
}

let code = HttpStatus::NotFound as u16;       // 404
```

### 16.2 enum 的总大小 = max(变体) + tag + padding

```rust
enum SkewedVariants {
    Tiny,                         // 0 字节
    Small(u8),                    // 1 字节
    HugeOne(u64, u64, u64, u64),  // 32 字节 → 整个 enum 至少 ~40 字节
}

assert!(size_of::<SkewedVariants>() >= 32);
```

实战经验：

- 如果某变体远大于其它（比如 `Big([u8; 1024])`），改用 `Box<[u8; 1024]>` 把大数据放堆上
- 整体 enum 立刻变小到一个指针 + tag

### 16.3 Niche / Null Pointer Optimization (NPO)

如果 inner 类型有"绝不会出现的位模式"（niche），编译器会把 `None` 编码成那个位模式，**不再额外占 tag 字节**。

最经典的例子：

- 引用 `&T` / `&mut T` 绝不能为 0
- `Box<T>` / `Vec<T>` / `String` 内部指针也不为 0
- `NonZeroU32` / `NonZeroI64` 等专门标了"不为 0"

```text
   Option<&i32> 的内存布局:

     +------------------+
     |   8 bytes (ptr)  |        size_of::<Option<&i32>>() == size_of::<&i32>() == 8
     +------------------+
            |
            +-- 非 0 地址   → Some(&T),  值就是那个地址
            +-- 0 (null)    → None      (利用 "&T 永非 null" 的性质)


   Option<i32> (没有 NPO, 需要额外 tag 字节):

     +-----+-----------+--------+
     | tag |  padding  |  i32   |     size_of::<Option<i32>>() == 8
     | 1B  |    3B     |   4B   |     (而 size_of::<i32>() == 4)
     +-----+-----------+--------+
```

```rust
use std::mem::size_of;

assert_eq!(size_of::<&i32>(),                size_of::<Option<&i32>>());     // 8 == 8
assert_eq!(size_of::<Box<i32>>(),            size_of::<Option<Box<i32>>>()); // 8 == 8
assert_eq!(size_of::<Vec<u8>>(),             size_of::<Option<Vec<u8>>>());  // 24 == 24
```

这让 Rust 的 `Option<&T>` 在性能上**和 C 的 nullable 指针完全一样**，同时保持安全性。

### 16.4 `#[repr]` 全景

| 属性 | 作用 | 典型场景 |
|------|------|----------|
| 默认（无 `repr`） | 编译器自由优化布局 | 99% 业务代码 |
| `#[repr(u8)]` / `#[repr(u16)]` / `#[repr(i32)]` | 指定 tag 宽度 | FFI、协议、序列化 |
| `#[repr(C)]` | 兼容 C 的 ABI | FFI、共享内存 |
| `#[repr(transparent)]` | Newtype 与内部类型 ABI 完全等价 | 包装 C 类型 |

完整运行实例见 `examples/14_memory_layout.rs`。

---

## 十七、enum 在真实世界

### 17.1 标准库里的高频 enum

| Enum | 定义 | 用途 |
|------|------|------|
| `Option<T>` | `None / Some(T)` | 替代 null |
| `Result<T, E>` | `Ok(T) / Err(E)` | 错误处理 |
| `std::cmp::Ordering` | `Less / Equal / Greater` | 比较结果 |
| `std::io::ErrorKind` | `NotFound / PermissionDenied / ...` | I/O 错误分类 |
| `std::net::IpAddr` | `V4(Ipv4Addr) / V6(Ipv6Addr)` | IP 地址 |
| `std::sync::TryLockError<T>` | 锁获取失败的几种情况 | 并发 |
| `std::ops::ControlFlow` | `Continue / Break` | 自定义控制流 |
| `Cow<'a, T>` | `Borrowed(&'a T) / Owned(T)` | 借用 / 拥有 二选一 |

### 17.2 生态库里的典型例子

```rust
// serde_json::Value：JSON 的代数数据类型
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

// clap::Subcommand：CLI 子命令直接是 enum
#[derive(Subcommand)]
enum Cli {
    Add { name: String },
    Remove { id: u64 },
    List,
}

// tokio::sync::oneshot::error::TryRecvError
pub enum TryRecvError {
    Empty,
    Closed,
}

// reqwest::StatusCode 内部用 enum 区分类别
// actix_web::Error 把所有 HTTP 错误用 enum 整理
// thiserror 派生宏帮你自动给 enum 实现 std::error::Error
```

### 17.3 enum 的八种典型角色

| 角色 | 举例 | 特征 |
|------|------|------|
| **互斥分类** | `Direction` / `LogLevel` | 单元变体 + 简单方法 |
| **可空 / 错误** | `Option` / `Result` | 标准库内置 |
| **消息 / 命令** | `Message` / `Action` | 各变体带不同数据 |
| **状态机** | `OrderState` / `Connection` | 配合 match 表达转移 |
| **AST / 树** | `Expr` / `JsonValue` | 递归 + Box |
| **路由 / Subcommand** | `Route` / clap 子命令 | 各变体携带参数 |
| **错误聚合** | `MyError` 多种错误来源 | 配合 `From` + `?` |
| **协议 / 状态码** | `HttpStatus` | `#[repr(...)]` + 显式 discriminant |

---

## 十八、常见错误与易错点

### 1. 漏处理变体（编译器会拦截）

```rust
match c {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    // 漏 Coin::Quarter
}
// error[E0004]: non-exhaustive patterns: `Coin::Quarter` not covered
```

**修复**：补齐变体，或者万不得已用 `_` 兜底（但失去新增变体的提醒）。

### 2. 模式里写变量名 = 绑定新变量（shadow 陷阱）

```rust
let x = 5;
let y = 10;
match x {
    1 => "one",
    y => "match anything",   // ⚠️ 这是新绑定！永远命中
    _ => "other",            // 永远到不了
}
```

**修复**：用守卫 `match x { v if v == y => ... }`。

### 3. 含 `f64` 的 enum 不能 derive `Eq`

```rust
#[derive(PartialEq, Eq)]      // ❌ f64 不是 Eq
enum Shape { Circle(f64) }
```

**修复**：只 derive `PartialEq`；想进 HashMap 就把 `f64` 换成 `u64`（存分）或用 `OrderedFloat`。

### 4. 含 `String` 的 enum 不能 derive `Copy`

```rust
#[derive(Copy, Clone)]        // ❌ String 不是 Copy
enum Msg { Echo(String) }
```

**修复**：只 derive `Clone`；或把 `String` 换成 `&'static str`（如果语义允许）。

### 5. 直接递归会无限大

```rust
enum List {
    Cons(i32, List),    // ❌ 大小无限大
    Nil,
}
```

**修复**：在递归位置加 `Box`：`Cons(i32, Box<List>)`。

### 6. 在 `let` 里用可反驳模式

```rust
let Some(x) = opt;     // ❌ refutable pattern in local binding
```

**修复**：用 `if let` / `let else` / `match`。

### 7. 守卫顺序写错导致 `unreachable_patterns`

```rust
match msg {
    Message::Echo(s) => format!("echo {s}"),
    Message::Echo(s) if s.is_empty() => "empty".into(),  // ⚠️ 永远到不了
}
```

**修复**：把更具体的臂（带守卫的）放前面。

### 8. 滥用 `_` 通配丢掉穷尽性检查

```rust
match status {
    Status::Active => "ok",
    _ => "other",            // ⚠️ 新增变体时编译器不会提醒
}
```

**修复**：业务代码尽量列出所有变体。

### 9. 不知道 `?` 要求函数返回 `Option` / `Result`

```rust
fn foo() -> i32 {
    let v: i32 = "abc".parse()?;     // ❌ 函数返回类型不是 Result
    v
}
```

**修复**：改函数签名为 `Result<i32, _>`，或用 `unwrap_or` / `match`。

### 10. 把 enum 当成"可继承的基类"

```rust
// ❌ 这是 OOP 思维
// enum Animal { Dog extends Animal { ... } }
```

**修复**：Rust 没有继承。用**组合**或 **trait 对象**（`dyn Trait`）表达多态。

---

## 十九、API 设计准则

### 1. 何时用 enum

| 你的需求 | 推荐 |
|---------|------|
| 一组互斥的可能性 | enum |
| 各种错误情况聚合 | enum + `From` |
| 表达"可能没有 / 可能失败" | `Option` / `Result` |
| 状态机的状态、命令、事件 | enum |
| AST / 树形数据 | enum + `Box` |
| 命令行子命令 | `#[derive(Subcommand)]` |

### 2. 何时用结构体而不是 enum

| 场景 | 推荐 |
|------|------|
| 一组字段同时存在 | struct |
| 配置 / DTO | struct |
| 数据容器（列表 / 图） | struct |

### 3. 何时用 trait 对象（`dyn Trait`）而不是 enum

| 维度 | enum | `dyn Trait` |
|------|------|-------------|
| 变体数量 | 编译期固定 | 运行期可扩展 |
| 性能 | 静态分发，零开销 | 动态分发，有 vtable 间接 |
| 跨 crate 扩展 | 不行（变体在定义处闭合） | 行（任何 crate 都能 `impl Trait`） |
| 适用 | 业务领域模型、协议、状态 | 插件、第三方扩展、运行时多态 |

经验法则：**默认用 enum，开放扩展用 trait 对象**。

### 4. 命名规范

| 对象 | 命名 | 例子 |
|------|------|------|
| enum 名 | 大驼峰，名词 | `Direction` / `OrderState` |
| 变体名 | 大驼峰，简洁 | `Up` / `Paid` / `NotFound` |
| 错误 enum | 名词 + Error | `CalcError` / `BuildError` |
| 命令 enum | 动词单词或短语 | `Add` / `Remove` / `ClearCompleted` |

### 5. 派生选型

| 类型特征 | 推荐派生 |
|----------|----------|
| 普通业务 enum | `Debug, Clone, PartialEq` |
| 纯值（小、不带 String/Vec） | `Debug, Clone, Copy, PartialEq, Eq` |
| 当 HashMap key | `Debug, Clone, PartialEq, Eq, Hash` |
| 有合理默认 | 加 `Default` + `#[default]` 标注变体 |
| 错误类型 | `Debug` + `impl Display + Error`（手写或用 `thiserror`） |

### 6. 何时用 `match` vs `if let`

- 多个变体都要分别处理 → `match`（享受穷尽性检查）
- 只关心一个变体，其它一概忽略 → `if let`
- 必须匹配上某个模式，否则早返回 → `let else`
- 循环消费 `Some(x)` → `while let`

### 7. 错误聚合的最小套路

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self { AppError::Parse(e) }
}

fn pipeline() -> Result<(), AppError> {
    let s = std::fs::read_to_string("a.txt")?;  // io::Error → AppError
    let n: i32 = s.trim().parse()?;             // ParseIntError → AppError
    if n < 0 { return Err(AppError::Custom("negative".into())); }
    Ok(())
}
```

下一章会用 `thiserror` / `anyhow` 优化这套写法。

---

## 二十、综合练习

### 练习 1：方向枚举进阶

定义 `Direction { Up, Down, Left, Right }`，实现：

- `opposite(self) -> Self`：反方向
- `turn_left(self) -> Self` / `turn_right(self) -> Self`
- `from_char(c: char) -> Option<Self>`：'U'/'D'/'L'/'R' 解析

要求：派生 `Debug, Clone, Copy, PartialEq, Eq, Hash`。

### 练习 2：HTTP 状态码

定义 `enum HttpStatus`，列出常用的 12 个状态码（200/201/204/301/302/400/401/403/404/500/502/503）。
实现：

- `category(self) -> Category`（其中 `Category` 是另一个 enum：`Success / Redirection / ClientError / ServerError`）
- `as_u16(self) -> u16`：返回数值（提示：用 `#[repr(u16)]`）
- `from_code(code: u16) -> Option<Self>`：反向解析

### 练习 3：Option 链式调用

写一个函数 `fn pipeline(input: &str) -> Option<i32>`，要求：

1. 解析为 `i32`
2. 必须 ≥ 0
3. 平方后必须 ≤ 1000
4. 返回平方值

完全用 `Option` 的组合子（`and_then` / `filter` / `map`）实现，不写 `match`。

### 练习 4：状态机 - 文章发布

实现一个 `Article` 状态机，状态：`Draft / Reviewing / Published / Archived`。
事件：`SubmitForReview / Approve / Reject / Publish / Archive`。
合法转换：

- Draft → Reviewing （SubmitForReview）
- Reviewing → Draft （Reject）
- Reviewing → Published （Approve+Publish 一步到位）
- Published → Archived（Archive）

非法转换返回 `Err(InvalidTransition { from, by })`。

### 练习 5：表达式求值器扩展

在 `examples/11_recursive_enum.rs` 的 `Expr` 基础上：

- 支持 `Pow(Box<Expr>, Box<Expr>)`（幂）
- 支持 `Sqrt(Box<Expr>)`（平方根，遇到负数返回 `Err`）
- 实现 `simplify(&self) -> Expr`：把所有常量折叠（如 `2 + 3` → `5`）

### 练习 6：JSON Value Pretty Printer

在 `examples/11_recursive_enum.rs` 的 `JsonValue` 基础上：

- 支持完整的 JSON 字符串转义（`\n` / `\t` / `\\` / `\"`）
- 支持紧凑模式（一行）和缩进模式（按层级缩进）
- 实现 `serialize(&self) -> String` 和 `parse(s: &str) -> Result<JsonValue, ParseError>`

### 练习 7：命令模式 - 银行账户

实现一个银行账户的命令模式：

- `Account { id, balance }`
- `Command { Deposit(u64), Withdraw(u64), Transfer { to: u64, amount: u64 } }`
- `CommandError { InsufficientFunds, AccountNotFound, AmountZero }`
- `dispatch(&mut self, cmd) -> Result<(), CommandError>`
- 每条命令都要有完整的单元测试（happy path + 错误路径）

### 练习 8：内存布局观察

写一段代码，对照打印以下类型的 `size_of`：

- `Option<()>`、`Option<bool>`、`Option<u8>`、`Option<u32>`、`Option<u64>`
- `Option<&i32>`、`Option<Box<i32>>`、`Option<Vec<i32>>`、`Option<String>`
- `Result<i32, ()>`、`Result<(), i32>`、`Result<Box<i32>, ()>`

观察哪些类型享受了 NPO，并尝试解释原因。

---

## 要点总结

### enum 的本质

1. enum 表达 **OR**（一组互斥的可能性），struct 表达 **AND**（一组同时存在的字段）
2. 每个变体可以是**单元 / 元组 / 结构体**形态，可在同一 enum 里混搭
3. 变体可以携带任意类型的数据（包括其它 enum、Vec、Box、HashMap）
4. 整个 enum 的大小 ≈ 最大变体 + tag + 对齐 padding

### match 与穷尽性

1. `match` 是表达式，可以赋值、可以作返回值
2. **穷尽性检查**：编译器强制覆盖所有变体，新增变体会让所有 match 编译报错
3. `_` 通配 / 命名兜底要慎用——它们会让你失去新增变体的安全网
4. 模式不仅限于 match：`let` / `for` / 函数参数 / `if let` / `while let` / `let else` 全都接受模式

### 模式语法

| 语法 | 用途 |
|------|------|
| 字面量 | `0` / `'a'` / `"hello"` |
| 多模式 `\|` | `1 \| 2 \| 3` |
| 范围 `..=` | `0..=9` / `'a'..='z'` |
| `_` 通配 | 占据一个位置 |
| `..` 忽略 | 元组/结构体里跳过其余 |
| 命名绑定 | `x` / `name` —— 但小心 shadow 陷阱 |
| 守卫 `if` | `Some(n) if n > 0` |
| `@` 绑定 | `n @ 0..=12` 同时匹配 + 命名 |
| 解构嵌套 | `Account::Member { profile: Profile { city, .. }, .. }` |

### `if let` / `let else` / `while let` 选型

| 场景 | 推荐 |
|------|------|
| 单变体，其它一概忽略 | `if let` |
| 单变体 + fallback | `if let` / `else` |
| 必须匹配，否则早返回 | `let else` |
| 循环消费 | `while let` |
| 多个变体分别处理 | `match` |

### Option / Result

| 场景 | 类型 | 关键 API |
|------|------|---------|
| 可能没值 | `Option<T>` | `map / and_then / unwrap_or_else / ?` |
| 可能失败 | `Result<T, E>` | `map / and_then / map_err / ?` + `From` |
| 默认值 | `unwrap_or` / `unwrap_or_else` / `unwrap_or_default` |
| 链式 | 优先组合子，避免一堆 match |
| 短路 | `?` 运算符（要求函数也返回相应 enum） |

### enum 的工程套路

1. **状态机**：状态用 enum，事件用 enum，转移规则用 `match (state, event)`
2. **命令模式**：command 用 enum，state 用 struct，reducer 用穷尽 match
3. **错误聚合**：把所有可能错误装进一个 enum，配合 `From` + `?` 实现错误传播
4. **递归数据**：用 `Box` 打破无限递归，配合 `match` 写出递归求值
5. **Type-State**：把每种状态做成独立类型，让编译器拒绝非法转换

### 内存布局

1. 每个 enum 实例都有一个**判别式（discriminant）**作为 tag
2. enum 的总大小 = max(变体 payload) + tag + padding
3. **Niche / NPO**：`Option<&T>`、`Option<Box<T>>`、`Option<Vec<T>>` 等与裸版本一样大
4. `#[repr(u8/u16/i32/...)]` 控制 tag 宽度，FFI / 协议必备

### 最实用的判断准则

| 你的需求 | 推荐做法 |
|----------|----------|
| 一组互斥的可能性 | enum |
| 一组同时存在的字段 | struct |
| 表达"可能没有 / 可能失败" | `Option<T>` / `Result<T, E>` |
| 状态机 | enum + match |
| 命令 / 事件 / 路由 | enum + reducer |
| 递归数据 | enum + Box |
| 多个错误聚合 | enum + `From<E>` + `?` |
| 想穷尽性检查 | `match`（不要用 `_` 兜底） |
| 只关心一个变体 | `if let` |
| 必须匹配，否则早返回 | `let else` |
| 循环消费集合 | `while let` |
| HashMap 里用枚举做 key | derive `PartialEq, Eq, Hash` |
| 有默认变体 | derive `Default` + 标 `#[default]` |
| 跨 crate 扩展 | trait 对象 `dyn Trait`（不要硬塞 enum） |

---

> **下一步：** 掌握了枚举与模式匹配后，下一章 `09集合类型` 会带你认识 Rust 标准库里最常用的三大数据结构：`Vec<T>` / `String` / `HashMap<K, V>`。
> 你会发现 `Option<&T>` / `Result<T, E>` 在它们的 API 里无处不在——本章学的所有组合子和 `?` 运算符都将立即派上用场。
> 再下一章 `10错误处理` 会基于 `Result<T, E>` 展开自定义 Error、`thiserror` 和 `anyhow`，把你写出的错误类型推向工程级别。
