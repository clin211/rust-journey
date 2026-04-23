#![allow(dead_code)]

use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 构建器模式（Builder Pattern）
//
// 一个结构体字段变多、规则变复杂时，直接 new(...) 或字面量就不够用了：
//   · 参数一长串，调用方看不清谁是谁
//   · 有可选字段，也有必填字段，用 new 无法优雅表达
//   · 构建时需要校验（端口范围 / 必填 / 互斥选项）
//
// 解决方案：**构建器模式** —— 先创建一个 Builder，一步步配置，最后 build() 出目标。
//
// 本示例分三个层次演示：
//
//   1. 「自我消费」轻量 builder：返回 Self 的 with_xxx，适合可选参数多的场景
//      （这在 07_associated_functions.rs 里已经见过，这里作为对比起点）
//
//   2. 「独立 Builder 结构体」：两阶段构建，Config 和 ConfigBuilder 分离
//      · Builder 字段可以是 Option<T>，允许"未设置"
//      · build() 时做合法性校验，返回 Result<Config, BuildError>
//      · 这是生产级库里最常见的做法（reqwest、hyper、tokio 都这样）
//
//   3. 「Type-State Builder」：用类型系统强制「必填字段必须设置」
//      · Builder 带状态参数：XxxBuilder<HasUrl, HasMethod>
//      · 编译期拒绝「缺少必填字段」的构建
//      · 零运行时开销（PhantomData）
//
// 选型指南：
//   · 字段少、都可选 → with_xxx 自消费链
//   · 字段多、有必填 + 需要校验 → 独立 Builder + build() -> Result
//   · 必填字段很关键、零错误容忍 → Type-State Builder（编译期保证）
//
// 运行时的代价：
//   Builder 模式几乎全是「编译期展开」，runtime 和「手动一次性写全参数」
//   没有区别。Rust 的零成本抽象在这里体现得淋漓尽致。
// ─────────────────────────────────────────────────────────────────────────────

// =============================================================================
// 第一层：「自我消费」轻量 builder（快速回顾）
// =============================================================================

// 适合：字段都可选，大多数时候用默认值就够
// 缺点：无法区分「没设过」vs「设成了默认值」；build() 不能做合法性校验
#[derive(Debug)]
struct LightConfig {
    name: String,
    verbose: bool,
    threads: u32,
}

impl LightConfig {
    fn new(name: &str) -> Self {
        LightConfig {
            name: name.into(),
            verbose: false,
            threads: 1,
        }
    }

    // 消费自身、返回新版本 —— 支持链式调用
    fn verbose(mut self, v: bool) -> Self {
        self.verbose = v;
        self
    }

    fn threads(mut self, n: u32) -> Self {
        self.threads = n;
        self
    }
}

// =============================================================================
// 第二层：「独立 Builder 结构体」——真正的构建器模式
// =============================================================================

// 目标结构体：我们希望构建出这个
#[derive(Debug)]
struct HttpRequest {
    method: String,                              // 必填
    url: String,                                 // 必填
    headers: Vec<(String, String)>,              // 可选：默认为空
    body: Option<String>,                        // 可选：默认为 None
    timeout_secs: u64,                           // 可选：默认为 30
}

// Builder 专用的错误类型：用 enum 表达各种可能的校验失败
#[derive(Debug)]
enum BuildError {
    MissingMethod,
    MissingUrl,
    InvalidTimeout(u64),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::MissingMethod => write!(f, "method 字段必填"),
            BuildError::MissingUrl => write!(f, "url 字段必填"),
            BuildError::InvalidTimeout(v) => write!(f, "timeout_secs 必须 > 0，收到 {}", v),
        }
    }
}

impl std::error::Error for BuildError {}

// 独立的 Builder 结构体：
//   · 字段都是 Option<T>（除了 Vec<...> 之类天然有空态的）
//   · 支持部分设置，最后 build() 时检查必填
#[derive(Default)]
struct HttpRequestBuilder {
    method: Option<String>,
    url: Option<String>,
    headers: Vec<(String, String)>,              // 没必要 Option，空 Vec 就是「未设置」
    body: Option<String>,
    timeout_secs: Option<u64>,
}

impl HttpRequestBuilder {
    fn new() -> Self {
        Self::default()
    }

    // 链式设置器 —— 用 &mut self + 返回 &mut Self，比 self 风格少一次 move
    // 但为了和示例 1 风格统一，这里用 self 风格（消费自身、返回 Self）
    fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    // 终结方法 build()：
    //   · 消费 self
    //   · 校验必填字段
    //   · 返回 Result<HttpRequest, BuildError>
    fn build(self) -> Result<HttpRequest, BuildError> {
        let method = self.method.ok_or(BuildError::MissingMethod)?;
        let url = self.url.ok_or(BuildError::MissingUrl)?;
        let timeout_secs = self.timeout_secs.unwrap_or(30);

        if timeout_secs == 0 {
            return Err(BuildError::InvalidTimeout(timeout_secs));
        }

        Ok(HttpRequest {
            method,
            url,
            headers: self.headers,
            body: self.body,
            timeout_secs,
        })
    }
}

// 给 HttpRequest 一个便捷入口：HttpRequest::builder()
impl HttpRequest {
    fn builder() -> HttpRequestBuilder {
        HttpRequestBuilder::new()
    }
}

// =============================================================================
// 第三层：Type-State Builder —— 编译期强制必填字段
// =============================================================================

// 状态标签：单元结构体，零大小
struct Empty;                                    // 未设置
struct Set;                                      // 已设置

// Builder 带两个类型参数：分别表示 url 和 method 是否已设置
// PhantomData 让这些状态参数只存在于类型系统中，运行时零开销
use std::marker::PhantomData;

struct RequestBuilder<UrlState, MethodState> {
    url: Option<String>,
    method: Option<String>,
    body: Option<String>,
    _url_state: PhantomData<UrlState>,
    _method_state: PhantomData<MethodState>,
}

// 最初状态：两个字段都未设置，Empty/Empty
impl RequestBuilder<Empty, Empty> {
    fn new() -> Self {
        RequestBuilder {
            url: None,
            method: None,
            body: None,
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

// 设置 url 后，UrlState 从 Empty 变 Set
// 注意 MethodState 保持不变（泛型参数 M 原样传递）
impl<M> RequestBuilder<Empty, M> {
    fn url(self, url: impl Into<String>) -> RequestBuilder<Set, M> {
        RequestBuilder {
            url: Some(url.into()),
            method: self.method,
            body: self.body,
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

// 设置 method 后，MethodState 从 Empty 变 Set
impl<U> RequestBuilder<U, Empty> {
    fn method(self, method: impl Into<String>) -> RequestBuilder<U, Set> {
        RequestBuilder {
            url: self.url,
            method: Some(method.into()),
            body: self.body,
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

// 可选方法：在任何状态下都能设置 body（不影响两个必填字段的状态）
impl<U, M> RequestBuilder<U, M> {
    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

// build() 方法：只在 Set/Set 状态下才定义，其它状态根本没有 build
// 这意味着：缺少必填字段时，代码在编译期就被拒绝
impl RequestBuilder<Set, Set> {
    fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            headers: Vec::new(),
            body: self.body,
            timeout_secs: 30,
        }
    }
}

fn main() {
    println!("{}", "=== 构建器模式（Builder Pattern） ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、第一层：轻量 with_xxx 链式 builder");
    // ─────────────────────────────────────────

    // 简单场景：字段全部可选，默认就够用
    let cfg = LightConfig::new("my-app")
        .verbose(true)
        .threads(8);

    println!("  cfg = {:?}", cfg);

    // 不需要任何链式调用也行 —— 默认值即可
    let cfg2 = LightConfig::new("bare");
    println!("  默认配置 = {:?}", cfg2);

    println!("  特点：实现简单，一个结构体 + 几个 with_xxx 就够");
    println!("  缺点：无法区分「没设置」和「设成了默认」；不支持 build() 校验");
    println!("小结：字段少 + 可选为主时，这是性价比最高的 builder 写法");

    // ─────────────────────────────────────────
    println!("\n2、第二层：独立 Builder + build() -> Result");
    // ─────────────────────────────────────────

    // 完整的 HTTP 请求构建
    let req = HttpRequest::builder()
        .method("POST")
        .url("https://api.example.com/users")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token")
        .body(r#"{"name":"alice"}"#)
        .timeout_secs(60)
        .build()
        .expect("必填字段都给齐了，应该不会失败");

    println!("  构建成功：{:?}", req);

    // 校验失败：缺少 url
    let bad = HttpRequest::builder()
        .method("GET")
        .build();

    match bad {
        Ok(_) => println!("  不应该成功"),
        Err(e) => println!("  构建失败 → {}", e),
    }

    // 校验失败：timeout 非法
    let bad2 = HttpRequest::builder()
        .method("GET")
        .url("https://example.com")
        .timeout_secs(0)
        .build();

    match bad2 {
        Ok(_) => println!("  不应该成功"),
        Err(e) => println!("  构建失败 → {}", e),
    }

    println!("  特点：");
    println!("    · Builder 和目标 Config 分离，职责清晰");
    println!("    · 必填字段通过 build() 时的 Result 校验");
    println!("    · header() 可以重复调用，适合可追加字段");
    println!("    · 错误枚举表达各种校验失败，调用方可精确处理");
    println!("  这是 hyper / reqwest / tokio / axum 等库的通用做法");
    println!("小结：生产级库最常见的 builder 形态，表达力与鲁棒性平衡点");

    // ─────────────────────────────────────────
    println!("\n3、第三层：Type-State Builder —— 编译期强制必填");
    // ─────────────────────────────────────────

    // 正确用法：按顺序设置 url 和 method，然后 build()
    let req = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .body("hello")
        .build();

    println!("  构建成功：{:?}", req);

    // 顺序可以随便颠倒，只要最终 url 和 method 都设置了就行
    let req2 = RequestBuilder::new()
        .method("POST")                          // 先 method
        .url("https://api.example.com")          // 再 url
        .build();

    println!("  顺序颠倒也 OK：{:?}", req2);

    // ❌ 下面这种代码在编译期就被拒绝，根本跑不起来：
    //
    // let bad = RequestBuilder::new()
    //     .url("https://example.com")
    //     .build();                             // ❌ method 还没设置，没有 build() 可用
    //
    // 错误信息大致是：
    //   no method named `build` found for struct `RequestBuilder<Set, Empty>` in the current scope
    //
    // 因为 build() 只在 impl RequestBuilder<Set, Set> 里定义
    // 当 MethodState 还是 Empty 时，build() 根本不存在

    println!("  特点：");
    println!("    · 必填字段的「已设置」状态被编码进类型参数");
    println!("    · build() 只在所有必填都已设置的状态下存在");
    println!("    · 缺少必填字段 → 编译错误，不会运行时失败");
    println!("    · 零运行时开销（PhantomData 不占空间）");

    println!("  代价：");
    println!("    · 实现复杂度显著提升，impl 块翻倍");
    println!("    · 泛型参数增加（有几个必填字段就多几个状态）");
    println!("    · 错误信息对新手不太友好");

    println!("  适合场景：");
    println!("    · 必填字段真的「必须有」，漏写可能导致严重问题");
    println!("    · 对外提供库 API，想把「错误可能性」编进类型");
    println!("    · 学习/演示 Rust 类型系统的强大能力");

    println!("小结：Type-State Builder 把「必填检查」从运行时挪到编译时，零开销");

    // ─────────────────────────────────────────
    println!("\n4、三种 builder 对比");
    // ─────────────────────────────────────────

    println!("  {:<30} {:<15} {:<20} {}", "方式", "复杂度", "必填检查", "推荐场景");
    println!("  {:-<30} {:-<15} {:-<20} {:-<30}", "", "", "", "");
    println!("  {:<30} {:<15} {:<20} {}",
        "with_xxx 链式（第一层）", "★☆☆☆☆", "无", "字段全可选、内部工具");
    println!("  {:<30} {:<15} {:<20} {}",
        "独立 Builder + Result", "★★★☆☆", "运行时", "生产库、对外 API");
    println!("  {:<30} {:<15} {:<20} {}",
        "Type-State Builder", "★★★★☆", "编译期", "强一致性关键 API");

    println!("  大多数情况下第二层就够用了，第三层留给真正需要零错误容忍的场景");
    println!("小结：选型核心是「错误检查发生在什么时候」——运行时 vs 编译期");

    // ─────────────────────────────────────────
    println!("\n5、builder 模式的常见扩展");
    // ─────────────────────────────────────────

    println!("  扩展 1：`derive_builder` crate");
    println!("    自动生成独立 Builder，少写大量样板代码");
    println!("    #[derive(Builder)] 标一下就够了");
    println!();
    println!("  扩展 2：返回 &mut Self 代替 Self（引用风格）");
    println!("    不消费实例，适合 Builder 可能被反复修改的场景");
    println!("    let mut b = Xxx::builder();");
    println!("    b.field1(...);");
    println!("    if cond {{ b.field2(...); }}");
    println!("    let x = b.build();");
    println!();
    println!("  扩展 3：异步 build()");
    println!("    在 build() 里做网络查询、读配置、校验等异步工作");
    println!("    签名：async fn build(self) -> Result<T, Error>");
    println!();
    println!("  扩展 4：Fluent API 链式 + method chaining");
    println!("    让调用读起来像自然语言：");
    println!("    query().from(\"users\").where(...).limit(10).build()");
    println!();
    println!("  扩展 5：Builder + Default trait");
    println!("    builder 自身派生 Default，初始化更简洁");
    println!("    #[derive(Default)] struct FooBuilder {{ ... }}");

    println!("小结：Builder 是 Rust 工程实践的高频模式，按需裁剪就好");

    // ─────────────────────────────────────────
    println!("\n【总结】构建器模式要点");
    // ─────────────────────────────────────────
    println!("  · 什么时候需要 Builder？");
    println!("      - 字段多（>5）且有些是可选");
    println!("      - 字段间有合法性约束");
    println!("      - 希望调用方按语义而不是按位置传参");
    println!();
    println!("  · 三层实现方式（按复杂度递增）：");
    println!("      1. with_xxx 链式自消费：简单直接，无校验");
    println!("      2. 独立 Builder + build() -> Result：运行时校验，生产首选");
    println!("      3. Type-State Builder：编译期校验必填，零运行时开销");
    println!();
    println!("  · 关键设计选择：");
    println!("      - 消费 self（mut self）vs 借用（&mut self）");
    println!("      - 字段用 Option<T>（区分「未设置」）vs 用合理默认值");
    println!("      - build() 返回 T vs Result<T, Error>");
    println!();
    println!("  · 生态现状：");
    println!("      - derive_builder、typed-builder 等 crate 可生成样板");
    println!("      - hyper::Request::builder() / tokio::runtime::Builder 是标杆");
    println!();
    println!("  · 核心价值：");
    println!("      让调用处读起来「像在描述需求，而不是在填字段」");
    println!("      把「参数正确性」从文档推到代码，从 runtime 推到 compile-time");
}
