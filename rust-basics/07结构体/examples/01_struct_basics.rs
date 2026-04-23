#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 结构体基础（Struct Basics）
//
// 结构体（struct）是 Rust 里最基础的自定义数据类型。
// 它可以把一组相关的字段"打包"成一个有名字的类型：
//
//   · 所有字段组成一个整体，有统一的生命周期
//   · 每个字段有自己的名字和类型，访问精确
//   · 字段默认私有，需要 pub 明确导出
//
// 三件重要的事情：
//   1. 如何定义一个结构体
//   2. 如何创建它的实例
//   3. 如何读取 / 修改字段
//
// 一个关键直觉：
//   结构体的「可变性」是作用在整个实例上的，不是字段级别的。
//   要修改某个字段，必须整个变量都是 mut。
//
// 与其它语言的对比（初学者常见误区）：
//   · JavaScript 对象：字段动态随意增减 → Rust 结构体字段在编译期就固定
//   · Python 类：属性可随意加 → Rust 结构体在 struct 定义里一次写完
//   · C 结构体：无方法 → Rust 结构体可以通过 impl 块挂方法
// ─────────────────────────────────────────────────────────────────────────────

// ── 1. 定义一个最基础的结构体 ─────────────────────────────────────────────────
// 结构体定义通常放在 main 之外（可以放任意位置，只要在使用前定义即可）。
// 字段的类型必须在编译期明确。
struct User {
    username: String,   // 拥有型字段：String 由该结构体实例拥有
    email: String,      // 同上，每个实例拥有自己的 email 字符串
    age: u32,           // Copy 类型，按位拷贝即可，开销极小
    active: bool,       // Copy 类型
}

// ── 2. 稍微复杂一点：嵌套结构体 ───────────────────────────────────────────────
// 结构体的字段可以是其他结构体，形成"组合"关系。
struct Address {
    city: String,
    street: String,
}

struct Employee {
    name: String,
    address: Address,   // 嵌套：Employee 拥有一个 Address
    salary: f64,
}

// ── 3. 一个只使用基本类型字段的结构体（后面演示 Copy/按位语义时用得上）────────
struct Point2D {
    x: f64,
    y: f64,
}

fn main() {
    println!("{}", "=== 结构体基础（Struct Basics） ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、定义结构体 vs 创建实例：两件不同的事");
    // ─────────────────────────────────────────

    // struct User { ... }  →  只是"定义"了一个类型（像图纸/模板）
    // let user1 = User { ... };  →  才是"创建"一个实例（按图纸造出来的实际物品）

    // 创建实例时，所有字段必须一起提供，不能漏掉任何一个
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };

    println!("  user1.username = {}", user1.username);
    println!("  user1.email    = {}", user1.email);
    println!("  user1.age      = {}", user1.age);
    println!("  user1.active   = {}", user1.active);
    println!("小结：struct 是定义，let x = S {{ ... }} 才是真正创建实例");

    // ─────────────────────────────────────────
    println!("\n2、字段访问：用 . 运算符");
    // ─────────────────────────────────────────

    // 字段访问没有什么魔法，就是 实例.字段名
    let len = user1.username.len();          // 访问字段，再调用字段自己的方法
    println!("  用户名 \"{}\" 的长度 = {}", user1.username, len);

    // 可以把字段拿出来单独使用（只读借用）
    let email_ref: &String = &user1.email;   // 借用字段，不转移所有权
    println!("  email 的引用: {email_ref}");
    println!("  user1.email 仍然可用: {}", user1.email); // 原字段依然完整

    // 也可以直接对字段"复制"值（当字段是 Copy 类型时）
    let age_copy: u32 = user1.age;           // u32 是 Copy，直接按位复制
    println!("  age 的副本 = {age_copy}，不影响原字段 user1.age = {}", user1.age);

    println!("小结：字段访问是 . 运算符，读写规则都遵守所有权和借用规则");

    // ─────────────────────────────────────────
    println!("\n3、可变性是「整体」而不是「字段级」");
    // ─────────────────────────────────────────

    // 如果实例不是 mut，任何字段都不能写
    let user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        age: 25,
        active: false,
    };
    // user2.age = 26;  // ❌ 编译错误：cannot assign to `user2.age`, as `user2` is not declared as mutable
    println!("  不可变实例：user2.age = {}（无法修改）", user2.age);

    // 必须整个实例用 mut，才能修改任何一个字段
    let mut user3 = User {
        username: String::from("carol"),
        email: String::from("carol@example.com"),
        age: 28,
        active: true,
    };

    user3.age += 1;                          // ✅ 可以修改
    user3.email = String::from("carol@new.com"); // ✅ 也可以整字段替换

    println!("  修改后 user3.age   = {}", user3.age);
    println!("  修改后 user3.email = {}", user3.email);

    // ⚠️ 注意：Rust 目前没有「单独字段 mut」这种语法
    // 不能写：struct User { mut age: u32, ... }
    // 可变性总是应用到整个实例上
    println!("  ⚠️ Rust 没有字段级 mut，要改字段，整个实例都必须是 mut");
    println!("小结：要修改字段，整个实例需声明为 let mut；不存在字段级 mut");

    // ─────────────────────────────────────────
    println!("\n4、嵌套结构体：组合优于继承");
    // ─────────────────────────────────────────

    // 把"地址"单独抽成 Address，让 Employee 组合它
    // 这比把 city/street 直接铺平在 Employee 里更清晰，也更容易复用
    let emp = Employee {
        name: String::from("David"),
        address: Address {
            city: String::from("Shanghai"),
            street: String::from("Nanjing Road 100"),
        },
        salary: 20000.0,
    };

    // 嵌套字段可以用链式 . 访问
    println!("  emp.name             = {}", emp.name);
    println!("  emp.address.city     = {}", emp.address.city);
    println!("  emp.address.street   = {}", emp.address.street);
    println!("  emp.salary           = {}", emp.salary);

    println!("  Rust 没有继承，通常用「组合」来表达父子/整体-部分关系");
    println!("小结：结构体嵌套是 Rust 组织复杂数据的主要方式，字段用 . 层层访问");

    // ─────────────────────────────────────────
    println!("\n5、结构体实例作为函数参数");
    // ─────────────────────────────────────────

    // 传值：会把所有权 move 进函数
    fn describe(u: User) {
        println!("  describe: user.username = {}", u.username);
    } // u 在这里离开作用域，整个 User 及其内部的 String 一起 drop

    // 传引用：只借用，不转移所有权
    fn describe_ref(u: &User) {
        println!("  describe_ref: user.username = {}", u.username);
    }

    // 传可变引用：可以修改字段
    fn celebrate_birthday(u: &mut User) {
        u.age += 1;
    }

    let mut user4 = User {
        username: String::from("evan"),
        email: String::from("evan@example.com"),
        age: 20,
        active: true,
    };

    describe_ref(&user4);                    // 借用：user4 还在
    celebrate_birthday(&mut user4);          // 可变借用：修改 age
    describe_ref(&user4);                    // 借用：看看是否变了

    println!("  生日后 user4.age = {}", user4.age);

    // 最后把 user4 move 进 describe（演示传值）
    describe(user4);
    // println!("{}", user4.username); // ❌ user4 已被 move，无法使用
    println!("  ❌ 传值到 describe 之后，user4 已经 move，不能再访问");

    println!("小结：User 实例跟 String 一样是「拥有资源的类型」，move/borrow 规则完全一致");

    // ─────────────────────────────────────────
    println!("\n6、几何实体：Point2D 展示基本类型字段");
    // ─────────────────────────────────────────

    // 全部字段都是 Copy 类型（f64），整个结构体既可以派生 Copy，也可以按值到处传
    // 这里先不派生 Copy，留到 08_debug_and_derives.rs 展开讲
    let p = Point2D { x: 3.0, y: 4.0 };

    // 欧几里得距离：距原点的长度
    let distance = (p.x * p.x + p.y * p.y).sqrt();
    println!("  p = ({:.1}, {:.1})，到原点距离 = {:.3}", p.x, p.y, distance);

    // 字段是 Copy 类型时，把字段值"复制"出来很便宜，可以随意使用
    let x_copy = p.x;
    let y_copy = p.y;
    println!("  x_copy = {x_copy}, y_copy = {y_copy}");
    println!("  p 原始值仍然可用：({:.1}, {:.1})", p.x, p.y);

    println!("小结：基本类型字段是 Copy，访问和复制开销极低，非常适合做值对象");

    // ─────────────────────────────────────────
    println!("\n【总结】结构体基础要点");
    // ─────────────────────────────────────────
    println!("  · 定义 vs 实例：struct 是类型声明，let x = S {{ ... }} 才是真实例");
    println!("  · 字段访问 ：用 . 运算符，遵循所有权 / 借用规则");
    println!("  · 可变性  ：作用在实例整体，没有字段级 mut");
    println!("  · 嵌套组合：Rust 没有继承，推荐组合字段来表达整体-部分关系");
    println!("  · 传参语义：传值 move、&借用、&mut 可变借用，全部按原规则");
    println!("  · 全部字段必写：创建实例时不能漏字段（后面介绍 ..base 可补齐）");
}
