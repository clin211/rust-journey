#![allow(dead_code)]

use colored::*;
use std::mem::size_of_val;

// ─────────────────────────────────────────────────────────────────────────────
// 单元结构体（Unit-Like Structs）
//
// 单元结构体没有任何字段，形态上像单元类型 `()`，但它是一个有名字的类型。
//
//   语法：struct AlwaysEqual;
//   实例：let a = AlwaysEqual;
//
// 它的存在看起来毫无用处，但在 Rust 里有几个非常经典的用法：
//
//   1. 零大小类型（ZST，Zero-Sized Type）
//        · size_of_val(&AlwaysEqual) == 0
//        · 不占用任何内存，集合里可以"几乎免费"地存储它们
//
//   2. 作为 trait 的「挂载点」（marker / tag 类型）
//        · 想为某个"无数据但需要方法"的概念实现一个 trait
//        · 例：EventHandler、BotCommand、Phantom 类型等
//
//   3. 作为类型状态机的「状态标签」
//        · 通过 PhantomData<Tag> 记录某个对象当前的状态
//        · 在编译期防止在错误状态下调用错误的方法
//
//   4. 作为空的命名空间或模块级别的「工具结构体」
//        · 挂一堆静态方法，形成 Utils::do_xxx() 风格的调用
//        · 但 Rust 里更常用模块函数，Util 结构体并不是最佳实践
//
// 本示例会把这些场景一一展开。
// ─────────────────────────────────────────────────────────────────────────────

// ── 1. 最基础的单元结构体 ───────────────────────────────────────────────────
// 尾巴上的分号不是多余的，它告诉编译器「这是单元结构体，没有字段」
// 对比：
//   struct Foo;       // 单元结构体：没有字段
//   struct Bar {}     // 空的具名字段结构体（大多数时候等价，但语义不完全相同）
//   struct Baz();     // 空的元组结构体（较少见）
struct AlwaysEqual;

// 和 AlwaysEqual 等价但写法不同，Rust 里更鼓励用 `struct X;`
struct EmptyBrace {}

// ── 2. trait 挂载点：SimpleLogger 不需要任何字段，只需要挂几个方法 ──────────
// 设想我们想定义一个 trait Logger，里面有一个 `log()` 方法
// 对「固定行为、无状态」的实现类来说，单元结构体就是最合适的选择
trait Logger {
    fn log(&self, msg: &str);
    fn name(&self) -> &'static str;
}

struct SimpleLogger;

impl Logger for SimpleLogger {
    fn log(&self, msg: &str) {
        println!("  [SimpleLogger] {msg}");
    }
    fn name(&self) -> &'static str {
        "simple"
    }
}

struct LoudLogger;                          // 另一个实现，也是单元结构体

impl Logger for LoudLogger {
    fn log(&self, msg: &str) {
        println!("  [LoudLogger] !!! {} !!!", msg.to_uppercase());
    }
    fn name(&self) -> &'static str {
        "loud"
    }
}

// 动态分发：多个 Logger 放一起用 trait 对象
fn dispatch(loggers: &[&dyn Logger], msg: &str) {
    for l in loggers {
        println!("  派发到 logger \"{}\":", l.name());
        l.log(msg);
    }
}

// ── 3. 类型状态机的「状态 tag」──────────────────────────────────────────────
// 用 PhantomData<State> 记录状态，单元结构体作为状态标签
use std::marker::PhantomData;

struct Draft;                                // 状态：草稿
struct Published;                            // 状态：已发布

// 文章带一个状态参数 S，但这个 S 实际上不占空间（ZST）
struct Article<S> {
    title: String,
    body: String,
    _state: PhantomData<S>,                  // 记录状态，不占运行时空间
}

impl Article<Draft> {
    fn new(title: &str, body: &str) -> Self {
        Article {
            title: title.to_string(),
            body: body.to_string(),
            _state: PhantomData,
        }
    }

    // 只有 Draft 状态的文章才能调用 publish
    // 调用后状态变为 Published，原 Draft 实例被消费
    fn publish(self) -> Article<Published> {
        Article {
            title: self.title,
            body: self.body,
            _state: PhantomData,
        }
    }
}

impl Article<Published> {
    // 只有 Published 状态的文章才能 render
    fn render(&self) -> String {
        format!("<h1>{}</h1><p>{}</p>", self.title, self.body)
    }
}

// ── 4. 实现空 trait 做「类型安全标记」──────────────────────────────────────
// 标记 trait：有身份、没方法，用来在类型系统里加语义
trait Administrator {}                       // 管理员标记 trait
trait Guest {}                               // 游客标记 trait

struct Root;                                 // Root 是管理员
impl Administrator for Root {}

struct Visitor;                              // Visitor 是游客
impl Guest for Visitor {}

// 泛型函数只接受实现了 Administrator 的类型
fn admin_only<T: Administrator>(_who: T) {
    println!("  ✅ 具有管理员权限的操作被允许");
}

fn main() {
    println!("{}", "=== 单元结构体 (Unit-Like Structs) ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、基础语法：struct Foo; 没有字段");
    // ─────────────────────────────────────────

    let a = AlwaysEqual;                     // 直接写类型名就创建实例，无字段要填
    let b = AlwaysEqual;                     // 所有 AlwaysEqual 实例都是「同一份 0 大小」

    let a_size = size_of_val(&a);            // 0 bytes
    let b_size = size_of_val(&b);            // 0 bytes
    println!("  AlwaysEqual 实例 a: {a_size} 字节");
    println!("  AlwaysEqual 实例 b: {b_size} 字节");

    let e = EmptyBrace {};                   // 空大括号形式也可以
    println!("  EmptyBrace 实例:   {} 字节", size_of_val(&e));

    println!("  这就是典型的零大小类型（ZST）：概念存在，内存占用 = 0");
    println!("小结：单元结构体本质上是一个 ZST，用类型表达存在性而不是数据");

    // ─────────────────────────────────────────
    println!("\n2、用作 trait 挂载点：同一个方法的多种实现");
    // ─────────────────────────────────────────

    let s = SimpleLogger;
    let l = LoudLogger;

    s.log("hello world");
    l.log("hello world");

    // 把它们组合在一起批量派发
    let loggers: Vec<&dyn Logger> = vec![&s, &l];
    dispatch(&loggers, "ready to go");

    println!("  两个 Logger 都没有字段，只是一个「方法集合的名字」");
    println!("小结：trait 实现者无字段时，单元结构体是最合适的挂载点");

    // ─────────────────────────────────────────
    println!("\n3、标记 trait：用单元结构体做「权限身份」");
    // ─────────────────────────────────────────

    admin_only(Root);                        // ✅ Root 实现了 Administrator
    // admin_only(Visitor);                  // ❌ Visitor 没实现 Administrator

    println!("  Root 是 Administrator，可以调用 admin_only");
    println!("  Visitor 不是 Administrator，调用会在编译期被拒绝");
    println!("  这就是「标记 trait + 单元结构体」的组合拳");

    println!("小结：在类型系统里表达「身份」，没有字段时用单元结构体最自然");

    // ─────────────────────────────────────────
    println!("\n4、类型状态机（Type State）：编译期拒绝「错误状态的方法调用」");
    // ─────────────────────────────────────────

    // new() 返回 Article<Draft>
    let draft = Article::<Draft>::new("Rust 结构体", "内容 ...");

    // Draft 状态下不能 render，Rust 在编译期阻止你：
    // draft.render();                       // ❌ Article<Draft> 没有 render 方法

    // 必须先 publish 转换为 Published 状态，才能 render
    let published = draft.publish();
    let html = published.render();
    println!("  渲染结果: {html}");

    // Published 状态下不能再 publish（publish 在 Article<Draft> 上才有）
    // published.publish();                  // ❌ Article<Published> 没有 publish 方法

    println!("  Draft / Published 都是单元结构体，作为状态标签：");
    println!("    · 实例上零额外开销（PhantomData 不占空间）");
    println!("    · 状态错误在编译期拒绝（根本不存在 Draft 上的 render）");
    println!("  这是 Rust 里「零运行时开销」的状态机常见做法");

    println!("小结：单元结构体 + PhantomData，可在类型系统里建模「状态转换」");

    // ─────────────────────────────────────────
    println!("\n5、单元结构体的实例都相等 / 互换");
    // ─────────────────────────────────────────

    // 因为没有字段，两个 AlwaysEqual 实例在「数据上」其实是完全一样的
    // 但默认并没有实现 PartialEq，直接比较会编译错误：
    // assert_eq!(a, b);                     // ❌ AlwaysEqual 没实现 PartialEq

    // 如果派生一下 PartialEq，就可以愉快地比较了
    #[derive(Debug, PartialEq, Eq)]
    struct Marker;

    let m1 = Marker;
    let m2 = Marker;
    assert_eq!(m1, m2);                      // ✅ 两个 Marker 永远相等
    println!("  Marker 派生 PartialEq/Eq 后，所有实例都相等");

    println!("  想让单元结构体「可比较、可哈希」，通常一条 #[derive] 就够了");
    println!("小结：单元结构体的语义其实就是「零字段 → 完全相同」，很适合派生比较");

    // ─────────────────────────────────────────
    println!("\n6、单元结构体 vs 空结构体 vs 单元类型 `()`");
    // ─────────────────────────────────────────

    // 三者在形状上很像，但语义各有不同：
    //
    //   struct Foo;        // 单元结构体：有名字、0 字段，可以挂 impl 和 trait
    //   struct Foo {}      // 空具名字段结构体：行为基本一致
    //   struct Foo();      // 空元组结构体：可用但少见
    //   ()                 // 单元类型：语言内置的「没有意义的值」，也是 ZST
    //
    // 关键区别：
    //   · () 是语言内置类型，不能对它 impl 任何东西（孤儿规则）
    //   · 单元结构体是你自己的类型，可以 impl 自己的方法和 trait
    //
    // 所以「单元结构体」的价值 = ZST + 可命名 + 可挂行为
    println!("  单元结构体 = (ZST) + 可命名 + 可挂方法/trait");
    println!("  单元类型 () = 语言内置的无意义值，不能挂方法，常用作函数无返回值");
    println!("  这是单元结构体存在的真正意义");
    println!("小结：单元结构体 = 「有名字的 ZST」，是 Rust 抽象体系里的重要一环");

    // ─────────────────────────────────────────
    println!("\n【总结】单元结构体要点");
    // ─────────────────────────────────────────
    println!("  · 语法：struct Foo;（尾部分号是必须的）");
    println!("  · 大小：0 字节（ZST），编译优化后几乎没有任何运行时开销");
    println!("  · 用途①：trait 挂载点 —— 表示「无状态，只有方法集合」的类型");
    println!("  · 用途②：标记 trait —— 在类型系统表达身份/权限");
    println!("  · 用途③：状态标签 —— 与 PhantomData 组合做类型状态机");
    println!("  · 用途④：从 () 升级为命名类型，可挂 impl 和 trait");
    println!("  · 对比：`struct X;` 是最惯用的写法，不推荐 `struct X {{}}`");
}
