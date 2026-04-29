//! 11. 递归枚举：链表 / JSON Value / 表达式树
//!
//! 运行：cargo run --example 11_recursive_enum
//!
//! 本例覆盖：
//! - 为什么"自包含的 enum"需要 Box / Rc 等间接层
//! - 用 enum + Box 写一个最小链表
//! - 用 enum + HashMap + Vec 写一个 JSON Value
//! - 用 enum 表达表达式树（AST），并写出递归求值

#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// 1. 为什么需要 Box
// ============================================================================
//
// 直觉上，你可能会写：
//
//   enum List {
//       Cons(i32, List),       // ❌ 编译报错
//       Nil,
//   }
//
// 编译器拒绝它的理由是：List 直接包含 List，导致大小无限大。
//
//   List 大小 = max(Cons 的大小, Nil 的大小)
//   Cons 大小 = i32 + List 大小
//                       ▲
//                       │ 又依赖 List 大小……
//                       │
//                  无限递归 → 编译器无法决定 size
//
// 只要在递归位置加一层"间接"（指针），就能打破循环：
//
//   enum List {
//       Cons(i32, Box<List>),  // ✅ Box 是固定大小（指针），递归被打破
//       Nil,
//   }
//
//   Box 在堆上分配 List，自身在栈上只占一个指针（8 字节，64 位平台）。

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    /// 长度
    fn len(&self) -> usize {
        match self {
            List::Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    /// 求和
    fn sum(&self) -> i32 {
        match self {
            List::Cons(head, tail) => head + tail.sum(),
            List::Nil => 0,
        }
    }

    /// 转成 Vec 方便打印
    fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut cur = self;
        while let List::Cons(head, tail) = cur {
            out.push(*head);
            cur = tail;
        }
        out
    }

    /// 用一组数构造一个 List
    fn from_slice(items: &[i32]) -> Self {
        let mut node = List::Nil;
        for &x in items.iter().rev() {
            node = List::Cons(x, Box::new(node));
        }
        node
    }
}

// ============================================================================
// 2. JSON Value：一个真实的递归 enum
// ============================================================================
//
// serde_json::Value 的简化版本——一个 JSON 值要么是基本类型，要么是
// "数组（含嵌套 Value）" / "对象（键到 Value 的映射）"，所以是递归的。

#[derive(Debug, Clone)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// 简单的 "Display" 风格输出，演示递归处理
    fn to_pretty(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{s}\""),
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    return "[]".into();
                }
                let inner: Vec<String> = arr
                    .iter()
                    .map(|v| format!("{}  {}", pad, v.to_pretty(indent + 1)))
                    .collect();
                format!("[\n{}\n{pad}]", inner.join(",\n"))
            }
            JsonValue::Object(map) => {
                if map.is_empty() {
                    return "{}".into();
                }
                // 排序 key 让输出可重复
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                let inner: Vec<String> = keys
                    .into_iter()
                    .map(|k| {
                        let v = &map[k];
                        format!("{}  \"{}\": {}", pad, k, v.to_pretty(indent + 1))
                    })
                    .collect();
                format!("{{\n{}\n{pad}}}", inner.join(",\n"))
            }
        }
    }

    /// 按 path 取值：obj["a"]["b"][2] 这种链式访问
    /// path 元素是 (key) 或 (index) —— 这里简化为字符串 path
    fn get(&self, key: &str) -> Option<&JsonValue> {
        if let JsonValue::Object(map) = self {
            map.get(key)
        } else {
            None
        }
    }
}

// ============================================================================
// 3. 表达式树（AST）+ 递归求值
// ============================================================================
//
// 这是 enum 的另一个杀手级用法：把"语法结构"用一个递归 enum 描述出来。
// 语言、模板、查询、表达式——一切结构化的东西都可以这样建模。

#[derive(Debug, Clone)]
enum Expr {
    Num(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Var(name) => write!(f, "{name}"),
            Expr::Add(a, b) => write!(f, "({a} + {b})"),
            Expr::Sub(a, b) => write!(f, "({a} - {b})"),
            Expr::Mul(a, b) => write!(f, "({a} * {b})"),
            Expr::Div(a, b) => write!(f, "({a} / {b})"),
            Expr::Neg(a) => write!(f, "(-{a})"),
        }
    }
}

#[derive(Debug)]
enum EvalError {
    UnboundVariable(String),
    DivByZero,
}

impl Expr {
    fn eval(&self, env: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        match self {
            Expr::Num(n) => Ok(*n),
            Expr::Var(name) => env
                .get(name.as_str())
                .copied()
                .ok_or(EvalError::UnboundVariable(name.clone())),
            Expr::Add(a, b) => Ok(a.eval(env)? + b.eval(env)?),
            Expr::Sub(a, b) => Ok(a.eval(env)? - b.eval(env)?),
            Expr::Mul(a, b) => Ok(a.eval(env)? * b.eval(env)?),
            Expr::Div(a, b) => {
                let bv = b.eval(env)?;
                if bv == 0.0 { return Err(EvalError::DivByZero); }
                Ok(a.eval(env)? / bv)
            }
            Expr::Neg(a) => Ok(-a.eval(env)?),
        }
    }
}

// 几个构造帮手，让构造表达式更顺
fn n(x: f64) -> Expr { Expr::Num(x) }
fn v(s: &str) -> Expr { Expr::Var(s.into()) }
fn add(a: Expr, b: Expr) -> Expr { Expr::Add(Box::new(a), Box::new(b)) }
fn sub(a: Expr, b: Expr) -> Expr { Expr::Sub(Box::new(a), Box::new(b)) }
fn mul(a: Expr, b: Expr) -> Expr { Expr::Mul(Box::new(a), Box::new(b)) }
fn div(a: Expr, b: Expr) -> Expr { Expr::Div(Box::new(a), Box::new(b)) }
fn neg(a: Expr) -> Expr { Expr::Neg(Box::new(a)) }

// ============================================================================
// 4. 链表的"持有共享尾部"：Rc 简介
// ============================================================================
//
// Box 是独占的——一份数据只有一个 Box 指向它。
// 如果你想让多个链表共享同一个尾部（典型函数式语言风格），需要用 Rc：
//
//   enum List<T> { Cons(T, Rc<List<T>>), Nil }
//
// 这部分会在第 17 章智能指针展开。这里先打个预告，让你知道"递归 enum"
// 的高级形态。

fn main() {
    println!("===== 1. 链表 =====");
    let lst = List::from_slice(&[1, 2, 3, 4, 5]);
    println!("  结构: {lst:?}");
    println!("  长度: {}", lst.len());
    println!("  求和: {}", lst.sum());
    println!("  转 Vec: {:?}", lst.to_vec());

    println!("\n===== 2. JSON Value =====");
    // 手动构造 {"name":"alice","age":30,"tags":["admin","vip"], "addr":{"city":"shanghai"}}
    let mut addr = HashMap::new();
    addr.insert("city".to_string(), JsonValue::String("shanghai".into()));

    let mut root = HashMap::new();
    root.insert("name".to_string(), JsonValue::String("alice".into()));
    root.insert("age".to_string(), JsonValue::Number(30.0));
    root.insert(
        "tags".to_string(),
        JsonValue::Array(vec![
            JsonValue::String("admin".into()),
            JsonValue::String("vip".into()),
        ]),
    );
    root.insert("addr".to_string(), JsonValue::Object(addr));

    let json = JsonValue::Object(root);
    println!("{}", json.to_pretty(0));

    println!("\n  按 path 取值:");
    println!("  json[\"name\"]   = {:?}", json.get("name"));
    println!("  json[\"missing\"]= {:?}", json.get("missing"));

    println!("\n===== 3. 表达式树（AST）=====");
    // 构造  ((x + 2) * 3) - (-y)
    let expr = sub(mul(add(v("x"), n(2.0)), n(3.0)), neg(v("y")));
    println!("  expr = {expr}");

    let env: HashMap<&str, f64> = [("x", 4.0), ("y", 1.5)].into_iter().collect();
    match expr.eval(&env) {
        Ok(v) => println!("  在 x=4, y=1.5 下求值 = {v}"),
        Err(e) => println!("  求值出错: {e:?}"),
    }

    // 报错路径：变量未绑定
    let env2: HashMap<&str, f64> = [("x", 4.0)].into_iter().collect();
    match expr.eval(&env2) {
        Ok(v) => println!("  无 y: {v}"),
        Err(e) => println!("  无 y: 报错 {e:?}"),
    }

    // 报错路径：除以 0
    let div_by_zero = div(n(1.0), sub(n(2.0), n(2.0)));
    match div_by_zero.eval(&HashMap::new()) {
        Ok(v) => println!("  1/(2-2) = {v}"),
        Err(e) => println!("  1/(2-2) -> 报错 {e:?}"),
    }

    println!("\n===== 内存大小 =====");
    use std::mem::size_of;
    println!("  size_of::<List>()      = {}", size_of::<List>());
    println!("  size_of::<JsonValue>() = {}", size_of::<JsonValue>());
    println!("  size_of::<Expr>()      = {}", size_of::<Expr>());

    println!("\n===== 要点回顾 =====");
    println!("· 自含的 enum 需要 Box（或其它指针）打破无限递归");
    println!("· JSON / AST / Tree / Linked List 这类天然递归的数据结构是 enum 的舞台");
    println!("· 递归求值就是'递归 match'：每种变体写一行处理规则");
}
