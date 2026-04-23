#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 结构体更新语法（Struct Update Syntax）
//
// 当你已经有一个结构体实例，只想「改几个字段、其他保持一致」时，
// 可以用 `..base` 语法一次性把剩余字段从 base 拷贝/move 过来。
//
//   语法：S { 新字段1: 值1, 新字段2: 值2, ..base }
//
// 有三个关键点必须理解清楚：
//
//   1. ..base 必须写在最后（且前面显式字段的所有字段顺序都允许）
//   2. ..base 的字段按「字段级」进行 move 或 copy：
//        · 字段是 Copy 类型（比如 u32）→ 按位复制
//        · 字段不是 Copy（比如 String / Vec）→ 所有权被 move
//   3. 如果 base 中「被 move 的字段」没有被完整覆盖，那 base 作为整体就不能继续用
//        · 只有当从 base 拿走的字段全部是 Copy 时，base 本身才仍然完整可用
//
// 这个语法在 Rust 里非常实用，但坑也集中在字段级 move 这一点，
// 下面会用多个场景把它彻底讲透。
// ─────────────────────────────────────────────────────────────────────────────

// ── 用户结构体：混合 Copy 字段和非 Copy 字段 ──────────────────────────────────
// 后面用它来演示「部分字段被 move、base 失效」的微妙之处
struct User {
    username: String,                        // 非 Copy：更新时会被 move
    email: String,                           // 非 Copy：更新时会被 move
    age: u32,                                // Copy：更新时按位复制
    active: bool,                            // Copy：更新时按位复制
}

// ── 配置结构体：全 Copy 字段，base 可以继续使用 ──────────────────────────────
#[derive(Debug, Clone, Copy)]
struct WindowConfig {
    width: u32,
    height: u32,
    fullscreen: bool,
    vsync: bool,
}

fn main() {
    println!("{}", "=== 结构体更新语法 ..base ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、基础用法：从 base 派生一个新实例");
    // ─────────────────────────────────────────

    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@old.com"),
        age: 30,
        active: true,
    };

    // 用 ..user1 表示：剩余字段全部从 user1 填充
    // 这里只显式修改 email，其他 3 个字段都继承自 user1
    let user2 = User {
        email: String::from("alice@new.com"),
        ..user1                              // 注意：..user1 必须写在最后
    };

    println!("  user2.username = {}", user2.username);  // "alice" 继承而来
    println!("  user2.email    = {}", user2.email);     // "alice@new.com"（新值）
    println!("  user2.age      = {}", user2.age);       // 30  继承而来
    println!("  user2.active   = {}", user2.active);    // true 继承而来

    // ⚠️ 这里 user1 已经发生了「字段级 move」：
    //   · username（String）被 move 到 user2
    //   · age / active 是 Copy，按位复制，原字段无影响
    //   · email 在 user2 里被显式替换了，没有 move user1.email
    //     → user1.email 理论上还「在原位」，但因为 user1 里 username 已经 move 出去，
    //       user1 整体就不能再当作一个完整的 User 来使用
    //
    // 所以下面这两种访问，规则不同：
    // println!("{}", user1.username);       // ❌ 已 move
    println!("  ⚠️ user1.username 已经被 move 到 user2，不能再访问");

    // 但 user1.email 由于没有被 move，仍然可以单独访问
    println!("  ✅ user1.email 仍可访问: {}", user1.email);
    //         原理：部分 move（partial move）—— 某些字段被 move，
    //         但剩下那些没被 move、没被用到的字段，其本身仍可用

    println!("小结：..base 按字段逐个 move/copy，整个 base 实例的可用性要看每个字段");

    // ─────────────────────────────────────────
    println!("\n2、字段覆盖顺序随意，..base 永远写最后");
    // ─────────────────────────────────────────

    let template = User {
        username: String::from("template"),
        email: String::from("template@x.com"),
        age: 0,
        active: false,
    };

    // 字段顺序可以随便排，只要 ..template 是最后
    let user3 = User {
        age: 18,                             // 先写 age
        active: true,                        // 再写 active
        // username 和 email 都用 ..template 补充
        ..template
    };

    println!("  user3.username = {}", user3.username); // "template"（来自 base）
    println!("  user3.email    = {}", user3.email);    // "template@x.com"（来自 base）
    println!("  user3.age      = {}", user3.age);      // 18（显式覆盖）
    println!("  user3.active   = {}", user3.active);   // true（显式覆盖）

    // ⚠️ 同样，template.username 和 template.email 被 move 进了 user3
    // 此时 template 作为整体已不可用
    // println!("{}", template.username); // ❌

    println!("  显式字段顺序随意，..base 必须出现在 {{ }} 中最后一个位置");
    println!("小结：..base 的位置固定（最后），但前面显式字段的顺序完全自由");

    // ─────────────────────────────────────────
    println!("\n3、全字段都是 Copy → base 仍然可以继续使用");
    // ─────────────────────────────────────────

    let default_win = WindowConfig {
        width: 1280,
        height: 720,
        fullscreen: false,
        vsync: true,
    };

    // 从默认配置派生一个 4K 全屏版本
    let fourk = WindowConfig {
        width: 3840,
        height: 2160,
        fullscreen: true,
        ..default_win                        // 这里 vsync 会按位复制过来
    };

    // ✅ default_win 仍然完整可用！因为 WindowConfig 所有字段都是 Copy
    println!("  默认配置 default_win = {:?}", default_win);
    println!("  4K 版本  fourk       = {:?}", fourk);

    println!("  关键：WindowConfig 所有字段都是 Copy → ..base 是按位复制，base 不失效");
    println!("小结：结构体若整体 Copy（所有字段 Copy），..base 可以反复使用 base");

    // ─────────────────────────────────────────
    println!("\n4、只覆盖 Copy 字段 → base 中非 Copy 字段仍可单独访问");
    // ─────────────────────────────────────────

    let base = User {
        username: String::from("partial"),
        email: String::from("partial@x.com"),
        age: 0,
        active: false,
    };

    // 只显式修改 age 和 active，这两个都是 Copy 字段
    // ..base 里只有 username / email 会被 move 走
    let user4 = User {
        age: 99,
        active: true,
        ..base                               // username 和 email 会被 move 到 user4
    };

    println!("  user4.username = {}", user4.username); // "partial"
    println!("  user4.age      = {}", user4.age);      // 99
    // println!("{}", base.username);  // ❌ username 已 move
    // println!("{}", base.email);     // ❌ email    已 move
    // base.age / base.active 是 Copy，仍可访问，但 base 作为整体不可用
    println!("  ⚠️ base.username / base.email 已被 move，不能再访问");
    println!("  ⚠️ 但 base.age / base.active 仍可单独访问（Copy 字段）");
    println!("  base.age = {}, base.active = {}", base.age, base.active);

    println!("小结：部分 move 后，只有「未被 move 且本身 Copy 的字段」还能访问");

    // ─────────────────────────────────────────
    println!("\n5、显式覆盖所有非 Copy 字段 → base 仍然完整可用");
    // ─────────────────────────────────────────

    let src = User {
        username: String::from("src"),
        email: String::from("src@x.com"),
        age: 10,
        active: true,
    };

    // 显式覆盖了所有 String 字段（username / email），..src 只会按位复制剩下的 Copy 字段
    let user5 = User {
        username: String::from("copied_out"),
        email: String::from("copied_out@x.com"),
        ..src                                // 只 copy age / active
    };

    println!("  user5.username = {}", user5.username);
    println!("  user5.age      = {}", user5.age); // 来自 src

    // ✅ src 仍然完全可用：String 字段没有被 move 走
    println!("  src.username   = {}", src.username);
    println!("  src.email      = {}", src.email);
    println!("  src.age        = {}", src.age);

    println!("  技巧：如果希望 base 继续可用，就把所有非 Copy 字段显式覆盖掉");
    println!("小结：部分 move 问题只要「显式覆盖掉非 Copy 字段」就可以规避");

    // ─────────────────────────────────────────
    println!("\n6、与 Default 配合使用（初探）");
    // ─────────────────────────────────────────

    // 这里只用一个简短的演示，Default / derive 在 08_debug_and_derives 会细讲
    #[derive(Default, Debug)]
    struct Config {
        host: String,
        port: u16,
        tls: bool,
    }

    // 从 Default::default() 派生，只改 host 和 port
    let cfg = Config {
        host: String::from("127.0.0.1"),
        port: 8080,
        ..Default::default()                 // tls 来自 Default::default()，bool 默认 false
    };
    println!("  cfg = {:?}", cfg);

    println!("  ..Default::default() 是实践中最常见的用法，");
    println!("  用来在构造时只显式写「你关心的」几个字段，其它留默认值。");
    println!("小结：..Default::default() + 命名字段，是 Rust 里最接近「命名参数」的写法");

    // ─────────────────────────────────────────
    println!("\n7、常见错误与规避");
    // ─────────────────────────────────────────

    // ❌ 错误 1：..base 不在最后
    // let err = User {
    //     ..some_base,
    //     username: String::from("wrong"),   // 编译错误：..base 必须在最后
    // };
    println!("  ❌ `..base` 必须写在结构体字面量最后，不能写中间");

    // ❌ 错误 2：继续使用已经被部分 move 的 base
    let some_base = User {
        username: String::from("x"),
        email: String::from("y@x.com"),
        age: 1,
        active: true,
    };
    let _derived = User {
        age: 2,
        active: false,
        ..some_base                          // username / email 已被 move
    };
    // println!("{:?}", some_base.username);  // ❌
    println!("  ❌ some_base 在 username/email 被 move 后，作为整体不可再用");

    println!("  排查思路：遇到「..base 后不能用 base」时，");
    println!("  检查字段是不是 Copy，要么全部显式覆盖非 Copy 字段，要么主动 clone()");
    println!("小结：..base 的坑几乎都来自「字段级 move」，理解这点就不容易踩");

    // ─────────────────────────────────────────
    println!("\n【总结】结构体更新语法要点");
    // ─────────────────────────────────────────
    println!("  · 语法：S {{ 显式字段..., ..base }}，..base 永远放最后");
    println!("  · 语义：按字段级 move / copy，而不是「整体复制」");
    println!("  · 何时 base 仍可用：");
    println!("      - 结构体整体 Copy，或");
    println!("      - 显式覆盖掉所有非 Copy 字段");
    println!("  · 结合 Default：..Default::default() 可以让构造函数看起来像命名参数");
    println!("  · 踩坑点  ：部分 move 后 base 整体不可用，但 Copy 字段仍可单独读");
}
