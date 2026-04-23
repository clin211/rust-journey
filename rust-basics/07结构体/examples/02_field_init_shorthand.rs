#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 字段初始化简写（Field Init Shorthand）
//
// 当「局部变量名」与「字段名」一致时，可以省略 `字段名: 字段名` 里重复的部分，
// 写成 `字段名,` 的简写形式。这是 Rust 一个很小但很顺手的语法糖。
//
//   没有简写：User { username: username, email: email, age: age, active: active }
//   使用简写：User { username, email, age, active }
//
// 它最常出现在：
//   · 工厂函数（build_user、Point::new 等）
//   · 从外部输入（函数参数、配置、API 响应）构造实例
//   · 解构某个元组或返回值后组装成结构体
//
// 注意：
//   · 简写不改变所有权语义：仍然会把变量 move 进结构体
//   · 字段顺序随意，由字段名匹配，不按位置匹配
//   · 只有「变量名」和「字段名」完全一致时才能简写
// ─────────────────────────────────────────────────────────────────────────────

struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// 一个清晰的几何点结构体
struct Point {
    x: f64,
    y: f64,
}

// ── 传统写法：字段名和变量名重复 ────────────────────────────────────────────
// 在简写出现之前，每个字段都得写两遍，对参数多的结构体特别啰嗦。
#[allow(clippy::needless_return)]
fn build_user_verbose(username: String, email: String) -> User {
    return User {
        username: username,  // 这里「username: username」冗余
        email: email,        // 这里「email: email」也冗余
        age: 0,
        active: true,
    };
}

// ── 简写版本：字段名省略重复 ────────────────────────────────────────────────
// 只要参数名和字段名一致，就可以省略 `name:` 重复部分。
fn build_user(username: String, email: String) -> User {
    User {
        username,            // 等价于 username: username
        email,               // 等价于 email: email
        age: 0,              // 和字段名不同的名字（比如字面值）仍然要写全
        active: true,
    }
}

// ── 简写与其它字段混用：完全没问题 ───────────────────────────────────────────
// 能简写就简写，不能简写就照常写。可以按任意顺序组合。
fn build_user_with_age(name_input: String, email: String, age: u32) -> User {
    User {
        username: name_input, // name_input 和 username 名字不同，必须写全
        email,                // 同名简写
        age,                  // 同名简写
        active: true,         // 字面量，写全
    }
}

// ── 简写用在 Point 构造器上 ──────────────────────────────────────────────────
fn make_point(x: f64, y: f64) -> Point {
    Point { x, y }           // 参数名 x/y 恰好与字段名一致，简写很自然
}

fn main() {
    println!("{}", "=== 字段初始化简写 ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、最基础：字段名 == 变量名 → 可以简写");
    // ─────────────────────────────────────────

    let username = String::from("alice");
    let email = String::from("alice@example.com");

    // 传统写法（不简写）：每个字段写两遍名字
    let _u1 = User {
        username: username.clone(),  // 注意：这里是为了演示，才 clone 一份
        email: email.clone(),
        age: 30,
        active: true,
    };

    // 简写写法：变量 username → 直接 username，
    // 编译器自动理解成 username: username
    let u2 = User {
        username,                    // 等价于 username: username
        email,                       // 等价于 email: email
        age: 30,
        active: true,
    };

    println!("  u2.username = {}", u2.username);
    println!("  u2.email    = {}", u2.email);

    // 注意：此时 username 和 email 已经被 move 进 u2 了，不能再用
    // println!("{username}"); // ❌ 已 move
    println!("  ⚠️ 简写同样会 move：原变量 username / email 已不可再用");
    println!("小结：简写只是少写一次名字，语义与 `name: name` 完全相同");

    // ─────────────────────────────────────────
    println!("\n2、典型场景：工厂函数（build_user）");
    // ─────────────────────────────────────────

    let user_ver = build_user_verbose(
        String::from("verbose"),
        String::from("verbose@example.com"),
    );
    let user_sho = build_user(
        String::from("short"),
        String::from("short@example.com"),
    );

    println!("  build_user_verbose 返回: username={}, age={}", user_ver.username, user_ver.age);
    println!("  build_user         返回: username={}, age={}", user_sho.username, user_sho.age);

    println!("  两个函数行为完全一致，只是写法更紧凑");
    println!("小结：简写最常用在工厂函数里，大幅减少样板代码");

    // ─────────────────────────────────────────
    println!("\n3、简写与正常写法可以混用");
    // ─────────────────────────────────────────

    let user_mix = build_user_with_age(
        String::from("carol"),       // 会赋给 username 字段（不同名）
        String::from("carol@x.com"), // 会赋给 email 字段（同名简写）
        22,                          // 会赋给 age 字段（同名简写）
    );
    println!("  混合构造 user_mix.username = {}", user_mix.username);
    println!("  混合构造 user_mix.age      = {}", user_mix.age);
    println!("  混合构造 user_mix.active   = {}", user_mix.active);

    println!("  简写仅针对「同名字段」，不同名的字段仍需要写 `name: expr`");
    println!("小结：可按字段灵活选择简写或普通写法，没有全有/全无限制");

    // ─────────────────────────────────────────
    println!("\n4、简写与字段顺序无关（字段名匹配）");
    // ─────────────────────────────────────────

    // 字段是按名字匹配的，顺序可以完全打乱
    let age = 40u32;
    let active = false;
    let username = String::from("fourth");
    let email = String::from("fourth@example.com");

    let u4 = User {
        // 顺序故意打乱：age 在最前，email 在后
        age,
        active,
        email,
        username,
    };

    println!("  u4.username = {}, u4.age = {}, u4.active = {}",
        u4.username, u4.age, u4.active);
    println!("  顺序怎么写都可以，最终字段的对应关系看名字");
    println!("小结：Rust 结构体字段是「命名」对齐，而不是「位置」对齐，顺序随意");

    // ─────────────────────────────────────────
    println!("\n5、Point::make_point —— 简写在几何点上最常见");
    // ─────────────────────────────────────────

    let p1 = make_point(3.0, 4.0);
    let p2 = Point { x: 1.0, y: 2.0 };       // 当然也可以直接写

    println!("  p1 = ({:.1}, {:.1})", p1.x, p1.y);
    println!("  p2 = ({:.1}, {:.1})", p2.x, p2.y);

    // 搭配临时变量：从参数计算得到 x/y 后再组装
    let dx = 10.0;
    let dy = 20.0;
    let p3 = Point { x: dx + 1.0, y: dy + 2.0 }; // 这里就不能简写了（不是直接变量）
    println!("  p3 = ({:.1}, {:.1})", p3.x, p3.y);
    println!("  只有「直接一个同名变量」时才能简写，表达式 / 字面量仍需 `field: expr`");
    println!("小结：简写是「最简情况」下的语法糖，复杂表达式请继续用完整语法");

    // ─────────────────────────────────────────
    println!("\n6、常见误区：不同名字就不能简写");
    // ─────────────────────────────────────────

    let user_name = String::from("dave");    // 变量叫 user_name
    // let u = User { username, ... };       // ❌ 字段叫 username，名字不一致

    // 正确做法：要么把变量也命名成 username，要么写完整 `field: expr`
    let u5 = User {
        username: user_name,                 // ✅ 明确写出映射关系
        email: String::from("dave@x.com"),
        age: 0,
        active: true,
    };

    println!("  修正后 u5.username = {}", u5.username);
    println!("  ⚠️ `field` 简写只在变量名 == 字段名时有效，名字不一致必须写全");
    println!("小结：简写是名字对齐才能用的语法糖，不要强行简写名字不同的变量");

    // ─────────────────────────────────────────
    println!("\n【总结】字段初始化简写");
    // ─────────────────────────────────────────
    println!("  · 语法：同名变量直接写一次即可，`field,` 等价于 `field: field`");
    println!("  · 适用：工厂函数、构造器、参数直接映射字段的场景");
    println!("  · 限制：变量名必须 == 字段名，表达式 / 字面量仍用完整语法");
    println!("  · 语义：与原语法完全等价，不改变所有权（仍然 move）");
    println!("  · 顺序：结构体字段按名字匹配，怎么写顺序都行");
}
