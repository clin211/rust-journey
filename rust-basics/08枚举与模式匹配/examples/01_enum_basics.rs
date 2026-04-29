//! 01. 枚举基础：定义、变体、命名空间
//!
//! 运行：cargo run --example 01_enum_basics
//!
//! 本例覆盖：
//! - 最简枚举：单元变体（unit variant）
//! - 变体的命名空间（`Direction::Up`）
//! - 枚举值在 match 中的使用
//! - 枚举 vs 结构体：表达 OR vs AND
//! - 与 C / TS / Java 的对比

#![allow(dead_code)]

// ============================================================================
// 1. 最简枚举：单元变体
// ============================================================================
//
// 这是最朴素的枚举形态：只列举几种可能性，每个变体本身不携带数据。
// 它的用途等同于"有限的常量集合"，但比一堆 `const` 更安全：
// - 类型唯一：Direction 不会和 i32 / Status 互相误用
// - 穷尽性：写 match 时编译器会强制检查每个变体

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// ============================================================================
// 2. 变体的命名空间
// ============================================================================
//
// 每个变体都"住"在它的 enum 命名空间里：要写 `Direction::Up`，
// 不能直接写 `Up`（除非提前 `use Direction::*` 把变体导入进来）。
//
// 这避免了不同枚举之间的变体重名冲突 —— 两个 enum 都可以有 `Active`，
// 因为它们各自属于自己的命名空间。

#[derive(Debug)]
enum UserStatus {
    Active,
    Inactive,
}

#[derive(Debug)]
enum ServerStatus {
    Active,
    Inactive,
    Crashed,
}

// ============================================================================
// 3. 枚举上的方法
// ============================================================================
//
// 和结构体一样，枚举也能挂方法。本节只演示一个最简单的"反向"方法，
// 03_enum_methods 会展开更多用法。

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl TrafficLight {
    /// 信号灯下一个状态：红 → 绿 → 黄 → 红
    fn next(self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    /// 当前是否允许通过
    fn can_go(self) -> bool {
        matches!(self, TrafficLight::Green)
    }
}

// ============================================================================
// 4. enum vs 结构体：OR vs AND
// ============================================================================
//
// 这是初学者最容易困惑的一对：什么时候用 struct，什么时候用 enum？
//
//   struct = AND（合取）：所有字段同时存在
//   enum   = OR  （析取）：变体里只有一个会成立
//
// 举例：一段 HTTP 响应的状态
//   - 用 struct？ 那就要塞一堆 Option<...>，并且约定"如果 ok 就看 body，
//     失败就看 err_msg"，但编译器没办法帮你检查这种约定。
//   - 用 enum？  天然就是"成功 + 数据"或"失败 + 错误码"，
//     永远不可能两者都存在或都不存在。
//
// 这就是 enum 在 Rust 里被称为 "sum type / 代数数据类型 (ADT)" 的原因。

// ❌ 用 struct 模拟 OR：状态约定靠人记，编译器无能为力
struct ResponseStruct {
    is_ok: bool,
    body: Option<String>,    // 当 is_ok=true 时才有意义
    err_code: Option<i32>,   // 当 is_ok=false 时才有意义
}

// ✅ 用 enum 表达 OR：每个变体都自带它自己的数据
enum ResponseEnum {
    Ok(String),
    Err(i32),
}

fn handle(resp: ResponseEnum) -> String {
    match resp {
        ResponseEnum::Ok(body) => format!("OK: {body}"),
        ResponseEnum::Err(code) => format!("ERR: {code}"),
    }
    // 注意：编译器强制你处理这两种情况，不会漏掉。
    // 即使将来给 ResponseEnum 新增 Pending 变体，编译器也会立刻提醒你来这里补一手。
}

// ============================================================================
// 5. 与其它语言对比
// ============================================================================
//
//   C        : enum 只能表示一组整数常量，不能携带不同形状的数据
//   TS       : 用 union types 模拟，但运行时仍是动态的（无完整穷尽检查）
//   Java     : 通过抽象类 + 多个子类模拟，繁琐且打开扩展性
//   Go       : 早期用 interface{} + 类型断言，1.18 后用泛型/接口收紧
//   Haskell  : data 关键字 = 完整 ADT，Rust enum 在底层和它一脉相承
//
// Rust 的 enum 站在了 ADT 这一脉上，又把内存布局做到 C 一样紧凑（见 14_memory_layout）。

fn main() {
    println!("===== 1. 最简枚举 =====");
    let dir = Direction::Up;
    let light = TrafficLight::Red;
    println!("dir = {:?}, light = {:?}", dir, light);

    // 打印所有方向（演示 enum 是 Copy，可以多次使用）
    let all = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
    for d in all {
        println!("  {:?} 的反向是 {:?}", d, d.opposite());
    }

    println!("\n===== 2. 命名空间隔离 =====");
    let u = UserStatus::Active;
    let s = ServerStatus::Active;
    // u 和 s 看上去都是 "Active"，但它们是两种独立类型，不能互相赋值
    println!("UserStatus::Active   = {:?}", u);
    println!("ServerStatus::Active = {:?}", s);

    println!("\n===== 3. 信号灯循环 =====");
    let mut cur = TrafficLight::Red;
    for _ in 0..6 {
        println!("  当前: {:?}, 可通过: {}", cur, cur.can_go());
        cur = cur.next();
    }

    println!("\n===== 4. enum vs struct (OR vs AND) =====");
    let ok = ResponseEnum::Ok("hello".to_string());
    let err = ResponseEnum::Err(404);
    println!("{}", handle(ok));
    println!("{}", handle(err));

    // 故意保留这段 struct 写法的痛点演示，看就行，不推荐用：
    let _bad = ResponseStruct {
        is_ok: true,
        body: Some("hi".into()),
        err_code: None,
    };
    // let buggy = ResponseStruct { is_ok: true, body: None, err_code: Some(500) };
    //                                ^^^^^^^^   ^^^^^^^^^                ^^^^^^^^^
    //                                逻辑上自相矛盾，但编译器不会拦截你 ⚠️

    println!("\n===== 要点回顾 =====");
    println!("enum 表达 'OR'   —— 一组互斥的可能性");
    println!("struct 表达 'AND' —— 一组同时存在的字段");
    println!("两者搭配，足以建模绝大多数业务场景");
}
