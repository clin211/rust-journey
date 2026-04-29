//! 08. Option<T>：替代 null 的标准答案
//!
//! 运行：cargo run --example 08_option
//!
//! 本例覆盖：
//! - 为什么 Rust 没有 null
//! - Option<T> 的本质（一个枚举而已）
//! - 常用 API：is_some / unwrap / unwrap_or / unwrap_or_else / map / and_then / or / take / replace
//! - 链式调用
//! - `?` 早返回 + Option 之间的传播
//! - Option 的内存布局（NPO 预告）

#![allow(dead_code, unused_variables)]
// 注：本文件作为 Option API 的教学样例, 必须在已知是 Some / None 的字面值上
// 演示 unwrap() / expect() / unwrap_or() 等的行为, 因此 allow 相关 lint。
#![allow(clippy::unnecessary_literal_unwrap)]

// ============================================================================
// 1. 为什么 Rust 没有 null
// ============================================================================
//
// 在 C / Java / JS / Python 里，"null"（或 None / nil / undefined）是导致最多线上
// bug 的特性之一。Tony Hoare 自己都说："null 是我十亿美元级的错误"。
//
// 问题在于：
// - 任何引用类型都可能为 null
// - 编译器不区分"可能为 null"和"绝对不为 null"
// - 解引用 null 是运行时崩溃
//
// Rust 的解决方案：完全不要 null，用 enum 在类型层面表达"可能没有"：
//
//   pub enum Option<T> {
//       None,
//       Some(T),
//   }
//
// 这样：
// - 一个值的类型是 `T`     → 它一定有效
// - 一个值的类型是 `Option<T>` → 你必须先 match / if let / 用 Option 方法处理 None
//
// 编译器全程帮你审查"我有没有处理 None"，避免空指针异常。

// ============================================================================
// 2. 创建 Option
// ============================================================================

fn create_examples() {
    let a: Option<i32> = Some(42);
    let b: Option<i32> = None;

    // 注意：单独写 None 时编译器有时推断不出 T，要么显式标注要么用上下文
    let c: Option<&str> = None;

    // 把可能失败的表达式封进 Option：
    let d: Option<i32> = "42".parse().ok();      // parse() 返回 Result，.ok() 转成 Option
    let e: Option<i32> = "abc".parse().ok();

    println!("  a={a:?}, b={b:?}, c={c:?}");
    println!("  d={d:?}, e={e:?}");
}

// ============================================================================
// 3. 简单查询：is_some / is_none
// ============================================================================

fn predicates() {
    let x = Some(3);
    let y: Option<i32> = None;

    println!("  x.is_some() = {}", x.is_some());
    println!("  x.is_none() = {}", x.is_none());
    println!("  y.is_some() = {}", y.is_some());
    println!("  y.is_none() = {}", y.is_none());

    // is_some_and / is_none_or 是布尔守卫的现代写法
    println!("  Some(3).is_some_and(|n| n > 0) = {}", Some(3).is_some_and(|n| n > 0));
    println!("  Some(0).is_some_and(|n| n > 0) = {}", Some(0).is_some_and(|n| n > 0));
}

// ============================================================================
// 4. 取值：unwrap 系列
// ============================================================================
//
// 把 Option<T> 转换回 T 的方法，从最危险到最安全：
//
//   .unwrap()              → 是 None 直接 panic！测试 / demo 用，生产慎用
//   .expect("msg")         → 是 None 时 panic 并打印自定义信息
//   .unwrap_or(默认)       → 是 None 时返回默认值（默认必须先算好）
//   .unwrap_or_else(|| ..) → 是 None 时调用闭包计算默认值（懒求值）
//   .unwrap_or_default()   → 是 None 时用 T::default()

fn unwrap_family() {
    let some_v: Option<i32> = Some(7);
    let none_v: Option<i32> = None;

    // unwrap：开发阶段快速调用，生产代码避免
    println!("  Some(7).unwrap()             = {}", some_v.unwrap());

    // expect：调试时优于 unwrap，错误信息更友好
    let cfg: Option<&str> = Some("/etc/app.conf");
    println!("  cfg.expect(...)              = {}", cfg.expect("缺少配置文件"));

    // unwrap_or：固定默认值
    println!("  Some(7).unwrap_or(0)         = {}", some_v.unwrap_or(0));
    println!("  None.unwrap_or(0)            = {}", none_v.unwrap_or(0));

    // unwrap_or_else：懒求值，闭包只有 None 时才执行
    let lazy_default = || {
        println!("  [懒求值闭包被执行]");
        99
    };
    println!("  Some(7).unwrap_or_else(...)  = {}", some_v.unwrap_or_else(lazy_default));
    println!("  None.unwrap_or_else(...)     = {}", none_v.unwrap_or_else(lazy_default));

    // unwrap_or_default：用 T::default()
    let s: Option<String> = None;
    println!("  None.unwrap_or_default()     = {:?}", s.unwrap_or_default());
}

// ============================================================================
// 5. 变换：map / and_then / filter / or / xor
// ============================================================================
//
// 这一组是 Option 的"组合子"，让你像写 Excel 公式一样把变换串起来，
// 完全不用写一堆 match 分支。

fn combinators() {
    // map: Some(x) → Some(f(x)); None → None
    let x = Some(10);
    let doubled = x.map(|n| n * 2);
    println!("  Some(10).map(*2)            = {doubled:?}");

    let none: Option<i32> = None;
    println!("  None.map(*2)                = {:?}", none.map(|n| n * 2));

    // and_then (flat_map / bind)：闭包返回 Option，扁平化
    // 用来组合多个"可能失败"的步骤
    fn parse(s: &str) -> Option<i32> { s.parse().ok() }
    fn positive(n: i32) -> Option<i32> { if n > 0 { Some(n) } else { None } }
    // 注意：当函数签名直接匹配时，可以直接传函数名，不必包一层闭包
    let result = Some("5").and_then(parse).and_then(positive);
    println!("  '5' -> parse -> positive    = {result:?}");
    let result2 = Some("0").and_then(parse).and_then(positive);
    println!("  '0' -> parse -> positive    = {result2:?}");
    let result3 = Some("abc").and_then(parse).and_then(positive);
    println!("  'abc'-> parse -> positive   = {result3:?}");

    // filter: Some(x) 满足谓词才保留，否则变 None
    let big = Some(42).filter(|&n| n > 100);
    println!("  Some(42).filter(>100)       = {big:?}");

    // or / or_else：Some 优先，否则尝试备选
    let a: Option<i32> = None;
    let b: Option<i32> = Some(5);
    println!("  None.or(Some(5))            = {:?}", a.or(b));
    println!("  Some(5).or(None)            = {:?}", b.or(a));

    // xor：恰好一个 Some 时保留，两边都是或都不是 → None
    println!("  Some(1).xor(None)           = {:?}", Some(1).xor::<>(None));
    println!("  Some(1).xor(Some(2))        = {:?}", Some(1).xor(Some(2)));
}

// ============================================================================
// 6. 转换：as_ref / as_mut / cloned / copied
// ============================================================================
//
// 这一组用来在"持有 Option" 与 "持有 Option 的引用 / 内部值"之间转换。

fn conversions() {
    // as_ref: Option<T> → Option<&T>，避免 move
    let owned: Option<String> = Some("hello".to_string());
    let r: Option<&String> = owned.as_ref();             // owned 还能继续用
    let len_opt: Option<usize> = r.map(|s| s.len());
    println!("  as_ref + map.len = {len_opt:?} (owned仍可用: {owned:?})");

    // as_mut: Option<T> → Option<&mut T>
    let mut maybe_buf: Option<Vec<u8>> = Some(vec![1, 2, 3]);
    if let Some(v) = maybe_buf.as_mut() {
        v.push(4);
    }
    println!("  as_mut 后: {maybe_buf:?}");

    // cloned: Option<&T> → Option<T> (T: Clone)
    let owned_again: Option<String> = r.cloned();
    println!("  cloned: {owned_again:?}");

    // copied: Option<&T> → Option<T> (T: Copy)
    let n: i32 = 42;
    let r: Option<&i32> = Some(&n);
    let c: Option<i32> = r.copied();
    println!("  copied: {c:?}");
}

// ============================================================================
// 7. 与 Result 互转
// ============================================================================

fn to_result_demo() {
    let some_v: Option<i32> = Some(7);
    let none_v: Option<i32> = None;

    // ok_or: None → Err(给定错误)
    let r1: Result<i32, &str> = some_v.ok_or("缺值");
    let r2: Result<i32, &str> = none_v.ok_or("缺值");
    println!("  ok_or: {r1:?} / {r2:?}");

    // ok_or_else: 懒求值版本
    let r3: Result<i32, String> = none_v.ok_or_else(|| format!("[{}] 缺值", "key"));
    println!("  ok_or_else: {r3:?}");

    // 反向：Result → Option
    let res: Result<i32, &str> = Ok(8);
    let opt = res.ok();        // Result<T,E> → Option<T>，丢弃错误
    println!("  Result.ok(): {opt:?}");
}

// ============================================================================
// 8. 就地变换：take / replace / get_or_insert
// ============================================================================
//
// 这一组用来"安全地把 Option 的内部值取出来 / 替换掉"，
// 而不用先 match 再赋值。

fn in_place_ops() {
    // take(): 把内部值取出来，自身变成 None
    let mut a = Some(10);
    let v = a.take();
    println!("  take: 取出 {v:?}, 原变量变成 {a:?}");

    // replace(): 用新值替换，返回旧值
    let mut b = Some(1);
    let old = b.replace(2);
    println!("  replace: 旧={old:?}, 新={b:?}");

    // get_or_insert(): 如果是 None，先塞进去，再返回内部 &mut
    let mut c: Option<Vec<i32>> = None;
    {
        let v = c.get_or_insert_with(Vec::new);
        v.push(7);
        v.push(8);
    }
    println!("  get_or_insert_with: {c:?}");
}

// ============================================================================
// 9. ? 运算符（Option 版本）
// ============================================================================
//
// `?` 让你写"短路链式":
//   - 如果当前是 Some(x)，自动解出 x 继续往下
//   - 如果当前是 None，函数立刻返回 None
//
// 这是写"多步可能失败"逻辑时最干净的写法（注意：函数返回值类型必须也是 Option）。

#[derive(Debug)]
struct User { id: u64, profile: Option<Profile> }

#[derive(Debug)]
struct Profile { contact: Option<Contact> }

#[derive(Debug)]
struct Contact { email: Option<String> }

fn user_email(u: &User) -> Option<&String> {
    let profile = u.profile.as_ref()?;            // None ? → 直接 return None
    let contact = profile.contact.as_ref()?;
    let email = contact.email.as_ref()?;
    Some(email)
}

// 没有 ? 的等价写法（多层嵌套）
fn user_email_old(u: &User) -> Option<&String> {
    match u.profile.as_ref() {
        Some(p) => match p.contact.as_ref() {
            Some(c) => c.email.as_ref(),
            None => None,
        },
        None => None,
    }
}

// ============================================================================
// 10. 实战：一个安全的"配置取值器"
// ============================================================================

#[derive(Debug, Default)]
struct AppConfig {
    log_level: Option<String>,
    timeout_ms: Option<u64>,
    db_url: Option<String>,
}

impl AppConfig {
    fn log_level_or_default(&self) -> &str {
        self.log_level.as_deref().unwrap_or("info")
    }

    fn timeout_or_default(&self) -> u64 {
        self.timeout_ms.unwrap_or(5000)
    }

    fn db_url(&self) -> Result<&str, &'static str> {
        self.db_url
            .as_deref()
            .ok_or("数据库连接串未配置")
    }
}

fn main() {
    println!("===== 1. 创建 Option =====");
    create_examples();

    println!("\n===== 2. 简单查询 =====");
    predicates();

    println!("\n===== 3. unwrap 系列 =====");
    unwrap_family();

    println!("\n===== 4. 组合子（map/and_then/filter/or/xor）=====");
    combinators();

    println!("\n===== 5. 引用转换 =====");
    conversions();

    println!("\n===== 6. 与 Result 互转 =====");
    to_result_demo();

    println!("\n===== 7. take / replace / get_or_insert =====");
    in_place_ops();

    println!("\n===== 8. ? 运算符链 =====");
    let u_full = User {
        id: 1,
        profile: Some(Profile {
            contact: Some(Contact {
                email: Some("alice@x.com".into()),
            }),
        }),
    };
    let u_partial = User {
        id: 2,
        profile: Some(Profile { contact: None }),
    };
    let u_none = User { id: 3, profile: None };
    println!("  u_full   -> {:?}", user_email(&u_full));
    println!("  u_partial-> {:?}", user_email(&u_partial));
    println!("  u_none   -> {:?}", user_email(&u_none));

    println!("\n===== 9. AppConfig 实战 =====");
    let cfg = AppConfig {
        log_level: Some("debug".into()),
        timeout_ms: None,
        db_url: None,
    };
    println!("  log_level: {}", cfg.log_level_or_default());
    println!("  timeout_ms: {}", cfg.timeout_or_default());
    match cfg.db_url() {
        Ok(u) => println!("  db_url ok: {u}"),
        Err(e) => println!("  db_url err: {e}"),
    }

    println!("\n===== 10. 内存布局：NPO =====");
    use std::mem::size_of;
    println!("  size_of::<&i32>()         = {}", size_of::<&i32>());
    println!("  size_of::<Option<&i32>>() = {} (NPO 让两者一样大)", size_of::<Option<&i32>>());
    println!("  size_of::<Box<i32>>()     = {}", size_of::<Box<i32>>());
    println!("  size_of::<Option<Box<i32>>>() = {}", size_of::<Option<Box<i32>>>());

    println!("\n===== 要点回顾 =====");
    println!("· Rust 没有 null：用 Option<T> 在类型层面表达可空");
    println!("· 优先用组合子（map/and_then/unwrap_or_else）而非 match");
    println!("· ? 是 'None 早返回' 的语法糖，函数签名要也返回 Option");
    println!("· Option<&T>、Option<Box<T>> 等享受 NPO 优化，与裸指针等大");
}
