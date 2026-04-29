//! 13. 命令模式：用 enum 表达"要执行的动作"
//!
//! 运行：cargo run --example 13_command_pattern
//!
//! 命令模式是 enum 在工程里出镜率最高的几个场景之一：
//! - CLI / Web 路由的 subcommand
//! - Redux / Flux 风格的 Action
//! - 事件溯源（Event Sourcing）的命令日志
//! - 工作流引擎、UI 状态更新
//!
//! 本例把"待办列表"做成一个最小实例。
//! 设计要点：
//! - 一个 enum 把所有命令列出来（穷尽性 → 不会漏处理）
//! - 一个 reducer 函数：(State, Command) -> State
//! - State 自身用 struct（多个并存字段），互斥的命令用 enum
//! - 让数据驱动逻辑：调用方关心 Command，业务关心 State

#![allow(dead_code)]

use std::collections::HashMap;

// ============================================================================
// 1. 数据模型：state 用 struct，command 用 enum
// ============================================================================
//
// 这是 enum/struct 选型的经典对照：
//   - state（多个字段同时存在）→ struct
//   - command（一组互斥的"要做什么"）→ enum

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub todos: HashMap<u64, Todo>,
    pub next_id: u64,
}

#[derive(Debug)]
pub enum Command {
    /// 新建一个 todo
    Add { title: String },
    /// 把指定 id 的 todo 标为完成
    Complete { id: u64 },
    /// 反转完成状态
    Toggle { id: u64 },
    /// 修改 title
    Rename { id: u64, title: String },
    /// 删除
    Remove { id: u64 },
    /// 一次清空所有已完成的
    ClearCompleted,
    /// 批量执行多条命令
    Batch(Vec<Command>),
}

// ============================================================================
// 2. 错误类型：表达"命令处理时可能出现的问题"
// ============================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum CommandError {
    NotFound(u64),
    EmptyTitle,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::NotFound(id) => write!(f, "todo #{id} 不存在"),
            CommandError::EmptyTitle => write!(f, "标题不能为空"),
        }
    }
}

// ============================================================================
// 3. reducer：核心入口
// ============================================================================
//
// reducer 把"当前状态 + 命令" 变成"新状态"。
// 这就是 Redux / Elm 架构的核心思想。
//
// 注意：
// - reducer 内部"穷尽 match" 所有命令变体——新增命令时编译器逼你来这里加分支
// - 失败用 Result，不在 reducer 里 panic
// - Batch 是递归的：一个命令可能由多个子命令组成，自然递归处理

impl AppState {
    pub fn new() -> Self { Self::default() }

    pub fn dispatch(&mut self, cmd: Command) -> Result<(), CommandError> {
        match cmd {
            Command::Add { title } => self.add(title),
            Command::Complete { id } => self.set_done(id, true),
            Command::Toggle { id } => self.toggle(id),
            Command::Rename { id, title } => self.rename(id, title),
            Command::Remove { id } => self.remove(id),
            Command::ClearCompleted => self.clear_completed(),
            Command::Batch(cmds) => {
                for c in cmds {
                    self.dispatch(c)?;
                }
                Ok(())
            }
        }
    }

    // —— 单条命令的处理 ——

    fn add(&mut self, title: String) -> Result<(), CommandError> {
        if title.trim().is_empty() {
            return Err(CommandError::EmptyTitle);
        }
        self.next_id += 1;
        let id = self.next_id;
        self.todos.insert(id, Todo { id, title, done: false });
        Ok(())
    }

    fn set_done(&mut self, id: u64, done: bool) -> Result<(), CommandError> {
        match self.todos.get_mut(&id) {
            Some(t) => { t.done = done; Ok(()) }
            None => Err(CommandError::NotFound(id)),
        }
    }

    fn toggle(&mut self, id: u64) -> Result<(), CommandError> {
        match self.todos.get_mut(&id) {
            Some(t) => { t.done = !t.done; Ok(()) }
            None => Err(CommandError::NotFound(id)),
        }
    }

    fn rename(&mut self, id: u64, title: String) -> Result<(), CommandError> {
        if title.trim().is_empty() {
            return Err(CommandError::EmptyTitle);
        }
        match self.todos.get_mut(&id) {
            Some(t) => { t.title = title; Ok(()) }
            None => Err(CommandError::NotFound(id)),
        }
    }

    fn remove(&mut self, id: u64) -> Result<(), CommandError> {
        match self.todos.remove(&id) {
            Some(_) => Ok(()),
            None => Err(CommandError::NotFound(id)),
        }
    }

    fn clear_completed(&mut self) -> Result<(), CommandError> {
        self.todos.retain(|_, t| !t.done);
        Ok(())
    }

    // —— 视图 / 查询接口 ——

    pub fn pending_count(&self) -> usize {
        self.todos.values().filter(|t| !t.done).count()
    }

    pub fn dump(&self) {
        let mut ids: Vec<&u64> = self.todos.keys().collect();
        ids.sort();
        for id in ids {
            let t = &self.todos[id];
            let mark = if t.done { "[x]" } else { "[ ]" };
            println!("    {} #{} {}", mark, t.id, t.title);
        }
    }
}

// ============================================================================
// 4. 一个"路由"用法：把 enum 变体当作 HTTP 路由 / CLI 子命令
// ============================================================================
//
// clap 4.x、actix-web、axum 等真实框架里，一个 #[derive(Subcommand)] 的 enum
// 就是 CLI 子命令的本体。这里给出一个简化版：

#[derive(Debug)]
enum Route {
    Index,
    UserDetail { id: u64 },
    UserEdit { id: u64 },
    Search { keyword: String, page: u32 },
    NotFound(String),
}

fn handle_route(route: Route) -> String {
    match route {
        Route::Index => "GET /".into(),
        Route::UserDetail { id } => format!("GET /users/{id}"),
        Route::UserEdit { id } => format!("PATCH /users/{id}"),
        Route::Search { keyword, page } => format!("GET /search?q={keyword}&page={page}"),
        Route::NotFound(path) => format!("404 - {path}"),
    }
}

// ============================================================================
// 5. 进阶：把 Command 序列化成日志（Event Sourcing 的影子）
// ============================================================================
//
// 真正的 Event Sourcing 需要把 Command 持久化为日志，重启后回放。
// 这里只演示"打印"——把 enum 变体打成可读字符串。

fn log_command(cmd: &Command) {
    match cmd {
        Command::Add { title } => println!("    LOG add[{title}]"),
        Command::Complete { id } => println!("    LOG complete[{id}]"),
        Command::Toggle { id } => println!("    LOG toggle[{id}]"),
        Command::Rename { id, title } => println!("    LOG rename[{id} → {title}]"),
        Command::Remove { id } => println!("    LOG remove[{id}]"),
        Command::ClearCompleted => println!("    LOG clear-completed"),
        Command::Batch(cmds) => {
            println!("    LOG batch[{}条]", cmds.len());
            for c in cmds {
                log_command(c);
            }
        }
    }
}

fn main() {
    println!("===== 1. 启动一个空的 TODO 应用 =====");
    let mut app = AppState::new();

    let initial = Command::Batch(vec![
        Command::Add { title: "学习 enum".into() },
        Command::Add { title: "学习 match".into() },
        Command::Add { title: "做计算器练习".into() },
    ]);
    log_command(&initial);
    app.dispatch(initial).unwrap();
    println!("  当前列表:");
    app.dump();

    println!("\n===== 2. 标记完成 + 重命名 =====");
    let mut updates = vec![
        Command::Complete { id: 1 },
        Command::Rename { id: 2, title: "深入掌握 match 语法".into() },
        Command::Toggle { id: 3 },
    ];
    for cmd in updates.drain(..) {
        log_command(&cmd);
        app.dispatch(cmd).unwrap();
    }
    app.dump();

    println!("\n===== 3. 错误路径 =====");
    let bad_cases = [
        Command::Add { title: "   ".into() },          // 空标题
        Command::Complete { id: 9999 },                 // 不存在
        Command::Rename { id: 1, title: "".into() },    // 空标题
    ];
    for cmd in bad_cases {
        log_command(&cmd);
        match app.dispatch(cmd) {
            Ok(()) => println!("    ok"),
            Err(e) => println!("    err: {e}"),
        }
    }

    println!("\n===== 4. 清理已完成 =====");
    app.dispatch(Command::ClearCompleted).unwrap();
    app.dump();
    println!("  剩余未完成: {}", app.pending_count());

    println!("\n===== 5. Route 路由派发 =====");
    let routes = [
        Route::Index,
        Route::UserDetail { id: 7 },
        Route::UserEdit { id: 7 },
        Route::Search { keyword: "rust".into(), page: 2 },
        Route::NotFound("/never/exist".into()),
    ];
    for r in routes {
        println!("  → {}", handle_route(r));
    }

    println!("\n===== 要点回顾 =====");
    println!("· state 用 struct（同时存在的字段）；command 用 enum（互斥的动作）");
    println!("· dispatch 是个穷尽 match：新增命令时编译器强制让你来这里加分支");
    println!("· 失败用 Result，不在 reducer 内 panic（保证可测试）");
    println!("· 同一份思想能用在 路由 / 工作流 / Action / Event Sourcing 等场景");
}

// ============================================================================
// 6. 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> AppState { AppState::new() }

    #[test]
    fn add_and_count() {
        let mut s = fresh();
        s.dispatch(Command::Add { title: "a".into() }).unwrap();
        s.dispatch(Command::Add { title: "b".into() }).unwrap();
        assert_eq!(s.todos.len(), 2);
        assert_eq!(s.pending_count(), 2);
    }

    #[test]
    fn empty_title_rejected() {
        let mut s = fresh();
        assert_eq!(
            s.dispatch(Command::Add { title: "  ".into() }),
            Err(CommandError::EmptyTitle)
        );
    }

    #[test]
    fn complete_unknown_id() {
        let mut s = fresh();
        assert_eq!(
            s.dispatch(Command::Complete { id: 42 }),
            Err(CommandError::NotFound(42))
        );
    }

    #[test]
    fn batch_aborts_on_error() {
        let mut s = fresh();
        let batch = Command::Batch(vec![
            Command::Add { title: "ok".into() },
            Command::Complete { id: 9999 },          // ← 这里失败
            Command::Add { title: "never".into() },  // ← 不会执行
        ]);
        let r = s.dispatch(batch);
        assert!(matches!(r, Err(CommandError::NotFound(9999))));
        assert_eq!(s.todos.len(), 1);                 // 只有第一条进去了
    }

    #[test]
    fn clear_completed() {
        let mut s = fresh();
        s.dispatch(Command::Add { title: "a".into() }).unwrap();
        s.dispatch(Command::Add { title: "b".into() }).unwrap();
        s.dispatch(Command::Complete { id: 1 }).unwrap();
        s.dispatch(Command::ClearCompleted).unwrap();
        assert_eq!(s.todos.len(), 1);
    }
}
