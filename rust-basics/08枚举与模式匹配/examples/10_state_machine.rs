//! 10. 用 enum 实现状态机：红绿灯 / 订单状态 / 网络连接
//!
//! 运行：cargo run --example 10_state_machine
//!
//! 本例覆盖：
//! - "状态 = enum 变体" 的核心思想
//! - 用方法表达"状态转移"
//! - 三种典型状态机：红绿灯（自动）、订单（事件驱动）、连接（带数据）
//! - Type-State 风格：用 enum + match 在编译期约束非法转换

#![allow(dead_code, unused_variables)]

use std::time::Duration;

// ============================================================================
// 1. 红绿灯：最简单的自动状态机
// ============================================================================
//
// 状态机的"状态"是一个有限集合，状态之间按规则转移：
//
//      +------+      30s       +-------+      3s        +--------+      30s
//      | Red  |  ─────────▶    | Green |  ─────────▶    | Yellow |  ─────────▶  Red
//      +------+                +-------+                +--------+
//
// 用 enum 直接把"状态"表达出来，每个变体就是一种状态。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    /// 状态转移规则
    fn next(self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    /// 当前状态持续多久
    fn duration(self) -> Duration {
        match self {
            TrafficLight::Red => Duration::from_secs(30),
            TrafficLight::Green => Duration::from_secs(30),
            TrafficLight::Yellow => Duration::from_secs(3),
        }
    }

    fn description(self) -> &'static str {
        match self {
            TrafficLight::Red => "停止",
            TrafficLight::Green => "通行",
            TrafficLight::Yellow => "准备停止",
        }
    }
}

// ============================================================================
// 2. 订单状态：事件驱动的状态机
// ============================================================================
//
// 现实业务里，状态转移往往是"事件触发"的，而不是"自动 tick"。
// 订单经典图：
//
//                 cancel              cancel              refund
//   Created ─────▶ Cancelled    Paid ───▶ Cancelled  Shipped ─▶ Refunded
//      │                         │                       │
//      │ pay                     │ ship                  │ deliver
//      ▼                         ▼                       ▼
//    Paid ────────────────────▶ Shipped ──────────▶  Delivered
//
// "非法转换"应该在编译期或运行期被拒绝（比如 Delivered 不能再 ship）。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderState {
    Created,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

#[derive(Debug)]
enum OrderEvent {
    Pay,
    Ship,
    Deliver,
    Cancel,
    Refund,
}

#[derive(Debug)]
enum TransitionError {
    InvalidTransition { from: OrderState, by: OrderEvent },
}

#[derive(Debug)]
struct Order {
    id: u64,
    state: OrderState,
}

impl Order {
    fn new(id: u64) -> Self {
        Order { id, state: OrderState::Created }
    }

    /// 事件触发的状态转移：用 match 把所有合法转换枚举出来，其它一律拒绝。
    fn apply(&mut self, event: OrderEvent) -> Result<(), TransitionError> {
        let new_state = match (&self.state, &event) {
            (OrderState::Created, OrderEvent::Pay) => OrderState::Paid,
            (OrderState::Created, OrderEvent::Cancel) => OrderState::Cancelled,
            (OrderState::Paid, OrderEvent::Ship) => OrderState::Shipped,
            (OrderState::Paid, OrderEvent::Cancel) => OrderState::Cancelled,
            (OrderState::Shipped, OrderEvent::Deliver) => OrderState::Delivered,
            (OrderState::Shipped, OrderEvent::Refund) => OrderState::Refunded,
            // 其它所有 (state, event) 组合都不合法
            (from, _) => {
                return Err(TransitionError::InvalidTransition {
                    from: *from,
                    by: event,
                });
            }
        };
        self.state = new_state;
        Ok(())
    }

    fn is_terminal(&self) -> bool {
        matches!(
            self.state,
            OrderState::Delivered | OrderState::Cancelled | OrderState::Refunded
        )
    }
}

// ============================================================================
// 3. 带数据的状态机：网络连接
// ============================================================================
//
// 不同状态可以"携带"不同的数据。
// 比如 TCP 连接：Disconnected 没有 socket，Connected 有 socket 和远端地址。
//
//   Disconnected → Connecting{addr} → Connected{addr, retries} → ...

#[derive(Debug)]
enum Connection {
    Disconnected,
    Connecting {
        host: String,
        port: u16,
    },
    Connected {
        host: String,
        port: u16,
        retries: u32,
    },
    Failed {
        host: String,
        reason: String,
    },
}

impl Connection {
    fn connect(host: String, port: u16) -> Self {
        Connection::Connecting { host, port }
    }

    fn complete(self) -> Self {
        match self {
            Connection::Connecting { host, port } => Connection::Connected {
                host,
                port,
                retries: 0,
            },
            // 其它状态不允许 complete，原样返回
            other => other,
        }
    }

    fn fail(self, reason: String) -> Self {
        match self {
            Connection::Connecting { host, .. } => Connection::Failed { host, reason },
            other => other,
        }
    }

    fn retry(self) -> Self {
        match self {
            Connection::Connected { host, port, retries } => Connection::Connected {
                host,
                port,
                retries: retries + 1,
            },
            other => other,
        }
    }

    fn close(self) -> Self {
        // 任何状态都可以 close，返回 Disconnected
        Connection::Disconnected
    }
}

// ============================================================================
// 4. Type-State 风格：让"非法转换"在编译期被拒绝
// ============================================================================
//
// 上面的 OrderState 是"运行期状态机"——非法转换会返回 Err。
// 还有一种更激进的写法：用单元结构体当类型标签，每种状态是一个独立类型，
// 只在合法状态上挂方法，让编译器在编译期就拒绝错误调用。
// 这部分内容会在 15 章稍微展开，这里只先放一个 enum 风格的"防御版本"。

/// 用 newtype 风格：把"已支付订单"封装成独立类型
#[derive(Debug)]
struct PaidOrder {
    id: u64,
    amount_cents: u32,
}

#[derive(Debug)]
struct ShippedOrder {
    id: u64,
    amount_cents: u32,
    carrier: String,
}

impl PaidOrder {
    fn ship(self, carrier: impl Into<String>) -> ShippedOrder {
        ShippedOrder {
            id: self.id,
            amount_cents: self.amount_cents,
            carrier: carrier.into(),
        }
    }
}

impl ShippedOrder {
    fn deliver(self) -> DeliveredOrder {
        DeliveredOrder {
            id: self.id,
            amount_cents: self.amount_cents,
            carrier: self.carrier,
        }
    }
    // ⚠️ 注意：ShippedOrder 上没有 ship 方法
    //   写 shipped.ship("DHL") 直接编译报错
}

#[derive(Debug)]
struct DeliveredOrder {
    id: u64,
    amount_cents: u32,
    carrier: String,
}

fn main() {
    println!("===== 1. 红绿灯（自动）=====");
    let mut light = TrafficLight::Red;
    for i in 1..=5 {
        println!(
            "  第 {i} 灯位 {:?} ({}, 持续 {:?})",
            light,
            light.description(),
            light.duration()
        );
        light = light.next();
    }

    println!("\n===== 2. 订单（事件驱动）=====");
    let mut o = Order::new(1001);
    println!("  初始: {:?}", o);

    for event in [
        OrderEvent::Pay,
        OrderEvent::Ship,
        OrderEvent::Deliver,
    ] {
        match o.apply(event) {
            Ok(()) => println!("  事件后: {:?}", o),
            Err(e) => println!("  非法转换: {:?}", e),
        }
    }
    println!("  是否终态: {}", o.is_terminal());

    // 终态后继续触发事件 —— 应该被拒绝
    let r = o.apply(OrderEvent::Cancel);
    println!("  在 Delivered 状态再 Cancel: {:?}", r);

    // 另一个轨迹：直接取消
    let mut o2 = Order::new(1002);
    let _ = o2.apply(OrderEvent::Cancel);
    println!("  Order 1002 创建后立刻 Cancel: {:?}", o2);

    println!("\n===== 3. 连接（带数据）=====");
    let c = Connection::Disconnected;
    println!("  初始: {c:?}");
    let c = Connection::connect("api.example.com".into(), 443);
    println!("  正在连接: {c:?}");
    let c = c.complete();
    println!("  已连接: {c:?}");
    let c = c.retry().retry();
    println!("  两次重试: {c:?}");
    let c = c.close();
    println!("  关闭: {c:?}");

    // 失败路径
    let c = Connection::connect("bad.host".into(), 80).fail("DNS 失败".into());
    println!("  失败路径: {c:?}");

    println!("\n===== 4. Type-State 风格 =====");
    let paid = PaidOrder { id: 9001, amount_cents: 19999 };
    println!("  paid: {paid:?}");
    let shipped = paid.ship("SF Express");
    println!("  shipped: {shipped:?}");
    let delivered = shipped.deliver();
    println!("  delivered: {delivered:?}");
    // 故意制造编译错误（取消注释会报错）：
    // let _bug = delivered.ship("Anyone"); // ❌ method `ship` not found

    println!("\n===== 要点回顾 =====");
    println!("· enum 的每个变体是一种状态，方法体里 match 表达转移规则");
    println!("· 事件驱动的状态机 = match (state, event)，非法组合返 Err");
    println!("· 不同状态可以携带不同数据，方法签名约束怎么用");
    println!("· 极致版：把每个状态做成独立类型 (Type-State)，编译期拦截错误");
}
