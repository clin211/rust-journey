//! 12. 综合练习：用 enum + match 实现一个简单计算器
//!
//! 运行：cargo run --example 12_calculator
//! 测试：cargo test --example 12_calculator
//!
//! README 中点名要做的练习：
//!     "用 enum + match 实现一个简单的计算器（加减乘除 + 错误处理）"
//!
//! 本例覆盖：
//! - 用 enum 表达"运算"和"错误"两类信息
//! - 函数返回 Result，让加减乘除统一接口
//! - 支持中缀字符串解析（最简版）→ 演示 enum 在编译器/计算器场景的实际样子
//! - 单元测试与 #[cfg(test)] 配合

#![allow(dead_code)]

// ============================================================================
// 1. 用 enum 表达"运算"
// ============================================================================
//
// 一个计算器最少需要回答两个问题：
//   - 我要做什么运算？      → Op 枚举（Add/Sub/Mul/Div）
//   - 出错了怎么报？        → CalcError 枚举（DivByZero/Overflow/...）
//
// 把它们都做成 enum，是 Rust 的惯用风格。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    /// 把字符（+/-/*/x// /÷）解析成 Op
    pub fn parse(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Add),
            '-' => Some(Op::Sub),
            '*' | 'x' | '×' => Some(Op::Mul),
            '/' | '÷' => Some(Op::Div),
            _ => None,
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }
}

// ============================================================================
// 2. 用 enum 表达"错误"
// ============================================================================
//
// 把可能出错的所有情况都列出来，每种情况携带必要的上下文。
// 这样调用方拿到 Err 后能进一步分类处理。

#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivByZero,
    Overflow,           // 整数溢出
    InvalidNumber(String),
    InvalidOperator(String),
    InvalidSyntax(String),
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::DivByZero => write!(f, "除数不能为 0"),
            CalcError::Overflow => write!(f, "整数运算溢出"),
            CalcError::InvalidNumber(s) => write!(f, "非法数字: {s}"),
            CalcError::InvalidOperator(s) => write!(f, "非法运算符: {s}"),
            CalcError::InvalidSyntax(s) => write!(f, "非法表达式: {s}"),
        }
    }
}

// 实现 std::error::Error，让它能融入标准错误生态（实际工程会配合 thiserror）
impl std::error::Error for CalcError {}

// ============================================================================
// 3. 核心计算
// ============================================================================
//
// 用 i64 + checked_xxx 系列做带"溢出检测"的整数运算。
// 任何分支失败都返回 CalcError，让调用方自己决定怎么处理。

pub fn calc(a: i64, op: Op, b: i64) -> Result<i64, CalcError> {
    match op {
        Op::Add => a.checked_add(b).ok_or(CalcError::Overflow),
        Op::Sub => a.checked_sub(b).ok_or(CalcError::Overflow),
        Op::Mul => a.checked_mul(b).ok_or(CalcError::Overflow),
        Op::Div => {
            if b == 0 {
                Err(CalcError::DivByZero)
            } else {
                a.checked_div(b).ok_or(CalcError::Overflow)
            }
        }
    }
}

// ============================================================================
// 4. 解析中缀表达式
// ============================================================================
//
// 这是最最简单的"a op b" 解析，不支持优先级和括号。
// 想要支持完整表达式可以走 Shunting-yard / 递归下降，那是 11 章 AST 的进阶版。

pub fn parse_binary(input: &str) -> Result<(i64, Op, i64), CalcError> {
    // 找第一个非数字、非空白的符号当 op
    let trimmed = input.trim();

    // 注意：要排除"前导负号"——`-3 + 5` 里第一个 - 不是运算符
    let mut chars = trimmed.char_indices();

    // 先吃掉可能的前导负号
    let _first = chars.next();
    // 然后从下一个位置开始找运算符
    for (i, c) in chars {
        if let Some(op) = Op::parse(c) {
            // 找到了运算符，前后切两段
            let (left, right) = trimmed.split_at(i);
            let right = &right[c.len_utf8()..];   // 跳过运算符本身

            let a: i64 = left
                .trim()
                .parse()
                .map_err(|_| CalcError::InvalidNumber(left.trim().into()))?;
            let b: i64 = right
                .trim()
                .parse()
                .map_err(|_| CalcError::InvalidNumber(right.trim().into()))?;

            return Ok((a, op, b));
        }
    }

    Err(CalcError::InvalidSyntax(input.into()))
}

/// 一站式 API：从字符串到结果
pub fn evaluate(input: &str) -> Result<i64, CalcError> {
    let (a, op, b) = parse_binary(input)?;
    calc(a, op, b)
}

// ============================================================================
// 5. 实际使用：把表达式 → 结果或友好的错误信息
// ============================================================================

fn pretty_run(input: &str) {
    match evaluate(input) {
        Ok(v) => println!("  {input:>10} => {v}"),
        Err(e) => println!("  {input:>10} => 错误: {e}"),
    }
}

fn main() {
    println!("===== 1. 直接调用 calc =====");
    let cases = [
        (3_i64, Op::Add, 4_i64),
        (10, Op::Sub, 7),
        (6, Op::Mul, 7),
        (20, Op::Div, 4),
        (10, Op::Div, 0),                    // 除 0
        (i64::MAX, Op::Add, 1),              // 溢出
    ];
    for (a, op, b) in cases {
        match calc(a, op, b) {
            Ok(v) => println!("  {a} {} {b} = {v}", op.symbol()),
            Err(e) => println!("  {a} {} {b} -> 错误: {e}", op.symbol()),
        }
    }

    println!("\n===== 2. 字符串表达式 =====");
    for input in [
        "3 + 4",
        "10 - 7",
        "6*7",
        "20 / 4",
        "10/0",
        "-3 + 5",
        "abc + 5",
        "5 % 2",       // 不支持的运算符
        "what",
    ] {
        pretty_run(input);
    }

    println!("\n===== 3. 综合：一组表达式批量求值 =====");
    let total: i64 = [
        "1 + 2",
        "3 * 4",
        "100 - 30",
        "16 / 4",
    ]
    .into_iter()
    .filter_map(|s| evaluate(s).ok())
    .sum();
    println!("  汇总 = {total}");

    println!("\n===== 要点回顾 =====");
    println!("· 用 enum 同时建模 'Op'（要做什么）和 'Error'（出了啥事）");
    println!("· calc 返回 Result<i64, CalcError>，让调用方决定怎么处理失败");
    println!("· checked_xxx 系列是处理整数溢出的标准方式");
    println!("· #[cfg(test)] 模块里写单元测试，覆盖 happy path + 各种错误路径");
}

// ============================================================================
// 6. 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- calc 部分 ----

    #[test]
    fn basic_add() {
        assert_eq!(calc(3, Op::Add, 4), Ok(7));
        assert_eq!(calc(-3, Op::Add, 5), Ok(2));
    }

    #[test]
    fn basic_sub() {
        assert_eq!(calc(10, Op::Sub, 7), Ok(3));
        assert_eq!(calc(0, Op::Sub, 5), Ok(-5));
    }

    #[test]
    fn basic_mul() {
        assert_eq!(calc(6, Op::Mul, 7), Ok(42));
        assert_eq!(calc(0, Op::Mul, 999), Ok(0));
    }

    #[test]
    fn basic_div() {
        assert_eq!(calc(20, Op::Div, 4), Ok(5));
        assert_eq!(calc(7, Op::Div, 2), Ok(3));        // 整除截断
    }

    #[test]
    fn div_zero_returns_err() {
        assert_eq!(calc(10, Op::Div, 0), Err(CalcError::DivByZero));
    }

    #[test]
    fn overflow_add() {
        assert_eq!(calc(i64::MAX, Op::Add, 1), Err(CalcError::Overflow));
    }

    #[test]
    fn overflow_mul() {
        assert_eq!(calc(i64::MAX, Op::Mul, 2), Err(CalcError::Overflow));
    }

    // ---- parse_binary 部分 ----

    #[test]
    fn parse_simple() {
        assert_eq!(parse_binary("3+4"), Ok((3, Op::Add, 4)));
        assert_eq!(parse_binary("3 + 4"), Ok((3, Op::Add, 4)));
        assert_eq!(parse_binary("  10 / 2  "), Ok((10, Op::Div, 2)));
    }

    #[test]
    fn parse_with_negative_left() {
        assert_eq!(parse_binary("-3 + 5"), Ok((-3, Op::Add, 5)));
    }

    #[test]
    fn parse_invalid_number() {
        assert!(matches!(
            parse_binary("abc + 5"),
            Err(CalcError::InvalidNumber(_))
        ));
    }

    #[test]
    fn parse_no_operator() {
        assert!(matches!(
            parse_binary("foobar"),
            Err(CalcError::InvalidSyntax(_))
        ));
    }

    // ---- evaluate（端到端）----

    #[test]
    fn end_to_end_ok() {
        assert_eq!(evaluate("12 * 3"), Ok(36));
    }

    #[test]
    fn end_to_end_div_zero() {
        assert_eq!(evaluate("10 / 0"), Err(CalcError::DivByZero));
    }

    #[test]
    fn op_parse() {
        assert_eq!(Op::parse('+'), Some(Op::Add));
        assert_eq!(Op::parse('×'), Some(Op::Mul));
        assert_eq!(Op::parse('÷'), Some(Op::Div));
        assert_eq!(Op::parse('?'), None);
    }
}
