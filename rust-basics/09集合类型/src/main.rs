//! 第 09 章 集合类型 —— 综合演示入口
//!
//! 运行：cargo run
//! 想看具体某个知识点：cargo run --example NN_xxxxx
//!
//! 本文件用一个 "图书馆借阅系统" 把本章核心概念串成一个最小完整流程：
//! - Vec<Book>：所有藏书
//! - HashMap<u64, Vec<u64>>：借阅记录（reader_id → book_ids）
//! - HashSet<u64>：当前在借的 book_ids
//! - BTreeMap<&str, u32>：按分类的统计（自带有序输出）
//! - String / &str：标题/作者/查询关键字

#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap, HashSet};

// ============================================================================
// 1. 数据模型
// ============================================================================

#[derive(Debug, Clone)]
struct Book {
    id: u64,
    title: String,
    author: String,
    category: &'static str,
    copies: u32,
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
    /// 借阅记录: reader_id -> 借走的 book_id 列表
    loans: HashMap<u64, Vec<u64>>,
    /// 当前在借的 book_id（用 HashSet 快速判定"是否被借走"）
    on_loan: HashSet<u64>,
}

#[derive(Debug)]
enum BorrowError {
    BookNotFound(u64),
    AlreadyBorrowed(u64),
    NotBorrowed(u64),
    NoCopies(u64),
}

impl Library {
    fn new() -> Self {
        Library {
            books: Vec::new(),
            loans: HashMap::new(),
            on_loan: HashSet::new(),
        }
    }

    fn add_book(&mut self, b: Book) {
        self.books.push(b);
    }

    fn find_by_id(&self, id: u64) -> Option<&Book> {
        self.books.iter().find(|b| b.id == id)
    }

    fn search_by_title(&self, keyword: &str) -> Vec<&Book> {
        let kw = keyword.to_lowercase();
        self.books
            .iter()
            .filter(|b| b.title.to_lowercase().contains(&kw))
            .collect()
    }

    fn category_stats(&self) -> BTreeMap<&'static str, u32> {
        let mut stats: BTreeMap<&'static str, u32> = BTreeMap::new();
        for b in &self.books {
            *stats.entry(b.category).or_insert(0) += b.copies;
        }
        stats
    }

    fn borrow(&mut self, reader_id: u64, book_id: u64) -> Result<(), BorrowError> {
        let book = self
            .books
            .iter()
            .find(|b| b.id == book_id)
            .ok_or(BorrowError::BookNotFound(book_id))?;

        // 已借走的不能再借
        if self.on_loan.contains(&book_id) {
            return Err(BorrowError::AlreadyBorrowed(book_id));
        }
        if book.copies == 0 {
            return Err(BorrowError::NoCopies(book_id));
        }

        self.on_loan.insert(book_id);
        self.loans
            .entry(reader_id)
            .or_insert_with(Vec::new)
            .push(book_id);
        Ok(())
    }

    fn return_book(&mut self, reader_id: u64, book_id: u64) -> Result<(), BorrowError> {
        if !self.on_loan.contains(&book_id) {
            return Err(BorrowError::NotBorrowed(book_id));
        }
        let list = self
            .loans
            .get_mut(&reader_id)
            .ok_or(BorrowError::NotBorrowed(book_id))?;
        let pos = list
            .iter()
            .position(|&id| id == book_id)
            .ok_or(BorrowError::NotBorrowed(book_id))?;
        list.swap_remove(pos);
        self.on_loan.remove(&book_id);
        Ok(())
    }

    /// 当前借阅总量
    fn current_loans(&self) -> usize {
        self.on_loan.len()
    }

    /// 没人借的书 (用集合差集思路)
    fn untouched(&self) -> Vec<&Book> {
        let all_ids: HashSet<u64> = self.books.iter().map(|b| b.id).collect();
        let touched: HashSet<u64> = self.loans.values().flatten().copied().collect();
        let mut diff: Vec<u64> = all_ids.difference(&touched).copied().collect();
        diff.sort();
        diff.iter().filter_map(|id| self.find_by_id(*id)).collect()
    }
}

// ============================================================================
// 2. 演示流程
// ============================================================================

fn run() -> Result<(), BorrowError> {
    let mut lib = Library::new();

    // 添加藏书
    let books = [
        ("The Rust Programming Language", "Steve Klabnik", "tech", 5),
        ("Programming Rust", "Jim Blandy", "tech", 3),
        ("Designing Data-Intensive Applications", "Martin Kleppmann", "tech", 4),
        ("百年孤独", "García Márquez", "fiction", 2),
        ("活着", "余华", "fiction", 6),
        ("人类简史", "Yuval Harari", "history", 4),
        ("人物群像研究", "佚名", "history", 1),
    ];
    for (i, (t, a, c, copies)) in books.iter().enumerate() {
        lib.add_book(Book {
            id: 1000 + i as u64,
            title: (*t).into(),
            author: (*a).into(),
            category: c,
            copies: *copies,
        });
    }

    println!("===== 1. 藏书概况 =====");
    println!("  藏书总数: {}", lib.books.len());
    println!("  分类统计 (BTreeMap, 自带有序):");
    for (cat, n) in lib.category_stats() {
        println!("    {cat:>10} -> {n} 本");
    }

    println!("\n===== 2. 关键字搜索 =====");
    for kw in ["rust", "活", "history"] {
        let hits = lib.search_by_title(kw);
        println!("  '{kw}' 命中 {} 本", hits.len());
        for b in hits {
            println!("    #{}  《{}》  by {}", b.id, b.title, b.author);
        }
    }

    println!("\n===== 3. 借阅流程 =====");
    let alice = 9001u64;
    let bob = 9002u64;
    lib.borrow(alice, 1000)?;
    lib.borrow(alice, 1004)?;
    lib.borrow(bob, 1003)?;

    // 重复借同一本: 应该被拒
    match lib.borrow(bob, 1000) {
        Ok(()) => println!("  (不应到这一步)"),
        Err(e) => println!("  按预期失败: {e:?}"),
    }

    println!("  当前在借数量: {}", lib.current_loans());
    println!("  alice 借走: {:?}", lib.loans.get(&alice));
    println!("  bob 借走:   {:?}", lib.loans.get(&bob));

    println!("\n===== 4. 还书 =====");
    lib.return_book(alice, 1000)?;
    println!("  alice 还了 1000 后, 当前在借: {}", lib.current_loans());

    println!("\n===== 5. 没人借的书（集合差集）=====");
    let untouched = lib.untouched();
    for b in untouched {
        println!("  #{}  《{}》", b.id, b.title);
    }

    println!("\n===== 6. 计数器: 各分类被借走的次数 =====");
    let mut counter: HashMap<&str, u32> = HashMap::new();
    for ids in lib.loans.values() {
        for id in ids {
            if let Some(b) = lib.find_by_id(*id) {
                *counter.entry(b.category).or_insert(0) += 1;
            }
        }
    }
    // 排序输出
    let mut pairs: Vec<(&&str, &u32)> = counter.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (c, n) in pairs {
        println!("  {c} -> {n} 次");
    }

    Ok(())
}

fn main() {
    println!("====== 第 09 章 综合演示：图书馆借阅系统 ======\n");
    if let Err(e) = run() {
        eprintln!("[demo] 终端错误: {e:?}");
    }

    println!("\n→ 想深入某个知识点, 可单独运行:");
    println!("    cargo run --example 01_vec_basics");
    println!("    cargo run --example 02_vec_iter");
    println!("    cargo run --example 03_vec_advanced");
    println!("    cargo run --example 04_string_basics");
    println!("    cargo run --example 05_str_vs_string");
    println!("    cargo run --example 06_utf8_iteration");
    println!("    cargo run --example 07_hashmap_basics");
    println!("    cargo run --example 08_hashmap_entry");
    println!("    cargo run --example 09_btreemap");
    println!("    cargo run --example 10_sets");
    println!("    cargo run --example 11_other_collections");
    println!("    cargo run --example 12_word_count");
    println!("    cargo run --example 13_collection_internals");
}
