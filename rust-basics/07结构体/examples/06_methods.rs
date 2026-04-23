#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 方法（Methods）：impl 块与 self 的三种形态
//
// 方法是「绑定在结构体（或枚举、trait 对象）上的函数」。
// 在 Rust 里，方法通过 `impl` 块挂到类型上，第一个参数必须是 self 的某种形式。
//
//   fn foo(self, ...)        → 消费实例，self 的所有权被移进方法
//   fn foo(&self, ...)       → 只读借用 self，不修改字段
//   fn foo(&mut self, ...)   → 可变借用 self，可以修改字段
//
// 与普通函数的区别：
//   · 调用方式：`x.foo(arg)`（方法调用语法） vs `foo(&x, arg)`（普通函数）
//   · 命名空间：方法属于类型，不会和其他类型的同名方法冲突
//   · 自动引用 / 自动解引用：x.foo() 会根据方法签名自动 & / &mut / 解引用
//
// 一个重要的直觉：
//   「self 相当于一个普通函数的第一个参数」，只是写法不同。
//   `self` 是 `self: Self`，`&self` 是 `self: &Self`，`&mut self` 是 `self: &mut Self`。
//
// 本示例通过一个 Counter 和 Rectangle 例子系统演示三种接收者。
// ─────────────────────────────────────────────────────────────────────────────

// ── Rectangle：只读方法与派生方法 ────────────────────────────────────────────
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self：只读访问字段，计算面积、周长、判断关系等「派生值」
    // 这是最常见的形式，日常 80% 以上的方法都是 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // can_hold：接收 &self 和 &other 两个引用，比较两个矩形
    // 返回 bool，不修改任何一个，典型的只读方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // &mut self：修改字段
    // 注意：调用它的变量必须是 mut（所有权可变），否则调不了这个方法
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // self（不带引用）：消费整个实例
    // 调用之后，原来的 Rectangle 就不能再用了（被 move 进方法）
    // 典型用途：把自身转换为另一种类型
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle {
            width: side,
            height: side,
        }
    }
}

// ── Counter：演示 &mut self 与可变性传递 ─────────────────────────────────────
struct Counter {
    value: u64,
    step: u64,
}

impl Counter {
    // 这里用字面量写一个简易构造器；关联函数会在 07_associated_functions 详细讲
    fn new(step: u64) -> Self {
        Counter { value: 0, step }
    }

    fn get(&self) -> u64 {
        self.value
    }

    fn increment(&mut self) {
        self.value += self.step;
    }

    fn reset(&mut self) {
        self.value = 0;
    }
}

// ── 多个 impl 块可以共存 ─────────────────────────────────────────────────────
// Rust 允许一个类型有多个 impl 块，通常用来把方法按「功能主题」分组
// 也便于在泛型场景下对不同的类型参数写不同的实现
impl Rectangle {
    // 「形状分类」相关的方法放在第二个 impl 块
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn is_tall(&self) -> bool {
        self.height > self.width
    }
}

fn main() {
    println!("{}", "=== 方法（Methods） ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、&self：最常用的只读方法");
    // ─────────────────────────────────────────

    let rect = Rectangle {
        width: 10,
        height: 4,
    };

    // 方法调用语法：rect.area() 等价于 Rectangle::area(&rect)
    println!("  area      = {}", rect.area());
    println!("  perimeter = {}", rect.perimeter());

    // 两种等价写法
    let a1 = rect.area();                    // 方法调用语法
    let a2 = Rectangle::area(&rect);         // 完全限定调用（显示传 &self）
    assert_eq!(a1, a2);
    println!("  rect.area() 和 Rectangle::area(&rect) 结果完全一致");

    println!("  &self 是不可变借用，rect 本身一直可用，也能并发读");
    println!("小结：只读计算派生值，首选 &self 方法");

    // ─────────────────────────────────────────
    println!("\n2、&self 比较两个实例：can_hold");
    // ─────────────────────────────────────────

    let big = Rectangle { width: 100, height: 50 };
    let small = Rectangle { width: 30, height: 20 };
    let thin = Rectangle { width: 101, height: 1 };

    println!("  big.can_hold(&small) = {}", big.can_hold(&small)); // true
    println!("  big.can_hold(&thin)  = {}", big.can_hold(&thin));  // false

    println!("  注意：other 参数写成 `&Rectangle`，调用时传 `&small`");
    println!("  比较操作绝不会修改数据，双方都用「只读借用」最合适");
    println!("小结：比较 / 读取多个实例的操作，标配是 &self + &other");

    // ─────────────────────────────────────────
    println!("\n3、&mut self：修改自身状态");
    // ─────────────────────────────────────────

    // 要调用 &mut self 的方法，变量本身必须声明为 mut
    let mut rect = Rectangle { width: 3, height: 4 };
    println!("  缩放前: {}x{}", rect.width, rect.height);
    rect.scale(5);                           // width=15, height=20
    println!("  缩放后: {}x{}", rect.width, rect.height);

    // 常见误区：变量不是 mut 时调用 &mut self 的方法
    let immutable_rect = Rectangle { width: 1, height: 2 };
    // immutable_rect.scale(2); // ❌ cannot borrow `immutable_rect` as mutable
    println!("  ⚠️ immutable_rect 不是 mut，不能调用 scale（它要 &mut self）");
    let _ = immutable_rect.area();           // ✅ &self 方法仍然可以调

    println!("  Rust 自动根据方法签名选择 &/&mut 借用，调用处不用写 & 或 &mut");
    println!("小结：修改状态的方法用 &mut self，并且变量本身必须 mut");

    // ─────────────────────────────────────────
    println!("\n4、self：消费实例，转换为另一个类型 / 形态");
    // ─────────────────────────────────────────

    let rect = Rectangle { width: 10, height: 4 };
    let square = rect.into_square();         // rect 被 move 进方法，之后不可用

    // println!("{}", rect.area());          // ❌ rect 已被 move
    println!("  rect.into_square() 之后 rect 已失效");
    println!("  square.area() = {}", square.area());

    // 命名惯例：消费 self 并转换形态的方法，通常以 into_ 开头
    //   - into_xxx(self) → T      消费自己，转换成 T
    //   - from_xxx(source) → Self 关联函数，把外部数据转成 Self
    //   - as_xxx(&self)   → &T    仅借用 self 的「视图」
    println!("  命名惯例：into_* 消费 self；as_* 借用；from_* 接收外部数据");
    println!("小结：要做类型转换或「结束生命」的方法，用 `self`");

    // ─────────────────────────────────────────
    println!("\n5、多个 impl 块：按主题拆分方法");
    // ─────────────────────────────────────────

    let square_like = Rectangle { width: 5, height: 5 };
    let tall = Rectangle { width: 2, height: 10 };
    let wide = Rectangle { width: 10, height: 2 };

    println!("  5x5   is_square = {}, is_tall = {}",
        square_like.is_square(), square_like.is_tall());
    println!("  2x10  is_square = {}, is_tall = {}",
        tall.is_square(), tall.is_tall());
    println!("  10x2  is_square = {}, is_tall = {}",
        wide.is_square(), wide.is_tall());

    println!("  一个类型可以有多个 impl 块，编译器会合并它们");
    println!("  典型用法：按「数据计算」、「分类判断」、「I/O」、「trait 实现」分开写");
    println!("小结：多 impl 块让大型结构体的方法集更容易阅读和组织");

    // ─────────────────────────────────────────
    println!("\n6、Counter 示例：&mut self 的完整闭环");
    // ─────────────────────────────────────────

    let mut c = Counter::new(2);             // step = 2
    println!("  初始 value = {}", c.get());

    for _ in 0..5 {
        c.increment();
    }
    println!("  迭代 5 次后 value = {}", c.get()); // 0 + 2*5 = 10

    c.reset();
    println!("  重置后 value = {}", c.get());

    println!("  一个小而完整的状态机：new 创建 → get 读 → increment 写 → reset 清");
    println!("小结：实际项目中大量使用 &mut self 封装状态变更");

    // ─────────────────────────────────────────
    println!("\n7、自动引用 / 自动解引用：调用时不用写 &");
    // ─────────────────────────────────────────

    let rect = Rectangle { width: 8, height: 3 };

    // 看似我们什么都没加，实际上 Rust 自动帮我们处理了引用
    rect.area();                             // 等价于 (&rect).area()
    // rect.scale(2);                        // ❌ rect 不是 mut，所以自动 &mut 转换失败

    // 即使我们手上只有一个 &Rectangle，调用 &self 方法仍然没问题
    let r_ref: &Rectangle = &rect;
    println!("  r_ref.area() = {} （自动处理，就像 rect.area()）", r_ref.area());

    // 即使我们手上是 &mut Rectangle，调用 &self 方法也可以
    // 因为 &mut T 可以被「临时降级」为 &T 借用
    let mut r_mut = Rectangle { width: 6, height: 2 };
    let r_mut_ref: &mut Rectangle = &mut r_mut;
    println!("  r_mut_ref.area() = {} （&mut 可向下兼容为 &）", r_mut_ref.area());

    // Rust 的这个特性叫做「自动引用 / 自动解引用」（automatic referencing）
    // 调用方法时，编译器会自动在调用者前插入 & / &mut / *，
    // 以匹配方法签名里 self 的形态。
    println!("  Rust 根据方法签名自动插入 & / &mut / *，省了一堆 & 符号");
    println!("小结：方法调用无需手写 &/&mut，语义来自方法签名里 self 的形态");

    // ─────────────────────────────────────────
    println!("\n8、直接通过引用调方法 vs 通过值调方法");
    // ─────────────────────────────────────────

    let r = Rectangle { width: 2, height: 3 };

    // 调用 &self 方法：值 / 引用都可以
    r.area();
    (&r).area();

    // 调用消费 self 的方法：必须是「值」而不是 &
    // (&r).into_square();                   // ❌ 不能从 &T 消费 self
    let _sq = r.into_square();               // ✅ r 本身作为值传入（被 move）
    println!("  调用消费 self 的方法：调用者必须持有「值」而不是引用");
    println!("小结：&self / &mut self 方法可以通过引用调用，self 方法必须有值");

    // ─────────────────────────────────────────
    println!("\n【总结】方法的三种接收者");
    // ─────────────────────────────────────────
    println!("  · {:<14} → 只读访问，最常用（读字段、计算派生值、比较）", "&self");
    println!("  · {:<14} → 可变访问，修改字段（mutations，需要调用者是 mut）",   "&mut self");
    println!("  · {:<14} → 消费自身，转换形态（into_xxx，结束当前实例）",        "self");
    println!("  · 方法调用语法 x.foo() 等价于 T::foo(&x)（自动引用 / 解引用）");
    println!("  · 多个 impl 块可以共存，方便按主题组织方法");
    println!("  · 命名惯例：getter 用 &self；setter / mutator 用 &mut self；into_ 用 self");
}
