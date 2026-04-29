//! 02. 带数据的枚举：每个变体可以携带自己的数据
//!
//! 运行：cargo run --example 02_enum_with_data
//!
//! 本例覆盖：
//! - 三种变体形态：单元 / 元组 / 结构体
//! - 经典 Message 例子（来自《The Rust Book》）
//! - "每个变体都自己定义自己" 的内存布局直觉
//! - 变体里也可以放 enum / Vec / Box（嵌套）

#![allow(dead_code)]

use std::net::IpAddr;

// ============================================================================
// 1. 三种变体形态
// ============================================================================
//
// Rust 的枚举变体可以是这三种形态之一（甚至可以混合在同一个 enum 中）：
//
//   单元变体     Quit                      ← 不带数据
//   元组变体     Echo(String)              ← 一个或多个匿名字段，按位置访问
//   结构体变体   Move { x: i32, y: i32 }   ← 具名字段
//
// 三者在 match 时的解构方式不同，但本质都是 "枚举的一种形态"。

#[derive(Debug)]
enum Message {
    Quit,                                 // 单元变体：用户退出
    Echo(String),                         // 元组变体：广播文本
    Move { x: i32, y: i32 },              // 结构体变体：移动到坐标
    ChangeColor(u8, u8, u8),              // 元组变体：RGB
}

fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("用户退出"),
        Message::Echo(text) => println!("回声: {text}"),
        Message::Move { x, y } => println!("移动到 ({x}, {y})"),
        Message::ChangeColor(r, g, b) => println!("颜色 -> #{r:02x}{g:02x}{b:02x}"),
    }
}

// ============================================================================
// 2. 用 struct 替代等价信息——观察痛苦
// ============================================================================
//
// 如果你不用 enum，要表达 Message 这种 "互斥 + 各带各的字段" 的数据，就得搞一个肿胀的 struct：
//
//   struct MessageStruct {
//       kind: u8,                 // 用一个 tag 区分到底是哪种
//       echo_text: Option<String>,
//       x: Option<i32>,
//       y: Option<i32>,
//       r: Option<u8>, g: Option<u8>, b: Option<u8>,
//   }
//
// 缺点显而易见：
// - 每种 message 只用到部分字段，剩下的全是 None
// - "kind=Move 时 x,y 必须 Some" 这个约束编译器无法帮你保证
// - match 时要写一堆 if/else 检查 Option 是否为 Some
//
// enum 把这些痛苦从源头消除。

// ============================================================================
// 3. 经典例子：IP 地址
// ============================================================================
//
// 标准库的 std::net::IpAddr 就是用 enum + 元组变体实现的：
//
//   pub enum IpAddr {
//       V4(Ipv4Addr),
//       V6(Ipv6Addr),
//   }
//
// 这样你可以同时持有 v4 和 v6，但你拿到一个具体值时编译器会强制你区分处理。

#[derive(Debug)]
enum MyIpAddr {
    V4(u8, u8, u8, u8),          // 四段
    V6(String),                  // 简化：直接存字符串
}

fn describe_ip(addr: &MyIpAddr) -> String {
    match addr {
        MyIpAddr::V4(a, b, c, d) => format!("IPv4 {a}.{b}.{c}.{d}"),
        MyIpAddr::V6(s) => format!("IPv6 {s}"),
    }
}

// ============================================================================
// 4. 嵌套：变体里放 enum / Vec / Box
// ============================================================================
//
// 变体里能放任何类型——包括其它 enum、Vec、HashMap、甚至 Box<Self>（递归枚举）。
// 这里先演示一种业务里超常见的"事件"建模：

#[derive(Debug)]
enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
enum AppEvent {
    UserSignedIn { user_id: u64, ip: IpAddr },
    UserSignedOut(u64),
    ConfigChanged {
        key: String,
        old: Option<String>,    // Option<String> 也是个 enum
        new: Option<String>,
    },
    Log(LogLevel, String),      // 嵌套另一个 enum
    Errors(Vec<String>),        // 一次报告一组错误
}

fn handle_event(ev: &AppEvent) {
    match ev {
        AppEvent::UserSignedIn { user_id, ip } => {
            println!("[in]  user={user_id} from {ip}");
        }
        AppEvent::UserSignedOut(user_id) => {
            println!("[out] user={user_id}");
        }
        AppEvent::ConfigChanged { key, old, new } => {
            println!("[cfg] {key}: {old:?} -> {new:?}");
        }
        AppEvent::Log(level, msg) => {
            println!("[{level:?}] {msg}");
        }
        AppEvent::Errors(es) => {
            println!("[errs] ({} 条)", es.len());
            for (i, e) in es.iter().enumerate() {
                println!("  {}: {e}", i + 1);
            }
        }
    }
}

// ============================================================================
// 5. "每个变体一份内存" 的直觉
// ============================================================================
//
// 一个 enum 实例，在内存里只持有它实际成立的那个变体的数据。
// 但 enum 在编译时已经决定了"最多需要多大"——这取决于它最大的变体。
//
//                                          discriminant
//                                          (告诉运行时是哪个变体)
//                                                │
//                                                ▼
//   Message 实例的内存:    [tag][      payload (按最大变体对齐)      ]
//
//   Quit                   [ 0 ][          (空)                    ]
//   Echo(String)           [ 1 ][ ptr | len | cap                  ]   ← String 是 24B
//   Move { x, y }          [ 2 ][ x: i32 | y: i32                  ]   ← 8B
//   ChangeColor(u,u,u)     [ 3 ][ r | g | b                        ]   ← 3B
//
//   所有变体共享同一片 payload 区域，大小由"最大变体" + 对齐填充决定。
//   详见 14_memory_layout，这里只需建立直觉。

fn main() {
    println!("===== 1. Message 三种变体 =====");
    let msgs = [
        Message::Quit,
        Message::Echo("hello".to_string()),
        Message::Move { x: 3, y: 5 },
        Message::ChangeColor(0xff, 0x66, 0x00),
    ];
    for m in &msgs {
        process_message(m);
    }

    println!("\n===== 2. 自定义 IP 地址 =====");
    let v4 = MyIpAddr::V4(127, 0, 0, 1);
    let v6 = MyIpAddr::V6("::1".to_string());
    println!("{}", describe_ip(&v4));
    println!("{}", describe_ip(&v6));

    println!("\n===== 3. 嵌套：AppEvent =====");
    let events = [
        AppEvent::UserSignedIn {
            user_id: 1001,
            ip: "127.0.0.1".parse().unwrap(),
        },
        AppEvent::UserSignedOut(1001),
        AppEvent::ConfigChanged {
            key: "log.level".into(),
            old: Some("info".into()),
            new: Some("debug".into()),
        },
        AppEvent::Log(LogLevel::Warn, "接近内存上限".into()),
        AppEvent::Errors(vec![
            "数据库连接超时".into(),
            "缓存未命中".into(),
        ]),
    ];
    for ev in &events {
        handle_event(ev);
    }

    println!("\n===== 4. 内存大小直觉 =====");
    use std::mem::size_of;
    println!("size_of::<Message>()    = {} 字节", size_of::<Message>());
    println!("size_of::<MyIpAddr>()   = {} 字节", size_of::<MyIpAddr>());
    println!("size_of::<AppEvent>()   = {} 字节", size_of::<AppEvent>());
    println!("size_of::<LogLevel>()   = {} 字节", size_of::<LogLevel>());

    println!("\n===== 要点回顾 =====");
    println!("· 单元变体 / 元组变体 / 结构体变体 可以混搭");
    println!("· 每个变体可以携带自己的数据");
    println!("· 整个 enum 的大小 ≈ 最大变体 + tag (+ 对齐 padding)");
}
