#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 结构体中的生命周期（Lifetimes in Structs）
//
// 当结构体的字段是「借用」（&T）而不是「拥有」（T）时，Rust 强制你用
// 生命周期参数 'a 描述这个借用能活多久。这是 Rust 最独特、也最让初学者
// 困惑的语法元素之一。
//
// 本示例要把这个话题彻底讲透：
//
//   1. 为什么拥有 String 不需要 'a，而借用 &str 就需要？
//   2. struct ImportantExcerpt<'a> { part: &'a str } 的经典例子
//   3. impl<'a> ImportantExcerpt<'a>：impl 块的生命周期写法
//   4. 生命周期省略规则（lifetime elision）：什么时候可以省略不写
//   5. 多个独立生命周期参数：struct Holder<'a, 'b>
//   6. &'static 字段：常量 / 静态字符串
//   7. 结构体生命周期的约束：实例不能比借用源活得更长
//   8. 何时该用「借用字段」、何时该用「拥有字段」
//
// 核心直觉：
//   · 生命周期参数不是「给数据定义活多久」，而是「描述借用关系」
//   · 'a 是一个「占位符」，告诉编译器：这个引用至少活 'a 这么长
//   · 编译器负责验证你的使用方式不会让引用悬垂
//
// 看懂生命周期参数，需要先接受一个事实：
//   它不改变运行时行为，只是告诉编译器「这些借用之间的生存关系」，
//   编译器拿着这些关系去验证你的代码是否安全。
// ─────────────────────────────────────────────────────────────────────────────

// ── 1. 没有借用字段的结构体：不需要 'a ──────────────────────────────────────
// String 是「拥有型」字段，User 实例拥有这段字符串的所有权
// 所以 User 本身的生命周期 = 它被创建到被 drop 之间的时间
// 不依赖任何外部数据 → 不需要 'a
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// ── 2. 有借用字段的结构体：必须写 'a ────────────────────────────────────────
// UserRef 不拥有字符串，只是「看着」外部的 &str
// 必须引入生命周期参数 'a，告诉编译器：
//   · name 和 email 都借用了某段数据
//   · 这段数据至少要活 'a 那么长
//   · UserRef 实例自己也不能比 'a 活得更久
#[derive(Debug)]
struct UserRef<'a> {
    id: u64,
    name: &'a str,
    email: &'a str,
}

// ── 3. impl 块也要带 'a ──────────────────────────────────────────────────────
// impl 后面的 <'a> 和 UserRef<'a> 里的 'a 必须一致
// 这个 'a 是「从这个 impl 块引入的类型参数」
impl<'a> UserRef<'a> {
    // 构造器：返回 Self（即 UserRef<'a>）
    fn new(id: u64, name: &'a str, email: &'a str) -> Self {
        UserRef { id, name, email }
    }

    // 方法：返回值是 &str，生命周期和 self 相同（编译器可以自动推断）
    // 完整写法是 fn name_of(&self) -> &'a str，但一般省略
    fn name_of(&self) -> &str {
        self.name
    }
}

// ── 4. 经典例子：ImportantExcerpt（《Rust Book》里的标志性例子）──────────────
// 从一篇长文章里「摘录」一段话，part 字段只是借用原文的一部分
// 这种 Ref 结构体在解析器、AST、视图层都非常常见
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 返回第一句话（从头到第一个句号为止）
    // 生命周期省略：输入 &self 是 &'a Self，输出 &str 自动继承 &'a
    fn first_sentence(&self) -> &str {
        let s = self.part;
        match s.find('.') {
            Some(i) => &s[..=i],
            None => s,
        }
    }

    // 接收额外的 &str 参数：
    // 多个入参时，生命周期省略不再自动生效，要显式写清楚返回值绑定哪个
    // 这里显式说明返回值的生命周期和 self.part 一致（'a），不是 announcement 的
    fn announce_and_return_part(&self, announcement: &str) -> &'a str {
        println!("  🔔 {}", announcement);
        self.part
    }
}

// ── 5. 多个独立生命周期：Holder<'a, 'b> ─────────────────────────────────────
// 两个借用字段来自不同源、生命周期不相关时，需要分开声明
// 'a 和 'b 是两个独立的占位符
//
// 这比都用 'a 更灵活：调用方可以传入两个不同生命周期的 &str
#[derive(Debug)]
struct Holder<'a, 'b> {
    source: &'a str,      // 第一个借用源
    note: &'b str,        // 另一个借用源（可以来自完全不同的作用域）
}

impl<'a, 'b> Holder<'a, 'b> {
    fn new(source: &'a str, note: &'b str) -> Self {
        Holder { source, note }
    }

    // 返回 &'a str：绑定 source 的生命周期
    fn get_source(&self) -> &'a str {
        self.source
    }

    // 返回 &'b str：绑定 note 的生命周期
    fn get_note(&self) -> &'b str {
        self.note
    }
}

// ── 6. &'static 生命周期：永久有效的借用 ────────────────────────────────────
// 'static 是最长的生命周期，覆盖程序的整个运行期
// 最常见来源是字符串字面量：它们被编译进二进制，程序运行期间一直存在
// 所以 &'static str 是完全合法的字段类型，且不需要 'a 参数
#[derive(Debug)]
struct LicenseHeader {
    product: &'static str,
    license: &'static str,
}

impl LicenseHeader {
    fn new(product: &'static str, license: &'static str) -> Self {
        LicenseHeader { product, license }
    }
}

fn main() {
    println!("{}", "=== 结构体中的生命周期 ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、对比：拥有 String 的结构体 vs 借用 &str 的结构体");
    // ─────────────────────────────────────────

    // User 拥有字符串：实例独立存在，不依赖任何外部数据
    let u1 = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    println!("  User (拥有 String)  = {:?}", u1);

    // UserRef 只借用字符串：需要外部 name/email 先存在
    let name = String::from("Bob");
    let email = "bob@example.com";                  // 字面量：&'static str
    let u2 = UserRef::new(2, &name, email);
    println!("  UserRef (借用 &str) = {:?}", u2);

    println!("  拥有型 User：不需要 'a，实例可独立 move/存活");
    println!("  借用型 UserRef<'a>：'a 描述字符串的借用源必须活得比实例长");
    println!("小结：字段是否需要生命周期参数，取决于「拥有」还是「借用」");

    // ─────────────────────────────────────────
    println!("\n2、结构体方法：生命周期省略规则");
    // ─────────────────────────────────────────

    // UserRef::name_of(&self) -> &str 里没写 'a
    // 因为 Rust 的省略规则：
    //   规则 1：每个 &-输入都有自己的生命周期（这里 &self 有 'a）
    //   规则 2：只有一个输入生命周期时，它被赋给所有输出
    //   规则 3：若有 &self / &mut self，输出默认绑定 self 的生命周期
    // 综合起来，name_of 的真实签名是 fn name_of(&'a self) -> &'a str
    let n = u2.name_of();
    println!("  u2.name_of() = {}", n);

    println!("  省略规则让简单情况下不必写 'a，代码更干净");
    println!("  规则核心：单 &self 时输出默认跟 self 同生命周期");
    println!("小结：大部分方法写法里可以省略 'a，编译器按规则自动补齐");

    // ─────────────────────────────────────────
    println!("\n3、ImportantExcerpt：经典例子");
    // ─────────────────────────────────────────

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    // ImportantExcerpt 借用了 novel 的一部分
    // 编译器验证：excerpt 不能比 novel 活得更长
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("  excerpt = {:?}", excerpt);
    println!("  first_sentence() = {:?}", excerpt.first_sentence());

    // 多参数方法：生命周期省略会失效
    let announcement = "新书发布！";
    let got = excerpt.announce_and_return_part(announcement);
    println!("  announce_and_return_part 返回 = {:?}", got);

    println!("  返回值显式写了 &'a str，明确绑定到 self.part 的生命周期");
    println!("  这样即使 announcement 是短命的参数，编译器也知道返回值的来源是 self");
    println!("小结：多输入参数时，通过 'a 显式指明「返回值跟谁同命」");

    // ─────────────────────────────────────────
    println!("\n4、多个独立生命周期：Holder<'a, 'b>");
    // ─────────────────────────────────────────

    // source 来自外层作用域
    let long_source = String::from("这是一段活得很久的原始字符串");

    // note 只活在一个小块里
    let note_result;
    {
        let short_note = String::from("note-inside-inner-block");
        let h = Holder::new(&long_source, &short_note);

        // 只要 h 没被带出这个块，两个借用都还有效
        println!("  holder.source = {:?}", h.get_source());
        println!("  holder.note   = {:?}", h.get_note());

        // 把来自 source（'a）的部分「抽出来」是 OK 的
        // 因为 long_source 活得比当前块长
        note_result = h.get_source();
    }
    // ❌ 不能把 h 的 note 部分带出来（note 活到块末就 drop）
    //   let leaked = h.get_note();   // 编译器会报 "borrowed value does not live long enough"

    println!("  从 holder 取出 source（来自外层）后，块外仍可用：");
    println!("    note_result = {:?}", note_result);

    println!("  Holder<'a, 'b> 让两个借用字段各自有独立生命周期");
    println!("  若都用 'a，就变成「两个字段生命周期被合并成最短那个」了");
    println!("小结：多个借用字段生命周期不相关时，分成 <'a, 'b> 更灵活");

    // ─────────────────────────────────────────
    println!("\n5、&'static 字段：永远有效的借用");
    // ─────────────────────────────────────────

    // 字符串字面量是 &'static str，编译到二进制里，整个程序生命周期都可用
    let header = LicenseHeader::new("rust-journey", "MIT");

    println!("  header = {:?}", header);
    println!("  产品名: {}", header.product);
    println!("  许可证: {}", header.license);

    println!("  &'static 是最长的生命周期，字面量自然满足");
    println!("  因为不涉及任何借用依赖，LicenseHeader 可以不带 <'a>");
    println!("小结：&'static 字段适合常量字符串 / 全局配置，不需要引入生命周期参数");

    // ─────────────────────────────────────────
    println!("\n6、生命周期错误案例：实例比借用源活得长");
    // ─────────────────────────────────────────

    // 下面的代码会编译错误，我们用注释形式讲解
    //
    // fn make_excerpt_bad() -> ImportantExcerpt<'static> {
    //     let s = String::from("temporary");       // s 在函数末尾就 drop
    //     ImportantExcerpt { part: &s }            // ❌ part 借用了 s，但 s 很快就消失
    // }
    //
    // 编译错误示意：
    //   error[E0515]: cannot return value referencing local variable `s`
    //   returns a value referencing data owned by the current function
    //
    // 原因：
    //   · s 在函数结束时就被 drop
    //   · 但返回的 ImportantExcerpt 里 part 还指着 s 的内存
    //   · 调用方拿到后使用 → 悬垂引用 → 未定义行为
    //   · 所以 Rust 编译期直接禁止这种写法
    //
    // 修正方案：
    //   · 让结构体拥有数据：struct ImportantExcerpt { part: String }（改成拥有型）
    //   · 或者：让参数传进来，把借用源在调用处创建
    println!("  典型错误：在函数内构造局部字符串，然后返回借用它的结构体");
    println!("  编译器会拒绝，因为返回后引用就悬垂了");
    println!("  解决：要么改成拥有型字段，要么让外部传入借用源");
    println!("小结：生命周期不是「能让短的变长」，而是「强制验证借用是否安全」");

    // ─────────────────────────────────────────
    println!("\n7、实战对比：什么时候用借用字段，什么时候用拥有字段");
    // ─────────────────────────────────────────

    // 借用型（UserRef<'a>）适合：
    //   · 函数内部临时处理：从一个大 buffer 抽出多个视图
    //   · 解析器 / Lexer：Token 引用原始输入字符串的片段
    //   · 不想复制数据，又需要结构化视图
    //
    // 拥有型（User）适合：
    //   · 跨函数 / 跨线程传递
    //   · 存到集合 / 缓存 / 数据库
    //   · 实例需要有独立生命周期（最常见）

    // 借用型示例：解析配置行 "key=value"
    fn parse_config_line(line: &str) -> Option<Holder<'_, '_>> {
        let eq = line.find('=')?;
        let key = &line[..eq];
        let value = &line[eq + 1..];
        Some(Holder::new(key, value))
    }

    let cfg = "host=localhost";
    if let Some(h) = parse_config_line(cfg) {
        println!("  解析配置: key={:?}, value={:?}", h.source, h.note);
    }

    // 注意返回类型里的 '_ 是「生命周期占位符」，Rust 根据输入自动推断
    //   fn parse_config_line(line: &str) -> Option<Holder<'_, '_>>
    // 等价于
    //   fn parse_config_line<'a>(line: &'a str) -> Option<Holder<'a, 'a>>

    println!("  借用型返回：零拷贝、高效，但调用方要管理好生命周期");
    println!("  拥有型返回：常用于跨边界数据，允许调用方自由持有");
    println!("小结：默认用拥有型；只有性能/架构需要时，才上借用型 + 生命周期");

    // ─────────────────────────────────────────
    println!("\n8、常见误区与注意事项");
    // ─────────────────────────────────────────

    println!("  误区 1：以为 'a 会「延长数据寿命」");
    println!("    事实：'a 只是「声明存在」某个生命周期，不能改变数据的实际寿命");
    println!("    编译器拿 'a 去验证你的使用方式，不会神奇地让短的变长");
    println!();
    println!("  误区 2：给所有借用字段都写 'a（合并了独立的生命周期）");
    println!("    正确：字段生命周期不相关时用 <'a, 'b>");
    println!("    单一 <'a> 会让编译器取两个借用的「最短交集」，限制更严");
    println!();
    println!("  误区 3：滥用 'static");
    println!("    &'static 是最强的约束，很多值没办法满足");
    println!("    请只在确实是「程序级永久数据」（字面量 / static）时使用");
    println!();
    println!("  误区 4：自引用结构体 struct Foo {{ s: String, r: &s }}");
    println!("    Rust 不允许，因为 Foo move 时 r 就变悬垂了");
    println!("    见 09_ownership_in_structs.rs：解决方案是索引、Rc<T>、self_cell 等");
    println!("小结：生命周期语法初学有门槛，但熟悉后你会发现它和「数据流方向」对得很齐");

    // ─────────────────────────────────────────
    println!("\n【总结】结构体生命周期要点");
    // ─────────────────────────────────────────
    println!("  · 借用字段 → 必须加 'a：struct Foo<'a> {{ x: &'a T }}");
    println!("  · 拥有字段 → 不需要 'a：struct Foo {{ x: T }}");
    println!("  · impl 块也要带 'a：impl<'a> Foo<'a> {{ ... }}");
    println!("  · 省略规则：&self 方法的输出默认跟 self 同生命周期，可以不写");
    println!("  · 多输入参数时，显式写 &'a T 指明返回值来源");
    println!("  · 多独立借用源 → 用多个参数 <'a, 'b>，避免合并成「最短寿命」");
    println!("  · &'static 字段：适合字面量、常量、全局资源");
    println!("  · 实例不能活得比借用源长，编译器会直接拒绝这种写法");
    println!();
    println!("  判断准则：");
    println!("    默认写拥有型字段（String / Vec<T>），让实例独立；");
    println!("    只在「性能敏感、明确短期持有」时才用借用型 + 生命周期。");
    println!();
    println!("  下一步：");
    println!("    生命周期是贯穿 Rust 全章的话题，接下来在「trait 与泛型」、");
    println!("    「迭代器」、「闭包」、「async 函数」里会反复出现。");
    println!("    把这一章的直觉吃透，后面就顺得多了。");
}
