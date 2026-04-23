#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 结构体所有权（Ownership in Structs）
//
// 结构体不只是一个数据容器，它同时是「所有权的边界」。
// 本示例把结构体和所有权的几个关键交互彻底讲清楚：
//
//   1. 结构体拥有其字段 —— 实例 drop 时，字段级联 drop
//   2. 结构体字段的 move / borrow 规则，和变量几乎一致
//   3. 部分 move（partial move）：结构体允许某些字段被 move，而其它字段仍可使用
//   4. String 字段 vs &str 字段：到底什么时候选哪个？
//   5. 借用字段 vs 借用整体实例
//   6. 自引用结构体的直觉（为什么 Rust 让它变得困难）
//
// 理解这些之后，你就能自然地在「拥有 String 还是借用 &str」之类问题上做出选择。
// ─────────────────────────────────────────────────────────────────────────────

// ── 拥有 String 字段的 User ─────────────────────────────────────────────────
// 所有字段都由结构体本身拥有：实例 drop 时，name / email 的堆内存一起释放
// 好处：User 独立存在，不依赖任何外部数据
// 代价：构造实例必须传入 String（或通过 .to_string() / String::from 产生）
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// ── 持有 &str 字段（借用字符串）的 UserRef ──────────────────────────────────
// 这个结构体不拥有字符串，只是「借用」它们
// 需要引入生命周期参数 'a，表示这个借用能活多久
// 好处：避免字符串的复制；若数据是只读的，不需要拥有权
// 代价：UserRef 的实例不能比它借用的字符串活得更长
#[derive(Debug)]
struct UserRef<'a> {
    id: u64,
    name: &'a str,
    email: &'a str,
}

// ── 一个混合字段类型的结构体：演示部分 move ─────────────────────────────────
#[derive(Debug)]
struct Document {
    title: String,                           // 非 Copy：move 后原字段不可再用
    content: String,                         // 同上
    length: u32,                             // Copy：不参与 move
    public: bool,                            // Copy：不参与 move
}

// ── Drop 可视化：观察资源释放顺序 ───────────────────────────────────────────
// 实现 Drop trait 之后，每次 drop 都会打印一行，便于「看见」所有权终结
struct Logged {
    name: &'static str,
}

impl Drop for Logged {
    fn drop(&mut self) {
        println!("  Drop::drop called for {}", self.name);
    }
}

struct Owner {
    a: Logged,
    b: Logged,
}

fn main() {
    println!("{}", "=== 结构体所有权 ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、结构体「拥有」其字段：实例 drop 时级联释放");
    // ─────────────────────────────────────────

    {
        let user = User {
            id: 1,
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
        };
        println!("  user 在块内：{:?}", user);
    } // user 在这里离开作用域 → user.name / user.email 的堆内存随之释放

    println!("  块外无法再访问 user，字段堆内存已在块结束时释放");
    println!("  这是 Rust 的 RAII：结构体拥有字段 → 字段生命周期 = 结构体的生命周期");
    println!("小结：结构体实例 drop 时，自动 drop 所有字段（按声明顺序的反序）");

    // ─────────────────────────────────────────
    println!("\n2、整个结构体实例的 move");
    // ─────────────────────────────────────────

    let u1 = User {
        id: 2,
        name: "Bob".into(),
        email: "bob@x.com".into(),
    };

    let u2 = u1;                             // 整个 u1 move 到 u2
    // println!("{:?}", u1);                 // ❌ u1 已经 move
    println!("  u2 = {:?}", u2);
    println!("  ⚠️ 原 u1 已经被 move，无法再访问");

    println!("  普通结构体和 String 一样会 move：");
    println!("    - 没有 #[derive(Copy)]");
    println!("    - 字段里包含非 Copy 的数据（比如 String）");
    println!("小结：结构体默认 move 语义，与 String/Vec 完全一致");

    // ─────────────────────────────────────────
    println!("\n3、传函数参数：三种契约");
    // ─────────────────────────────────────────

    // 传值：move 进函数
    fn take(u: User) -> User {
        println!("    take: {:?}", u);
        u                                    // 再把所有权还回来
    }

    // 只读借用
    fn peek(u: &User) {
        println!("    peek: {:?}", u);
    }

    // 可变借用
    fn bump_id(u: &mut User) {
        u.id += 1;
    }

    let mut user = User {
        id: 10,
        name: "Carol".into(),
        email: "c@x.com".into(),
    };

    peek(&user);                             // 借用，不 move
    bump_id(&mut user);                      // 可变借用
    let user = take(user);                   // move 进 take，再接收返回值
    println!("  最终 user = {:?}", user);

    println!("  跟基本类型一样：传值 move、&借用、&mut 可变借用");
    println!("小结：结构体在所有权/借用系统里没有特殊地位，规则一模一样");

    // ─────────────────────────────────────────
    println!("\n4、部分 move（partial move）：只把某个字段拿走");
    // ─────────────────────────────────────────

    let doc = Document {
        title: String::from("Rust 结构体"),
        content: String::from("......"),
        length: 8,
        public: true,
    };

    // 把 title 字段 move 出来，content 仍在 doc 里
    let stolen_title: String = doc.title;

    // 从这一刻起，doc 作为「整体」已经不再完整可用
    // println!("{:?}", doc);                // ❌：title 已被 move，doc 不完整

    // 但仍然可以访问「没被 move」的字段：
    //   - doc.content：非 Copy，但没有被 move，仍然可以单独访问（甚至再 move）
    //   - doc.length / doc.public：Copy 字段，本就不参与 move，永远可读
    println!("  stolen_title = {stolen_title}");
    println!("  doc.content  = {}", doc.content);   // ✅ 没被 move，可单独访问
    println!("  doc.length   = {}", doc.length);    // ✅ Copy
    println!("  doc.public   = {}", doc.public);    // ✅ Copy

    // 再把 content 也 move 出来
    let stolen_content = doc.content;
    // 至此 doc 的两个 String 都被拿走，但 Copy 字段还能读
    // println!("{:?}", doc);                // ❌ doc 作为整体仍然不可打印
    println!("  stolen_content = {stolen_content}");
    println!("  ⚠️ doc 作为整体不能再使用，但 Copy 字段（length/public）仍可读");

    println!("  ✅ 实用直觉：结构体的 move 是「字段级」的，不是「整体」的");
    println!("小结：部分 move 让你可以「拆掉一个结构体，把需要的字段拿出来用」");

    // ─────────────────────────────────────────
    println!("\n5、String 字段 vs &str 字段：怎么选？");
    // ─────────────────────────────────────────

    // 拥有 String 的 User：独立存在，字符串由 User 持有
    let owned = User {
        id: 1,
        name: "owned".into(),
        email: "owned@x.com".into(),
    };

    // 借用 &str 的 UserRef：只是一张「看见外部字符串」的名片
    let n = String::from("borrowed-name");
    let e = "borrowed@static";               // 字面量是 &'static str
    let borrowed = UserRef {
        id: 2,
        name: &n,                            // 借用 n
        email: e,                            // 借用字面量
    };

    println!("  owned  = {:?}", owned);
    println!("  borrowed = {:?}", borrowed);

    // 借用的 UserRef 不能比数据活得更久
    // 若试图把 borrowed move 出 n 的作用域，Rust 会阻止你
    // 关键对比：
    //   - User（持有 String）：独立生命周期、可长期保存、代价是 String 分配
    //   - UserRef（持有 &str）：依赖外部字符串、生命周期受限、零拷贝
    //
    // 经验法则：
    //   · 结构体要「独立存在、长时间保存、跨线程传」 → 用 String / 拥有型字段
    //   · 结构体只是「一次处理的视图，跟随数据生命周期」→ 用 &str / 借用型字段
    println!("  经验：通常默认用 String 字段，必须零分配时再考虑 &str + 生命周期");
    println!("小结：String 字段「自主独立」；&str 字段「生命周期受限，但不复制」");

    // ─────────────────────────────────────────
    println!("\n6、借用字段：让借用精准而不「整块借」");
    // ─────────────────────────────────────────

    let u = User {
        id: 1,
        name: "Dana".into(),
        email: "d@x.com".into(),
    };

    // 借用「整个 User」
    let u_ref: &User = &u;
    println!("  u_ref.name = {}", u_ref.name);

    // 借用「某个字段」
    let name_ref: &String = &u.name;
    println!("  name_ref   = {name_ref}");

    // 可以同时借用多个不同的字段（Rust 支持分字段借用）
    // 注意：是「不同字段」才可以同时 &mut，同一字段不能同时 &mut 两次
    fn split_borrow(u: &mut User) -> (&mut u64, &mut String) {
        (&mut u.id, &mut u.name)             // 同一结构体的两个「不同字段」可同时 &mut
    }

    let mut u2 = User {
        id: 42,
        name: "Ego".into(),
        email: "e@x.com".into(),
    };
    let (id_mut, name_mut) = split_borrow(&mut u2);
    *id_mut += 1;
    name_mut.push_str("!");
    println!("  分字段借用后：u2.id = {}, u2.name = {}", u2.id, u2.name);

    println!("  Rust 的借用分析是「字段级」的，不同字段可以同时被可变借用");
    println!("小结：结构体支持对字段各自独立借用，这是 Rust 的强项而不是限制");

    // ─────────────────────────────────────────
    println!("\n7、Drop 顺序：从「持有者」到「被持有」");
    // ─────────────────────────────────────────

    // Owner { a: Logged, b: Logged } 中：
    //   - 先按「声明相反顺序」drop 字段：先 drop b，再 drop a
    //   - 最外层的 owner 本体 drop 前，字段们已经全部 drop 完毕
    println!("  进入块，创建 Owner：");
    {
        let _o = Owner {
            a: Logged { name: "a" },
            b: Logged { name: "b" },
        };
        println!("    (块内：Owner 实例已创建)");
    } // _o 离开作用域，开始 drop
    println!("  块结束，观察上面的 drop 顺序：字段按声明逆序释放（b 先，a 后）");

    println!("  这是 RAII：资源的释放顺序由结构体的字段顺序决定");
    println!("小结：Rust 的 drop 顺序是确定的、可预测的，实际项目里这点很重要");

    // ─────────────────────────────────────────
    println!("\n8、自引用结构体：为什么它在 Rust 里很棘手");
    // ─────────────────────────────────────────

    // 一个「典型想法」：struct Foo { s: String, r: &str /* 借用自 s */ }
    // 这个在 Rust 里天然不成立：
    //   - Foo 自己 move 时，s 会跟着 move，但 r 指向的是「旧地址」
    //   - 如果 r 内部存的是字节指针，move 后会瞬间变成悬垂引用
    //
    // 你会遇到这种模式时，常见解决方案是：
    //   1. 用索引 / 偏移量代替引用：`struct Foo { s: String, start: usize, len: usize }`
    //   2. 用 Rc<String> / Arc<String> + String 索引
    //   3. 用第三方 crate 的 Pin / self_cell / ouroboros 等高级方案
    //
    // 初学阶段的经验：
    //   · 不要尝试在结构体里写「借用自己字段」的引用
    //   · 优先用索引或包装 Rc<T> 来表达自引用关系
    //   · 真正需要「指向自己」的场景极少，出现时先问问自己：能用索引吗？
    println!("  自引用结构体（struct Foo {{ s: String, r: &str }}）在 Rust 里几乎不可能直接写");
    println!("  绕过方式：");
    println!("    · 用索引/偏移量代替 &str（start..end）");
    println!("    · 用 Rc<String> / Arc<String> 共享所有权");
    println!("    · 进阶方案：Pin、self_cell / ouroboros 等 crate");
    println!("  日常练习阶段建议直接避免这种模式");
    println!("小结：Rust 里「自引用」是高阶话题，基础阶段通常不需要涉及");

    // ─────────────────────────────────────────
    println!("\n【总结】结构体所有权要点");
    // ─────────────────────────────────────────
    println!("  · 结构体「拥有」所有字段：实例 drop 时自动释放所有资源（RAII）");
    println!("  · 结构体默认是 move 语义，与 String / Vec 等一致");
    println!("  · 部分 move：单独 move 一个非 Copy 字段后，整体不可用但其他字段可单独访问");
    println!("  · 字段选型：");
    println!("    - 默认用 String / Vec<T>（拥有型，实例独立）");
    println!("    - 零拷贝场景才用 &str / &[T]（借用型，需要生命周期注解）");
    println!("  · 分字段借用：不同字段可同时 &mut，Rust 支持精确的字段级借用分析");
    println!("  · Drop 顺序：按字段声明的「逆序」drop，这点在资源释放场景很重要");
    println!("  · 自引用：初学阶段尽量避免；必须要时用索引 / Rc<String> / 高级 crate");
}
