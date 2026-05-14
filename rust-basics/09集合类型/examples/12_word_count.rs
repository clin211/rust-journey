//! 12. 综合实战：文本词频统计
//!
//! 运行：cargo run --example 12_word_count
//! 测试：cargo test --example 12_word_count
//!
//! README 中点名要做的练习：
//!     "统计一段文本中每个单词的出现次数（用 HashMap）"
//!
//! 我们把它做成一个完整的小工具:
//! - 多种归一化策略 (大小写、标点)
//! - 统计 + 排序输出
//! - 处理停用词
//! - 可对外的纯函数 + 单元测试

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

// ============================================================================
// 1. 数据模型
// ============================================================================

#[derive(Debug, Clone)]
pub struct WordCounter {
    counts: HashMap<String, u32>,
    stopwords: HashSet<String>,
    case_insensitive: bool,
}

impl WordCounter {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
            stopwords: HashSet::new(),
            case_insensitive: true,
        }
    }

    /// 配置: 是否大小写不敏感（默认 true）
    pub fn case_sensitive(mut self) -> Self {
        self.case_insensitive = false;
        self
    }

    /// 配置: 加入停用词
    pub fn with_stopwords<I, S>(mut self, words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for w in words {
            let mut s = w.into();
            if self.case_insensitive {
                s = s.to_lowercase();
            }
            self.stopwords.insert(s);
        }
        self
    }

    /// 喂入一段文本
    pub fn feed(&mut self, text: &str) {
        for raw in text.split(|c: char| !c.is_alphanumeric()) {
            if raw.is_empty() {
                continue;
            }
            let word = if self.case_insensitive {
                raw.to_lowercase()
            } else {
                raw.to_string()
            };
            if self.stopwords.contains(&word) {
                continue;
            }
            *self.counts.entry(word).or_insert(0) += 1;
        }
    }

    /// 取某个词的次数
    pub fn count(&self, word: &str) -> u32 {
        let key = if self.case_insensitive {
            word.to_lowercase()
        } else {
            word.to_string()
        };
        self.counts.get(&key).copied().unwrap_or(0)
    }

    /// 不重复词的数量
    pub fn unique_words(&self) -> usize {
        self.counts.len()
    }

    /// 总词数
    pub fn total_words(&self) -> u32 {
        self.counts.values().sum()
    }

    /// Top-N 高频词 (频次降序, 频次相同按字母升序)
    pub fn top(&self, n: usize) -> Vec<(String, u32)> {
        let mut pairs: Vec<(String, u32)> = self
            .counts
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        pairs.truncate(n);
        pairs
    }
}

impl Default for WordCounter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. 一行式版本：能不分类先来一行简单的
// ============================================================================
//
// 真实工程里常用的"快糙猛"写法。一两行就能搞定。

pub fn word_freq(text: &str) -> HashMap<String, u32> {
    let mut m: HashMap<String, u32> = HashMap::new();
    for w in text.split_whitespace() {
        let clean: String = w
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();
        if clean.is_empty() {
            continue;
        }
        *m.entry(clean).or_insert(0) += 1;
    }
    m
}

// ============================================================================
// 3. 漂亮打印
// ============================================================================

fn print_top(counter: &WordCounter, n: usize) {
    let top = counter.top(n);
    let max_w = top.iter().map(|(w, _)| w.chars().count()).max().unwrap_or(0);
    let max_n = top.first().map(|(_, n)| *n).unwrap_or(0);
    let bw = 30usize;

    println!("  ╭─{:─<width$}─┬───────┬{:─<bw$}─╮", "", "", width = max_w, bw = bw);
    println!("  │ {:width$} │ {:>5} │ {:bw$} │", "word", "count", "bar", width = max_w, bw = bw);
    println!("  ├─{:─<width$}─┼───────┼{:─<bw$}─┤", "", "", width = max_w, bw = bw);
    for (w, n) in &top {
        let len = if max_n == 0 { 0 } else { (n * bw as u32 / max_n) as usize };
        let bar = "█".repeat(len);
        println!("  │ {w:width$} │ {n:>5} │ {bar:bw$} │", width = max_w, bw = bw);
    }
    println!("  ╰─{:─<width$}─┴───────┴{:─<bw$}─╯", "", "", width = max_w, bw = bw);
}

fn main() {
    println!("===== 1. 一行式 word_freq =====");
    let s = "The quick brown fox jumps over the lazy dog. The dog was sleeping.";
    let m = word_freq(s);
    let mut pairs: Vec<_> = m.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (w, n) in &pairs {
        println!("  {w:>10} -> {n}");
    }

    println!("\n===== 2. WordCounter 完整版（含停用词、Top-N）=====");
    let mut c = WordCounter::new()
        .with_stopwords(["the", "a", "an", "and", "or", "of", "to", "in", "on", "is", "was", "be"]);
    let text = r#"
        Rust is a systems programming language that runs blazingly fast,
        prevents segfaults, and guarantees thread safety.
        Rust is a great language. Rust empowers everyone to build reliable
        and efficient software. Performance is a feature in Rust.
    "#;
    c.feed(text);

    println!("  全部词数:       {}", c.total_words());
    println!("  不同词数:       {}", c.unique_words());
    println!("  'rust' 出现次数: {}", c.count("Rust"));      // 大小写不敏感
    println!("  'fast' 出现次数: {}", c.count("fast"));
    println!();

    print_top(&c, 8);

    println!("\n===== 3. 大小写敏感模式 =====");
    let mut cs = WordCounter::new().case_sensitive();
    cs.feed("Rust rust RUST rUsT");
    let top = cs.top(10);
    for (w, n) in &top {
        println!("  {w} -> {n}");
    }

    println!("\n===== 要点回顾 =====");
    println!("· word_freq 一行式: 适合脚本 / demo");
    println!("· WordCounter 工程版: 配置化 + 停用词 + Top-N");
    println!("· 用 entry().or_insert(0) += 1 做计数, 是最经典模式");
    println!("· 排序后输出: 频次降序, 相同频次按字典序升序");
}

// ============================================================================
// 4. 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_liner_basic() {
        let m = word_freq("the cat sat on the mat");
        assert_eq!(m.get("the"), Some(&2));
        assert_eq!(m.get("cat"), Some(&1));
        assert_eq!(m.get("nope"), None);
    }

    #[test]
    fn one_liner_punct() {
        let m = word_freq("Hello, world! Hello? hello.");
        assert_eq!(m.get("hello"), Some(&3));
        assert_eq!(m.get("world"), Some(&1));
    }

    #[test]
    fn counter_default_case_insensitive() {
        let mut c = WordCounter::new();
        c.feed("Rust RUST rust");
        assert_eq!(c.count("rust"), 3);
        assert_eq!(c.count("Rust"), 3);
        assert_eq!(c.count("RUST"), 3);
    }

    #[test]
    fn counter_case_sensitive() {
        let mut c = WordCounter::new().case_sensitive();
        c.feed("Rust RUST rust");
        assert_eq!(c.count("Rust"), 1);
        assert_eq!(c.count("RUST"), 1);
        assert_eq!(c.count("rust"), 1);
    }

    #[test]
    fn counter_stopwords() {
        let mut c = WordCounter::new().with_stopwords(["the", "is"]);
        c.feed("the cat is on the mat");
        assert_eq!(c.count("cat"), 1);
        assert_eq!(c.count("the"), 0);
        assert_eq!(c.count("is"), 0);
        assert_eq!(c.unique_words(), 3);     // cat, on, mat
    }

    #[test]
    fn counter_top_n_ordering() {
        let mut c = WordCounter::new();
        c.feed("a b c b c c");
        let top = c.top(2);
        assert_eq!(top, vec![("c".into(), 3), ("b".into(), 2)]);

        let top3 = c.top(3);
        assert_eq!(top3, vec![("c".into(), 3), ("b".into(), 2), ("a".into(), 1)]);
    }

    #[test]
    fn counter_top_n_tiebreak_alphabetical() {
        // 频次相同时, 按字典升序
        let mut c = WordCounter::new();
        c.feed("zebra apple mango apple zebra mango");
        let top = c.top(3);
        // apple/mango/zebra 都是 2 次, 字典序 apple < mango < zebra
        assert_eq!(
            top,
            vec![("apple".into(), 2), ("mango".into(), 2), ("zebra".into(), 2)]
        );
    }

    #[test]
    fn counter_aggregates() {
        let mut c = WordCounter::new();
        c.feed("a b a c b a");
        assert_eq!(c.total_words(), 6);
        assert_eq!(c.unique_words(), 3);
    }

    #[test]
    fn counter_punctuation_stripped() {
        let mut c = WordCounter::new();
        c.feed("Hello, world! Hello? hello.");
        assert_eq!(c.count("hello"), 3);
        assert_eq!(c.count("world"), 1);
    }
}
