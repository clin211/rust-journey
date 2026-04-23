#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 综合练习：Rectangle（《Rust 权威指南》第 5 章经典例子）
//
// 本示例是整个第七章的「压轴练习」：
// 它把前面 01~09 讲过的几乎所有要点串成一个完整、可用、可读的 Rectangle 类型。
//
// 覆盖的知识点：
//   · 定义具名字段结构体（01_struct_basics）
//   · 字段初始化简写（02_field_init_shorthand）
//   · 结构体更新语法 ..base（03_struct_update_syntax）
//   · #[derive(Debug)] + {:?} / {:#?}（08_debug_and_derives）
//   · impl 块：&self / &mut self / self（06_methods）
//   · 关联函数 new / square / from_tuple（07_associated_functions）
//   · 结构体所有权 + 字段借用（09_ownership_in_structs）
//   · dbg! 宏调试（08_debug_and_derives）
//
// 完整能力：
//   · 计算面积 / 周长 / 对角线
//   · 判断是否包含、是否相交、是否为正方形
//   · 等比缩放、旋转（交换长宽）
//   · 多种构造方式：new、square、from_tuple、unit
//   · Debug 打印、clone、比较
// ─────────────────────────────────────────────────────────────────────────────

// ── Rectangle：一个功能相对完整的矩形类型 ───────────────────────────────────
// 派生 Debug / Clone / PartialEq：
//   · Debug  → 可以 println!("{:?}", r) 调试
//   · Clone  → 可以显式 .clone() 生成副本
//   · PartialEq → 两个矩形相等性比较（长宽都相同）
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ── 构造器（关联函数）────────────────────────────────────────────────────────
impl Rectangle {
    // 默认构造器：最常用的入口
    // 返回类型写 Self，提升可维护性；本质上就是 Rectangle
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // 正方形：边长相同的特殊情况，用专门的命名构造器更清晰
    fn square(side: u32) -> Self {
        Rectangle { width: side, height: side }
    }

    // 从元组构造：常见于解析后得到 (w, h) 的场景
    fn from_tuple(size: (u32, u32)) -> Self {
        let (width, height) = size;
        Self { width, height }               // 等价于 Rectangle { ... }
    }

    // 单位矩形：1x1，通常用作测试或默认值
    fn unit() -> Self {
        Rectangle { width: 1, height: 1 }
    }
}

// ── 只读方法：&self（计算与判断）─────────────────────────────────────────────
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // 对角线长度（浮点结果）：将 u32 转为 f64 再开方
    fn diagonal(&self) -> f64 {
        let w = self.width as f64;
        let h = self.height as f64;
        (w * w + h * h).sqrt()
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // 经典题目：self 能不能装下 other（严格大于，等长不算）
    // 在教材里这是 Rectangle 章节的标志性练习，务必亲手实现一遍
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 宽松版包含：边长可以相等（面积刚好能装进去的边界情况）
    fn can_hold_inclusive(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

// ── 修改自身的方法：&mut self ────────────────────────────────────────────────
impl Rectangle {
    // 按系数缩放两个维度
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 旋转 90°：交换宽高。这里用 std::mem::swap，避免手动引入中间变量
    fn rotate_90(&mut self) {
        std::mem::swap(&mut self.width, &mut self.height);
    }
}

// ── 消费 self 的方法：转换成其它类型 ────────────────────────────────────────
impl Rectangle {
    // 把自己转换成「以最长边为边长的正方形」
    // 消费原实例 → 返回新实例
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }

    // 拆成 (width, height) 的元组，消费自身
    fn into_tuple(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

fn main() {
    println!("{}", "=== 综合练习：Rectangle ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、多种方式构造 Rectangle");
    // ─────────────────────────────────────────

    // 方式 1：直接用结构体字面量（所有字段显式）
    let a = Rectangle { width: 30, height: 50 };

    // 方式 2：关联函数 new
    let b = Rectangle::new(10, 40);

    // 方式 3：square 快速构造正方形
    let sq = Rectangle::square(7);

    // 方式 4：从元组构造
    let t = Rectangle::from_tuple((8, 12));

    // 方式 5：单位矩形
    let u = Rectangle::unit();

    println!("  a  = {:?}", a);
    println!("  b  = {:?}", b);
    println!("  sq = {:?}", sq);
    println!("  t  = {:?}", t);
    println!("  u  = {:?}", u);

    println!("  一个功能丰富的类型通常要提供多种构造方式，提高调用方的可读性");
    println!("小结：结构体字面量 + 多种命名构造器，让代码既灵活又自文档");

    // ─────────────────────────────────────────
    println!("\n2、{{:?}} 与 {{:#?}} 打印");
    // ─────────────────────────────────────────

    println!("  {{:?}}  → {:?}", a);
    println!("  {{:#?}} → {:#?}", a);

    println!("  Debug 派生让 Rectangle 直接具备了开发调试能力");
    println!("小结：Debug 是结构体学习 / 开发期最常用的能力，务必派生");

    // ─────────────────────────────────────────
    println!("\n3、只读方法：area / perimeter / diagonal");
    // ─────────────────────────────────────────

    let r = Rectangle::new(3, 4);
    println!("  r              = {:?}", r);
    println!("  r.area()       = {}", r.area());        // 12
    println!("  r.perimeter()  = {}", r.perimeter());   // 14
    println!("  r.diagonal()   = {:.3}", r.diagonal()); // 5.000

    println!("  所有这些方法都是 &self：读完就结束，不修改 r");
    println!("小结：&self 方法负责「派生信息」，它们是 Rectangle 的主要对外表面");

    // ─────────────────────────────────────────
    println!("\n4、关系判断：can_hold / is_square");
    // ─────────────────────────────────────────

    let outer = Rectangle::new(10, 8);
    let inner = Rectangle::new(5, 6);
    let too_wide = Rectangle::new(11, 2);

    println!("  outer.can_hold(&inner)          = {}", outer.can_hold(&inner));      // true
    println!("  outer.can_hold(&too_wide)       = {}", outer.can_hold(&too_wide));   // false
    println!("  outer.can_hold_inclusive(&outer)= {}", outer.can_hold_inclusive(&outer)); // true (自包含)

    let s = Rectangle::square(9);
    println!("  s.is_square()                   = {}", s.is_square());               // true
    println!("  r.is_square()                   = {}", r.is_square());               // false

    println!("  can_hold 是《Rust 权威指南》里的经典题目：不修改、比较两个实例");
    println!("小结：判断类方法天然适合 &self + &other 组合，不涉及所有权转移");

    // ─────────────────────────────────────────
    println!("\n5、修改自身：scale / rotate_90");
    // ─────────────────────────────────────────

    let mut m = Rectangle::new(3, 4);
    println!("  初始       m = {:?}", m);

    m.scale(3);                              // 宽高各乘 3：9x12
    println!("  scale(3)    m = {:?}", m);

    m.rotate_90();                           // 交换宽高：12x9
    println!("  rotate_90() m = {:?}", m);

    println!("  注意：要调用 scale / rotate_90，m 必须是 let mut；");
    println!("  Rust 不允许对不可变绑定调用 &mut self 方法");
    println!("小结：修改类操作总是在 &mut self 中完成，调用者必须具备可变性");

    // ─────────────────────────────────────────
    println!("\n6、消费自身：into_square / into_tuple");
    // ─────────────────────────────────────────

    let r = Rectangle::new(6, 10);
    let s = r.into_square();                 // r 被消费（move 进方法）
    // println!("{:?}", r);                  // ❌ r 已失效
    println!("  r.into_square() 得到的 s = {:?}", s);

    let r2 = Rectangle::new(7, 8);
    let (w, h) = r2.into_tuple();
    println!("  r2.into_tuple() 得到的 (w, h) = ({w}, {h})");
    println!("  r2 此刻已被 move，无法再访问");

    println!("  命名惯例：消费 self 并转换形态的方法通常以 into_ 开头");
    println!("小结：self 方法将「当前实例」转化为「新值」，表达一次「终结变身」");

    // ─────────────────────────────────────────
    println!("\n7、Clone 与比较");
    // ─────────────────────────────────────────

    let r1 = Rectangle::new(5, 5);
    let r2 = r1.clone();                     // 显式深拷贝
    let r3 = Rectangle::new(6, 5);

    println!("  r1 == r2 → {}", r1 == r2);   // true
    println!("  r1 == r3 → {}", r1 == r3);   // false
    println!("  r1 仍可用: {:?}", r1);        // clone 不 move 原值
    println!("  r2 仍可用: {:?}", r2);

    println!("  Clone 在结构体里非常常见：当你需要「两份独立副本」时用 .clone()");
    println!("小结：PartialEq 让结构体可比较，Clone 让结构体可以安全多份使用");

    // ─────────────────────────────────────────
    println!("\n8、结构体更新语法：..base 派生新实例");
    // ─────────────────────────────────────────

    // Rectangle 所有字段都是 Copy（u32），是 ..base 最友好的情况
    let base = Rectangle::new(100, 80);
    let wider = Rectangle { width: 200, ..base };   // height 继承 base
    let taller = Rectangle { height: 400, ..base }; // width 继承 base

    println!("  base   = {:?}", base);
    println!("  wider  = {:?}", wider);
    println!("  taller = {:?}", taller);

    // ✅ 因为所有字段都是 Copy，base 仍然可用
    println!("  base 仍然可用: {:?}", base);

    println!("  字段全 Copy 的结构体，用 ..base 是最舒服的场景");
    println!("小结：修改「部分维度」时，..base 比全字段重写更简洁，也更显意图");

    // ─────────────────────────────────────────
    println!("\n9、dbg! 宏做调试输出");
    // ─────────────────────────────────────────

    let r = Rectangle::new(30, 50);

    // dbg! 打印到 stderr，同时附带文件 / 行号 / 表达式原文
    let w2 = dbg!(r.width * 2);              // 打印 r.width * 2 的值，并返回它
    let area_dbg = dbg!(r.area());           // 打印方法调用结果

    println!("  w2       = {w2}");
    println!("  area_dbg = {area_dbg}");
    println!("  dbg!(x) 会输出到 stderr，并返回 x 本身；非常适合「边打日志边计算」");
    println!("小结：dbg! 是日常开发最实用的调试武器，不用写 println! 手动拼字符串");

    // ─────────────────────────────────────────
    println!("\n10、把一切综合起来：小例子 —— 找出面积最大的矩形");
    // ─────────────────────────────────────────

    let rects = vec![
        Rectangle::new(3, 4),
        Rectangle::new(10, 2),
        Rectangle::square(5),                // 5x5 = 25
        Rectangle::from_tuple((7, 6)),       // 42
        Rectangle::new(8, 8),                // 64
    ];

    // 只读借用每个矩形，计算面积并找最大
    let largest = rects
        .iter()                              // 返回 &Rectangle 的迭代器
        .max_by_key(|r| r.area())            // 根据 area() 选最大
        .unwrap();

    println!("  所有矩形:");
    for r in &rects {
        println!("    {:?} area = {}", r, r.area());
    }
    println!("  面积最大的是: {:?}, area = {}", largest, largest.area());

    println!("  这段小代码用到了：关联函数 / 迭代器 / &self 方法 / 闭包 / Debug 派生");
    println!("小结：结构体配合迭代器 + 闭包，是 Rust 典型的数据处理管线");

    // ─────────────────────────────────────────
    println!("\n【总结】本章综合回顾");
    // ─────────────────────────────────────────
    println!("  · 数据：定义一个紧凑、语义清晰的 Rectangle 结构体");
    println!("  · 构造：new / square / from_tuple / unit 多种入口");
    println!("  · 行为：");
    println!("    - &self：area / perimeter / diagonal / is_square / can_hold");
    println!("    - &mut self：scale / rotate_90");
    println!("    - self：into_square / into_tuple");
    println!("  · 体验：Debug 打印、..base 更新、dbg! 调试、.clone()、== 比较");
    println!("  · 扩展：iter().max_by_key(...) 风格的函数式处理");
    println!();
    println!("  本章到此为止，你已经具备了独立设计 Rust 结构体的能力：");
    println!("    · 从定义字段开始 → 选择合适的派生 → 写构造器 → 补齐方法 → 结合所有权做转换");
    println!();
    println!("  下一步：08枚举与模式匹配 会把「多种变体的数据」组合起来建模，");
    println!("         结构体 + 枚举是 Rust 里表达业务语义最核心的两张牌。");
}

// ─────────────────────────────────────────────────────────────────────────────
// 单元测试：Rust 的内建测试能力
//
// #[cfg(test)] 是条件编译属性，告诉编译器「这段代码只在 cargo test 时编译」
//   · 正式发布时 tests 模块不会被编译进二进制，不增加产物体积
//   · 开发/测试时可以随时运行 cargo test 验证行为
//
// 运行方式：
//   cargo test --example 10_rectangle
//
// Rust 官方把测试放在同一个文件里是一种很强的约定：
//   · 被测代码和测试代码在同一个模块，可以访问 pub(crate) / 私有字段
//   · 改代码时测试就在旁边，不容易漏
//   · 文件不会膨胀太多（tests 模块放文件末尾）
//
// 这是 Rust「代码即测试」文化的直接体现，也是布道 Rust 时的加分项。
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    // super::* 引入父模块的所有内容，这样测试就能直接用 Rectangle
    use super::*;

    // ── 构造器测试 ─────────────────────────────────────────────────────────
    #[test]
    fn new_creates_rectangle_with_given_dimensions() {
        let r = Rectangle::new(3, 4);
        assert_eq!(r.width, 3);
        assert_eq!(r.height, 4);
    }

    #[test]
    fn square_creates_rectangle_with_equal_sides() {
        let s = Rectangle::square(5);
        assert_eq!(s.width, 5);
        assert_eq!(s.height, 5);
        assert!(s.is_square());
    }

    #[test]
    fn from_tuple_destructures_correctly() {
        let r = Rectangle::from_tuple((7, 2));
        assert_eq!(r, Rectangle { width: 7, height: 2 });
    }

    #[test]
    fn unit_rectangle_is_1x1() {
        assert_eq!(Rectangle::unit(), Rectangle::new(1, 1));
    }

    // ── 计算方法测试 ───────────────────────────────────────────────────────
    #[test]
    fn area_is_width_times_height() {
        assert_eq!(Rectangle::new(3, 4).area(), 12);
        assert_eq!(Rectangle::new(10, 10).area(), 100);
        assert_eq!(Rectangle::new(0, 5).area(), 0);
    }

    #[test]
    fn perimeter_is_twice_sum_of_sides() {
        assert_eq!(Rectangle::new(3, 4).perimeter(), 14);
        assert_eq!(Rectangle::new(5, 5).perimeter(), 20);
    }

    #[test]
    fn diagonal_of_3_4_is_5() {
        // 经典 3-4-5 直角三角形
        let r = Rectangle::new(3, 4);
        // 浮点比较要用误差范围，不能用 ==
        assert!((r.diagonal() - 5.0).abs() < 1e-9);
    }

    // ── 关系判断测试 ───────────────────────────────────────────────────────
    #[test]
    fn can_hold_strict_inequality() {
        let outer = Rectangle::new(10, 8);
        let inner = Rectangle::new(5, 6);

        assert!(outer.can_hold(&inner));
        // 边界情况：相等的矩形不能 hold（严格大于）
        assert!(!outer.can_hold(&outer));
        // 反向不成立
        assert!(!inner.can_hold(&outer));
    }

    #[test]
    fn can_hold_inclusive_allows_equality() {
        let r = Rectangle::new(10, 10);
        // inclusive 版本允许等于
        assert!(r.can_hold_inclusive(&r));
        assert!(r.can_hold_inclusive(&Rectangle::new(5, 5)));
    }

    #[test]
    fn is_square_detects_equal_sides() {
        assert!(Rectangle::new(5, 5).is_square());
        assert!(Rectangle::square(100).is_square());
        assert!(!Rectangle::new(3, 4).is_square());
    }

    // ── 修改方法测试 ───────────────────────────────────────────────────────
    #[test]
    fn scale_multiplies_both_dimensions() {
        let mut r = Rectangle::new(3, 4);
        r.scale(3);
        assert_eq!(r, Rectangle::new(9, 12));
    }

    #[test]
    fn rotate_90_swaps_width_and_height() {
        let mut r = Rectangle::new(3, 4);
        r.rotate_90();
        assert_eq!(r, Rectangle::new(4, 3));
        // 旋转两次回到原样
        r.rotate_90();
        assert_eq!(r, Rectangle::new(3, 4));
    }

    // ── 消费方法测试 ───────────────────────────────────────────────────────
    #[test]
    fn into_square_uses_longer_side() {
        let r = Rectangle::new(6, 10);
        let s = r.into_square();
        assert_eq!(s, Rectangle::square(10));
    }

    #[test]
    fn into_tuple_returns_width_and_height() {
        let r = Rectangle::new(7, 8);
        assert_eq!(r.into_tuple(), (7, 8));
    }

    // ── 派生行为测试：Clone 和 PartialEq ───────────────────────────────────
    #[test]
    fn clone_produces_equal_independent_instance() {
        let r1 = Rectangle::new(5, 5);
        let r2 = r1.clone();
        assert_eq!(r1, r2);
        // clone 不会让 r1 失效
        assert_eq!(r1.area(), 25);
    }

    #[test]
    fn partial_eq_checks_all_fields() {
        assert_eq!(Rectangle::new(3, 4), Rectangle::new(3, 4));
        assert_ne!(Rectangle::new(3, 4), Rectangle::new(4, 3));
    }

    // ── 集成式测试：多个功能组合 ───────────────────────────────────────────
    // 好的测试不仅验证单个方法，还验证「方法组合时语义依然正确」
    #[test]
    fn find_largest_by_iterator() {
        let rects = vec![
            Rectangle::new(3, 4),
            Rectangle::new(10, 2),
            Rectangle::square(5),
            Rectangle::from_tuple((7, 6)),
            Rectangle::new(8, 8),
        ];

        let largest = rects.iter().max_by_key(|r| r.area()).unwrap();
        assert_eq!(*largest, Rectangle::new(8, 8));
        assert_eq!(largest.area(), 64);
    }

    #[test]
    fn scale_then_rotate_works_correctly() {
        let mut r = Rectangle::new(3, 4);
        r.scale(3);          // 9 x 12
        r.rotate_90();       // 12 x 9
        assert_eq!(r, Rectangle::new(12, 9));
    }

    #[test]
    fn struct_update_syntax_preserves_other_fields() {
        let base = Rectangle::new(100, 80);
        let wider = Rectangle { width: 200, ..base };
        assert_eq!(wider, Rectangle::new(200, 80));
        // 因为 u32 是 Copy，base 还能继续用
        assert_eq!(base, Rectangle::new(100, 80));
    }

    // ── panic / should_panic 测试 ──────────────────────────────────────────
    // 演示错误场景：如果我们有一个会 panic 的方法
    // 这里构造一个辅助方法做演示（在实际 Rectangle 上没有这种行为）
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn scale_overflow_panics_in_debug() {
        // u32::MAX * 2 在 debug 模式下会 panic（integer overflow）
        // 在 release 模式下会 wrap around（这是 Rust 的故意设计）
        let mut r = Rectangle::new(u32::MAX, 1);
        r.scale(2);
        // 如果没触发 panic，这里不会到达（测试就会失败）
        let _ = r;
    }
}
