# Rust 学习计划

> 基于《Rust程序设计（第2版）》（Programming Rust）和《Rust权威指南》（The Rust Programming Language）
> 每天约 2-3 小时，以"阅读 + 编码练习"交替推进
> 项目结构：`rust-basics/` 基础练习 | `the-book/` 跟随《Rust程序设计》实操

---

## 当前进度

- [x] Hello World (`rust-basics/01helloworld`)
- [x] 变量与基础 (`rust-basics/02变量与常量`)
- [x] 格式化输出 Display/Debug (`rust-basics/01helloworld`)

---

## 第一阶段：基础语法（Day 1 - Day 8）

### Day 1 — 基本数据类型

**阅读**

- 《Rust程序设计》第3章：基本类型
- 《Rust权威指南》第3.1-3.2节：变量与数据类型

**练习** — `rust-basics/03数据类型/`

- [x] 整数类型 (i8/u8/i32/u32/i64/u64/usize) 的范围与溢出
- [x] 浮点类型 f32/f64，演示精度差异
- [x] 布尔、字符 (char/Unicode) 类型
- [x] 类型转换 `as` 关键字：`let a: i32 = 42; let b: f64 = a as f64;`
- [x] 元组与数组：创建、访问、解构
- [x] 类型推断：对比显式标注 vs 让编译器推断

### Day 2 — 函数与控制流

**阅读**

- 《Rust程序设计》第2章（函数/控制流部分）
- 《Rust权威指南》第3.3-3.5节：函数、控制流

**练习** — `rust-basics/04functions/`

- [ ] 函数定义、参数、返回值（显式返回 vs 表达式返回）
- [ ] 语句 (statement) vs 表达式 (expression)：`let y = { let x = 3; x + 1 };`
- [ ] if/else if/else 表达式（注意 Rust 的 if 是表达式，可以赋值）
- [ ] loop（含 break 返回值）、while、for..in 循环
- [ ] 循环标签与 break/continue：`'outer: loop { ... break 'outer; }`

### Day 3 — 所有权（核心！）

**阅读**

- 《Rust程序设计》第4章：所有权与引用（重点精读）
- 《Rust权威指南》第4.1节：什么是所有权

**练习** — `rust-basics/05ownership/`

- [ ] String vs &str 的区别，堆 vs 栈
- [ ] 所有权转移：`let s1 = String::from("hello"); let s2 = s1;` → s1 失效
- [ ] 克隆：`s2 = s1.clone()` → 两者独立
- [ ] 函数传参时的所有权转移：传入 String 后原变量失效
- [ ] 函数返回值的所有权
- [ ] 练习：写一个 `fn takes_ownership(s: String)` 和 `fn gives_ownership() -> String`

### Day 4 — 引用与借用

**阅读**

- 《Rust程序设计》第4章（后半部分）
- 《Rust权威指南》第4.2节：引用与借用

**练习** — `rust-basics/06references/`

- [ ] 不可变引用 `&T`：多个不可变引用可以共存
- [ ] 可变引用 `&mut T`：同一时间只能有一个可变引用
- [ ] 悬垂引用：Rust 如何通过编译器阻止
- [ ] 引用规则：多个 & 或 一个 &mut，不能同时存在
- [ ] 切片 (slice)：字符串切片 `&str`、数组切片 `&[i32]`
- [ ] 练习：写一个 `fn first_word(s: &str) -> &str` 函数

### Day 5 — 结构体

**阅读**

- 《Rust程序设计》第5章（结构体部分）
- 《Rust权威指南》第5章：使用结构体组织数据

**练习** — `rust-basics/07structs/`

- [ ] 定义 struct、实例化、字段访问
- [ ] 字段初始化简写、结构体更新语法 `..user1`
- [ ] 元组结构体 `struct Color(i32, i32, i32)`
- [ ] 单元结构体 `struct AlwaysEqual;`
- [ ] 为 struct 实现方法 `impl` 块：`&self`、`&mut self`、`self`
- [ ] 关联函数 `String::from()` 风格：`impl Point { fn new() -> Self { ... } }`
- [ ] 练习：实现一个 `Rectangle` struct，包含 `area()` 和 `can_hold()` 方法

### Day 6 — 枚举与模式匹配

**阅读**

- 《Rust程序设计》第5章（枚举部分）+ 第6章
- 《Rust权威指南》第6章：枚举与模式匹配

**练习** — `rust-basics/08enums/`

- [ ] 定义 enum 和变体：`enum Direction { Up, Down, Left, Right }`
- [ ] 带数据的枚举：`enum Message { Quit, Echo(String), Move{x:i32,y:i32} }`
- [ ] `match` 表达式：穷尽性检查
- [ ] `if let` 简化单个变体匹配
- [ ] `while let` 模式
- [ ] Option<T> 枚举：Some/None，替代 null
- [ ] 练习：用 enum + match 实现一个简单的计算器（加减乘除 + 错误处理）

### Day 7 — 集合类型

**阅读**

- 《Rust程序设计》第2章（集合部分）
- 《Rust权威指南》第8章：常见集合

**练习** — `rust-basics/09collections/`

- [ ] Vec<T>：创建、push、索引、遍历、`get()` 安全访问
- [ ] String：创建、追加、拼接、遍历（字节/标量/字形簇）
- [ ] HashMap<K,V>：创建、插入、访问、遍历、`entry()` API
- [ ] 练习：统计一段文本中每个单词的出现次数（用 HashMap）

### Day 8 — 错误处理

**阅读**

- 《Rust程序设计》第8章
- 《Rust权威指南》第9章：错误处理

**练习** — `rust-basics/10errors/`

- [ ] `panic!` 不可恢复错误：何时使用
- [ ] `Result<T, E>` 枚举：Ok/Err
- [ ] `?` 运算符：错误传播
- [ ] `unwrap()` 和 `expect()` 的区别
- [ ] 自定义错误类型：实现 `std::error::Error` trait
- [ ] 练习：用 `?` 链式调用实现一个文件读取+解析函数

---

## 第二阶段：进阶特性（Day 9 - Day 16）

### Day 9 — 模块与包管理

**阅读**

- 《Rust程序设计》第9章
- 《Rust权威指南》第7章：管理项目

**练习** — `rust-basics/11modules/`

- [ ] `mod` 关键字定义模块、`pub` 控制可见性
- [ ] `use` 导入路径、`as` 别名、嵌套导入 `use std::io::{self, Read}`
- [ ] `pub use` 重导出
- [ ] 拆分模块到不同文件
- [ ] Cargo.toml 依赖管理、`[dev-dependencies]`
- [ ] 练习：创建一个包含多个子模块的项目，练习可见性控制

### Day 10 — 泛型

**阅读**

- 《Rust程序设计》第10章（泛型部分）
- 《Rust权威指南》第10.1节：泛型数据类型

**练习** — `rust-basics/12generics/`

- [ ] 泛型函数：`fn largest<T: PartialOrd>(list: &[T) -> &T`
- [ ] 泛型结构体：`struct Point<T> { x: T, y: T }`
- [ ] 泛型枚举：`Option<T>`、`Result<T, E>` 回顾
- [ ] 泛型方法：`impl<T> Point<T> { ... }` vs `impl Point<f32>`
- [ ] 练习：实现一个泛型栈 `Stack<T>`（push/pop/peek/is_empty）

### Day 11 — Trait

**阅读**

- 《Rust程序设计》第10章（Trait 部分）
- 《Rust权威指南》第10.2节：Trait

**练习** — `rust-basics/13traits/`

- [ ] 定义 trait：`trait Summary { fn summarize(&self) -> String; }`
- [ ] 为类型实现 trait：`impl Summary for Article`
- [ ] 默认实现
- [ ] trait 作为参数：`fn notify(item: &impl Summary)`
- [ ] trait bound：`fn notify<T: Summary>(item: &T)`
- [ ] 多个 trait bound：`+` 语法和 `where` 子句
- [ ] 练习：为自定义类型实现 `Display`、`FromStr`、`Iterator` trait

### Day 12 — 生命周期

**阅读**

- 《Rust程序设计》第4章（生命周期部分）
- 《Rust权威指南》第10.3节：生命周期

**练习** — `rust-basics/14lifetimes/`

- [ ] 生命周期标注语法 `'a`：函数签名中的生命周期
- [ ] 生命周期省略规则（三条规则）
- [ ] 结构体中的生命周期：`struct ImportantExcerpt<'a> { part: &'a str }`
- [ ] 生命周期子类型：`'static` 生命周期
- [ ] 练习：写一个 `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`

### Day 13 — 迭代器与闭包

**阅读**

- 《Rust程序设计》第11章
- 《Rust权威指南》第13章：迭代器与闭包

**练习** — `rust-basics/15iterators/`

- [ ] 闭包：`let add = |a, b| a + b;`
- [ ] 闭包捕获变量：所有权 / 不可变引用 / 可变引用
- [ ] `move` 闭包
- [ ] Iterator trait：`next()` 方法
- [ ] 适配器方法：map、filter、collect、fold、enumerate、zip
- [ ] 练习：用迭代器链式调用实现数据转换管道（filter → map → collect）

### Day 14 — 编写自动化测试

**阅读**

- 《Rust程序设计》第9章（测试部分）
- 《Rust权威指南》第11章：编写自动化测试

**练习** — `rust-basics/16testing/`

- [ ] `#[test]` 标注、`cargo test` 运行
- [ ] 断言宏：`assert!`、`assert_eq!`、`assert_ne!`
- [ ] 测试 panic：`#[should_panic]`
- [ ] 使用 `Result<T, E>` 在测试中
- [ ] 单元测试（同文件 `#[cfg(test)]`）vs 集成测试（`tests/` 目录）
- [ ] 练习：为之前写的 `Stack<T>` 补充完整的单元测试和集成测试

### Day 15 — 智能指针

**阅读**

- 《Rust程序设计》第10章（智能指针部分）
- 《Rust权威指南》第15章：智能指针

**练习** — `rust-basics/17smartpointers/`

- [ ] `Box<T>`：堆分配、递归类型
- [ ] `Rc<T>`：引用计数、共享所有权
- [ ] `RefCell<T>`：内部可变性、运行时借用检查
- [ ] `Deref` trait：自动解引用强制转换 (deref coercion)
- [ ] `Drop` trait：自动资源释放
- [ ] 练习：用 `Rc<RefCell<T>>` 实现一个双向链表节点

### Day 16 — 并发编程

**阅读**

- 《Rust程序设计》第13章
- 《Rust权威指南》第16章：无畏并发

**练习** — `rust-basics/18concurrency/`

- [ ] `spawn` 创建线程、`move` 闭包
- [ ] `mpsc` 通道：单生产者-多消费者
- [ ] `Mutex<T>`：互斥锁
- [ ] `Arc<T>`：原子引用计数（线程安全的 Rc）
- [ ] `Send` 和 `Sync` trait
- [ ] 练习：多线程并发下载模拟（用 channel 汇总结果）

---

## 第三阶段：实战项目（Day 17 - Day 24）

### Day 17 — 实战项目 I：minigrep

**阅读**

- 《Rust权威指南》第12章：I/O 项目 minigrep

**练习** — `the-book/minigrep/`

- [ ] 创建项目、接受命令行参数
- [ ] 读取文件内容
- [ ] 实现搜索逻辑（区分大小写/不区分）
- [ ] 使用 `Result` 和 `?` 做错误处理
- [ ] 提取到 lib.rs + 集成测试
- [ ] 使用环境变量 `CASE_INSENSITIVE`

### Day 18 — 实战项目 II：minigrep 完善 + Cargo 深入

**阅读**

- 《Rust权威指南》第14章：深入 Cargo

**练习** — 继续完善 `the-book/minigrep/`

- [ ] 发布 Profile：`dev` vs `release`
- [ ] 文档注释 `///` 和 `//!`，`cargo doc` 生成文档
- [ ] `pub use` 组织公开 API
- [ ] workspace 概念理解

### Day 19 — 实战项目 III：多线程 Web Server

**阅读**

- 《Rust权威指南》第21章：多线程 Web Server
- 《Rust程序设计》第13章（回顾）

**练习** — `the-book/webserver/`

- [ ] 监听 TCP 连接
- [ ] 解析 HTTP 请求
- [ ] 返回 HTML 页面
- [ ] 用线程池处理并发请求
- [ ] 优雅关闭

### Day 20 — 实战项目 IV：命令行工具

**阅读**

- 《Rust程序设计》第2章综合回顾

**练习** — `the-book/cli-tool/`

- [ ] 使用 `clap` crate 解析命令行参数
- [ ] 实现一个文件搜索/文本处理工具
- [ ] 彩色终端输出（`colored` crate）
- [ ] 完整的错误处理和用户提示

### Day 21 — 异步编程

**阅读**

- 《Rust程序设计》第14章：异步编程
- 《Rust权威指南》第17章：Async/Await

**练习** — `rust-basics/19async/`

- [ ] `async fn` 和 `.await` 语法
- [ ] `Future` trait 概念
- [ ] 使用 `tokio` 运行时
- [ ] 异步 HTTP 请求（`reqwest`）
- [ ] 练习：异步并发请求多个 URL 并汇总结果

### Day 22 — Cargo Workspace 项目组织

**阅读**

- 回顾《Rust程序设计》第9章 + 《Rust权威指南》第14章

**练习** — 重组项目结构

- [ ] 将 `rust-journey` 项目改造为 Cargo Workspace
- [ ] 共享依赖、统一版本管理
- [ ] 各子项目间相互引用

### Day 23 — 对象 oriented 特性

**阅读**

- 《Rust程序设计》第10章（回顾 Trait）
- 《Rust权威指南》第18章：OOP 特性

**练习** — `rust-basics/20oop/`

- [ ] 用 trait 对象实现多态：`dyn Draw`
- [ ] 状态模式：用 enum 替代传统 OOP 状态模式
- [ ] Trait 对象 vs 泛型（动态分发 vs 静态分发）
- [ ] 练习：实现一个 GUI 组件系统（用 trait 对象）

### Day 24 — 类型系统深入

**阅读**

- 《Rust程序设计》第10章
- 《Rust权威指南》第19章（类型系统部分）

**练习** — `rust-basics/21typesys/`

- [ ] 类型别名 `type Kilometers = i32;`
- [ ] Never 类型 `!`
- [ ] 动态大小类型 (DST) 和 `Sized` trait
- [ ] Newtype 模式：封装外部类型实现外部 trait
- [ ] 练习：用 newtype 模式实现类型安全的单位系统

---

## 第四阶段：高级主题（Day 25 - Day 30）

### Day 25 — 不安全 Rust

**阅读**

- 《Rust程序设计》第16章
- 《Rust权威指南》第19章（unsafe 部分）

**练习** — `rust-basics/22unsafe/`

- [ ] `unsafe` 块的五种能力：解引用裸指针、调用 unsafe 函数、访问可变静态变量、实现 unsafe trait、访问 union 字段
- [ ] 裸指针 `*const T` 和 `*mut T`
- [ ] FFI：调用 C 函数
- [ ] 练习：用 unsafe 实现一个简单的动态数组，对比安全封装

### Day 26 — 宏

**阅读**

- 《Rust程序设计》第15章
- 《Rust权威指南》第19章（宏部分）

**练习** — `rust-basics/23macros/`

- [ ] `macro_rules!` 声明宏：基本语法
- [ ] 宏 vs 函数的区别
- [ ] 过程宏 (procedural macro) 概念：派生宏、属性宏、函数宏
- [ ] `vec![]` 宏源码阅读
- [ ] 练习：写一个 `hashmap!` 宏实现 `hashmap!("a" => 1, "b" => 2)`

### Day 27 — 高级模式匹配

**阅读**

- 《Rust程序设计》第6章
- 《Rust权威指南》第19章（模式部分）

**练习** — `rust-basics/24patterns/`

- [ ] 所有可使用模式的位置
- [ ] 可反驳性 vs 不可反驳性
- [ ] 模式语法：字面量、变量、多模式 `|`、范围 `..=`、解构、守卫 `if`
- [ ] 绑定 `@` 运算符
- [ ] 练习：用复杂模式匹配实现一个简单表达式解析器

### Day 28 — 实战项目 V：完整 CLI 应用

**综合练习** — `the-book/todo-cli/`

- [ ] 使用 `clap` v4 定义 CLI 接口
- [ ] 实现增删改查功能
- [ ] JSON 文件持久化（`serde_json`）
- [ ] 完整的测试覆盖
- [ ] 错误处理与用户友好提示
- [ ] `cargo doc` 生成文档

### Day 29 — 实战项目 VI：简易 HTTP API

**综合练习** — `the-book/http-api/`

- [ ] 使用 `axum` 或 `actix-web` 创建 HTTP 服务
- [ ] 实现几个 REST API 端点
- [ ] JSON 序列化/反序列化
- [ ] 错误处理中间件
- [ ] 使用 `tokio` 异步运行时

### Day 30 — 总结回顾 + 进阶方向

**阅读**

- 回顾两本书的重点章节
- 浏览 Rust 官方 RFC 和标准库文档

**练习**

- [ ] 复习所有权、借用、生命周期（画内存模型图）
- [ ] 复习 trait 系统、泛型约束
- [ ] 复习并发模型（线程 + async）
- [ ] 整理笔记到 `rust-basics/` 各目录的 README.md
- [ ] 选择进阶方向：
  - Web 开发 → axum/actix-web 生态
  - 系统编程 → unsafe/FFI/嵌入式
  - 区块链/WASM → solana/cosmos/wasm-bindgen

---

## 学习原则

1. **每天必写代码** — 阅读理解后立即动手，不拖延
2. **遇到编译错误先自己思考** — Rust 编译器是最好的老师
3. **每个练习都跑 `cargo test`** — 养成测试习惯
4. **用 `cargo clippy` 检查代码质量** — 学习 Rust 惯用写法
5. **做笔记** — 在每个练习目录写 README.md 记录要点
6. **卡住了就看两本书的对应章节** — 交叉参考加深理解
