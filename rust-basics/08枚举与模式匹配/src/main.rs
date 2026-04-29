//! 第 08 章 枚举与模式匹配 —— 综合演示入口
//!
//! 运行：cargo run
//! 想看具体某个知识点：cargo run --example NN_xxxxx
//!
//! 本文件用一个 "餐厅订单系统" 把本章的核心概念串成一个最小完整流程：
//! - 用 enum 表达"菜品 / 折扣策略 / 订单状态"
//! - 用 match 处理状态转移和价格计算
//! - 用 Option / Result 做找菜 / 找不到 / 验证失败
//! - 用 if let / while let 简化处理
//! - 用 #[derive] 让结构体/枚举一次具备 Debug / Clone / 比较等能力

#![allow(dead_code)]

use std::collections::HashMap;

// ============================================================================
// 1. 数据模型
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    Drink,
    Main,
    Dessert,
}

#[derive(Debug, Clone)]
struct MenuItem {
    name: String,
    category: Category,
    price_cents: u32,
}

/// 折扣策略：典型的"互斥的可能性" → 用 enum
#[derive(Debug, Clone, Copy)]
enum Discount {
    None,
    Percent(u8),       // x% off，例如 Percent(10) = 9 折
    FixedAmount(u32),  // 直接抵 N 分
    Code(&'static str),
}

/// 订单状态：状态机 + enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderStatus {
    Drafting,
    Submitted,
    Paid,
    Served,
    Cancelled,
}

#[derive(Debug)]
struct Order {
    id: u64,
    items: Vec<MenuItem>,
    discount: Discount,
    status: OrderStatus,
}

#[derive(Debug)]
enum OrderError {
    EmptyOrder,
    AlreadySubmitted,
    AlreadyPaid,
    Cancelled,
    Forbidden(&'static str),
    UnknownItem(String),
}

// ============================================================================
// 2. Menu：用 HashMap 装菜单，演示 Option 查询
// ============================================================================

struct Menu(HashMap<String, MenuItem>);

impl Menu {
    fn demo() -> Self {
        let items = [
            MenuItem { name: "美式咖啡".into(), category: Category::Drink, price_cents: 1500 },
            MenuItem { name: "鲜榨橙汁".into(), category: Category::Drink, price_cents: 2200 },
            MenuItem { name: "牛肉汉堡".into(), category: Category::Main, price_cents: 4500 },
            MenuItem { name: "意面套餐".into(), category: Category::Main, price_cents: 5800 },
            MenuItem { name: "提拉米苏".into(), category: Category::Dessert, price_cents: 3200 },
        ];
        let mut map = HashMap::new();
        for it in items {
            map.insert(it.name.clone(), it);
        }
        Menu(map)
    }

    fn find(&self, name: &str) -> Option<&MenuItem> {
        self.0.get(name)
    }
}

// ============================================================================
// 3. 业务函数：折扣计算 / 订单状态转移
// ============================================================================

fn apply_discount(subtotal: u32, d: &Discount) -> u32 {
    // 折扣是几种互斥情况，match 是最自然的表达方式
    match d {
        Discount::None => subtotal,
        Discount::Percent(p) => {
            let kept = 100u32.saturating_sub(*p as u32);
            subtotal * kept / 100
        }
        Discount::FixedAmount(amount) => subtotal.saturating_sub(*amount),
        Discount::Code(code) => match *code {
            // Code 内部又是一组互斥策略，再来一层 match
            "WELCOME10" => subtotal * 9 / 10,
            "VIP25" => subtotal * 75 / 100,
            "FREE_DRINK" => subtotal,             // 这里简化，实际要看商品
            _ => subtotal,                         // 未知码视为没折扣
        },
    }
}

impl Order {
    fn new(id: u64) -> Self {
        Order {
            id,
            items: Vec::new(),
            discount: Discount::None,
            status: OrderStatus::Drafting,
        }
    }

    fn add_item(&mut self, item: MenuItem) -> Result<(), OrderError> {
        // 状态保护：已经提交以后不能再加菜
        if !matches!(self.status, OrderStatus::Drafting) {
            return Err(OrderError::Forbidden("订单已提交，不能添加商品"));
        }
        self.items.push(item);
        Ok(())
    }

    fn add_item_by_name(&mut self, menu: &Menu, name: &str) -> Result<(), OrderError> {
        // 演示 Option<&MenuItem> -> Result 的套路
        let item = menu
            .find(name)
            .cloned()
            .ok_or_else(|| OrderError::UnknownItem(name.into()))?;
        self.add_item(item)
    }

    fn set_discount(&mut self, d: Discount) -> Result<(), OrderError> {
        if !matches!(self.status, OrderStatus::Drafting) {
            return Err(OrderError::Forbidden("订单已提交，不能修改折扣"));
        }
        self.discount = d;
        Ok(())
    }

    fn submit(&mut self) -> Result<(), OrderError> {
        match self.status {
            OrderStatus::Drafting if self.items.is_empty() => Err(OrderError::EmptyOrder),
            OrderStatus::Drafting => {
                self.status = OrderStatus::Submitted;
                Ok(())
            }
            OrderStatus::Submitted => Err(OrderError::AlreadySubmitted),
            OrderStatus::Paid => Err(OrderError::AlreadyPaid),
            OrderStatus::Served => Err(OrderError::Forbidden("订单已上菜")),
            OrderStatus::Cancelled => Err(OrderError::Cancelled),
        }
    }

    fn pay(&mut self) -> Result<u32, OrderError> {
        if self.status != OrderStatus::Submitted {
            return Err(OrderError::Forbidden("只有 Submitted 订单能付款"));
        }
        let total = self.total();
        self.status = OrderStatus::Paid;
        Ok(total)
    }

    fn serve(&mut self) -> Result<(), OrderError> {
        match self.status {
            OrderStatus::Paid => {
                self.status = OrderStatus::Served;
                Ok(())
            }
            _ => Err(OrderError::Forbidden("还未付款，不能上菜")),
        }
    }

    fn cancel(&mut self) -> Result<(), OrderError> {
        match self.status {
            OrderStatus::Drafting | OrderStatus::Submitted => {
                self.status = OrderStatus::Cancelled;
                Ok(())
            }
            OrderStatus::Paid => Err(OrderError::Forbidden("已付款，请走退款流程")),
            OrderStatus::Served => Err(OrderError::Forbidden("已上菜，请走售后")),
            OrderStatus::Cancelled => Err(OrderError::Cancelled),
        }
    }

    fn subtotal(&self) -> u32 {
        self.items.iter().map(|i| i.price_cents).sum()
    }

    fn total(&self) -> u32 {
        apply_discount(self.subtotal(), &self.discount)
    }
}

fn yuan(cents: u32) -> String {
    format!("{}.{:02}", cents / 100, cents % 100)
}

// ============================================================================
// 4. 演示流程
// ============================================================================

fn run() -> Result<(), OrderError> {
    let menu = Menu::demo();

    println!("===== 1. 创建订单 + 加菜 =====");
    let mut order = Order::new(20240429);
    for name in ["美式咖啡", "牛肉汉堡", "意面套餐", "提拉米苏"] {
        match order.add_item_by_name(&menu, name) {
            Ok(()) => println!("  + {name}"),
            Err(e) => println!("  ! {name} 失败: {e:?}"),
        }
    }

    // 演示一次"找不到的菜"，让 Option / Result 走完一圈
    if let Err(e) = order.add_item_by_name(&menu, "鱼香肉丝") {
        println!("  (按预期不存在: {e:?})");
    }

    println!("\n===== 2. 设置折扣 =====");
    order.set_discount(Discount::Code("VIP25"))?;
    println!("  折扣 = {:?}", order.discount);

    println!("\n===== 3. 价格 =====");
    println!("  小计:   ¥{}", yuan(order.subtotal()));
    println!("  折后:   ¥{}", yuan(order.total()));

    println!("\n===== 4. 状态推进 =====");
    println!("  当前状态: {:?}", order.status);
    order.submit()?;
    println!("  submit() ok, 状态: {:?}", order.status);
    let paid = order.pay()?;
    println!("  pay() ok, 实付 ¥{}, 状态: {:?}", yuan(paid), order.status);
    order.serve()?;
    println!("  serve() ok, 状态: {:?}", order.status);

    println!("\n===== 5. 已经在 Served，再 cancel 应该被拒绝 =====");
    match order.cancel() {
        Ok(()) => println!("  (不应到这一步)"),
        Err(e) => println!("  按预期被拒: {e:?}"),
    }

    println!("\n===== 6. 走另一条流程：取消 =====");
    let mut o2 = Order::new(20240430);
    o2.add_item_by_name(&menu, "鲜榨橙汁")?;
    o2.cancel()?;
    println!("  o2 状态: {:?}", o2.status);

    println!("\n===== 7. while let 风格统计：把所有 Drink 价格累加 =====");
    let drinks = ["美式咖啡", "鲜榨橙汁"];
    let mut iter = drinks.iter();
    let mut total = 0u32;
    while let Some(name) = iter.next() {
        if let Some(item) = menu.find(name) {
            total += item.price_cents;
            println!("  + {name} ¥{}", yuan(item.price_cents));
        }
    }
    println!("  Drinks 合计: ¥{}", yuan(total));

    Ok(())
}

fn main() {
    println!("====== 第 08 章 综合演示：餐厅订单 ======\n");
    if let Err(e) = run() {
        eprintln!("[demo] 终端错误: {e:?}");
    }

    println!("\n→ 想深入某个知识点，可单独运行：");
    println!("    cargo run --example 01_enum_basics");
    println!("    cargo run --example 02_enum_with_data");
    println!("    cargo run --example 03_enum_methods");
    println!("    cargo run --example 04_match_basics");
    println!("    cargo run --example 05_match_patterns");
    println!("    cargo run --example 06_destructuring");
    println!("    cargo run --example 07_if_let_while_let");
    println!("    cargo run --example 08_option");
    println!("    cargo run --example 09_result_intro");
    println!("    cargo run --example 10_state_machine");
    println!("    cargo run --example 11_recursive_enum");
    println!("    cargo run --example 12_calculator");
    println!("    cargo run --example 13_command_pattern");
    println!("    cargo run --example 14_memory_layout");
    println!("    cargo run --example 15_advanced_patterns");
}
