use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 借用规则全景（Borrow Rules Overview）
//
// Rust 的借用检查器（Borrow Checker）在编译期强制执行以下两条铁律：
//
//   规则一：同一时刻，可以存在任意数量的不可变引用（&T）
//           但不能同时存在任何可变引用（&mut T）
//
//   规则二：同一时刻，有且仅有一个可变引用（&mut T）
//           此时不能存在任何其他引用（不管可变还是不可变）
//
// 简记：「多读 XOR 单写」——两种状态互斥，不能共存
//
// NLL（Non-Lexical Lifetimes，非词法生命周期）——Rust 2018 引入：
//   借用的「结束点」是最后一次实际使用该引用的位置
//   而非词法上的花括号 }，这让很多看似冲突的代码合法化
//
// 这两条规则从根本上消灭了「数据竞争」（Data Race）：
//   数据竞争需要同时满足：
//     (a) 两个或以上指针访问同一数据
//     (b) 至少一个指针在写入
//     (c) 没有任何同步机制
//   Rust 的规则让 (a)+(b) 在编译期就不可能同时成立
// ─────────────────────────────────────────────────────────────────────────────

// 用于演示借用的辅助函数：只读取，接受 &str
fn read(label: &str, value: &str) {
    println!("  [{label}] 读取到: {value}");
}

// 接受不可变引用并返回长度，演示借用不影响所有权
fn length_of(s: &str) -> usize {
    s.len() // 借用结束时所有权归还调用方
}

fn main() {
    println!("{}", "=== 借用规则全景 ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、规则一：同一时刻可以有任意数量的不可变引用 &T");
    // ─────────────────────────────────────────

    let document = String::from("Rust 权威指南"); // document 是唯一 owner

    // 同时创建任意数量的不可变引用，全部合法
    let reader1 = &document; // 第一个读者
    let reader2 = &document; // 第二个读者（完全合法）
    let reader3 = &document; // 第三个读者（依然合法）
    let reader4 = &document; // 第四个读者（没有上限）

    // 四个引用可以同时被使用
    println!("  reader1 = {reader1}");
    println!("  reader2 = {reader2}");
    println!("  reader3 = {reader3}");
    println!("  reader4 = {reader4}");

    // 通过函数传递不可变引用，进一步证明多读合法
    read("A", reader1); // reader1 仍有效（&str 是 Copy）
    read("B", reader2);
    let len = length_of(reader3); // reader3 传入函数，函数结束后仍可用
    println!("  通过 reader3 获取长度: {len}");
    println!("  原变量 document 始终有效: {document}");

    // 验证所有引用指向同一内存地址
    println!("  reader1 地址: {:p}", reader1.as_ptr());
    println!("  reader2 地址: {:p}", reader2.as_ptr()); // 与 reader1 相同
    println!("  reader3 地址: {:p}", reader3.as_ptr()); // 与 reader1 相同

    println!("小结：多个 &T 并存是安全的，读-读操作天然不产生竞争");

    // ─────────────────────────────────────────
    println!("\n2、规则二：同一时刻只能有一个 &mut T（独占写）");
    // ─────────────────────────────────────────

    let mut config = String::from("host=localhost");

    // ❌ 错误：同时存在两个 &mut，编译器拒绝
    // let w1 = &mut config;
    // let w2 = &mut config;            // cannot borrow `config` as mutable more than once at a time
    // println!("{w1}, {w2}");          // w1 的借用仍活跃时，w2 无法成立

    // ❌ 错误：即便不同时使用，只要两个 &mut 同时「存在」就不行
    // let w1 = &mut config;
    // let w2 = &mut config;            // 编译器看到 w1 和 w2 同时在作用域内就报错
    // println!("{w2}");                // 即使只用 w2，只要 w1 还没结束就不行

    // ✅ 正确方案一：用 NLL，让第一个 &mut 在最后一次使用后自然结束
    let w1 = &mut config; // 第一个可变引用
    w1.push_str(";port=8080"); // w1 最后一次使用
    println!("  w1 修改后: {w1}"); // ← w1 借用在此结束（NLL）

    let w2 = &mut config; // ✅ w1 已结束，w2 合法
    w2.push_str(";debug=true");
    println!("  w2 修改后: {w2}"); // ← w2 借用在此结束

    // ✅ 正确方案二：用花括号显式限定每个 &mut 的生命周期
    let mut log = String::from("[INFO]");
    {
        let entry1 = &mut log; // 花括号内的可变引用
        entry1.push_str(" 服务启动");
        println!("  entry1: {entry1}");
    } // entry1 在此离开作用域，借用结束
    {
        let entry2 = &mut log; // entry1 已消失，entry2 合法
        entry2.push_str(" 端口监听");
        println!("  entry2: {entry2}");
    }
    println!("  最终 log: {log}");

    println!("小结：写时独占，不允许两个 &mut 同时存在；花括号或 NLL 均可隔离");

    // ─────────────────────────────────────────
    println!("\n3、规则三：&T 与 &mut T 不能同时活跃");
    // ─────────────────────────────────────────

    let mut value = String::from("原始数据");

    // ❌ 错误：不可变引用存活期间不能创建可变引用
    // let immut_ref = &value;          // 不可变引用
    // let mut_ref = &mut value;        // cannot borrow `value` as mutable because it is also borrowed as immutable
    // println!("{immut_ref}");         // immut_ref 仍活跃，mut_ref 非法

    // ❌ 错误：可变引用存活期间不能创建不可变引用
    // let mut_ref = &mut value;        // 可变引用
    // let immut_ref = &value;          // cannot borrow `value` as immutable because it is also borrowed as mutable
    // println!("{mut_ref}, {immut_ref}"); // 两者同时存在，编译器拒绝

    // ❌ 错误：即使先创建不可变、再创建可变，只要不可变还在用就非法
    // let r = &value;                  // 不可变引用 r
    // let m = &mut value;              // 此时 r 还没结束 → 报错
    // println!("{r}");                 // r 在这里用，证明 r 确实还活着

    // ✅ 正确方案一：让不可变引用先「用完」，再创建可变引用（NLL）
    let immut = &value; // 不可变引用开始
    println!("  先读取: {immut}"); // ← immut 最后一次使用，借用在此结束

    let mutable = &mut value; // ✅ immut 已结束，mutable 合法
    mutable.push_str("（已修改）");
    println!("  再修改: {mutable}");

    // ✅ 正确方案二：在独立作用域内隔离
    let mut data = String::from("ABC");
    let snapshot = {
        let r = &data;
        r.len() // 在闭合作用域内用完 r，返回 len 而非引用
    }; // r 在此离开作用域

    // 此时没有任何引用，可以安全地可变借用
    let m = &mut data;
    m.push_str("DEF");
    println!("  snapshot len = {snapshot}, 修改后 data = {m}");

    println!("小结：读写互斥；先结束读借用（NLL或花括号），再开始写借用，反之亦然");

    // ─────────────────────────────────────────
    println!("\n4、NLL 详细演示：借用结束于最后一次「使用」而非花括号");
    // ─────────────────────────────────────────

    // NLL 之前（旧词法生命周期）：借用持续到花括号 }
    // NLL 之后（Rust 2018+）：借用持续到最后一次使用该引用的语句

    let mut sentence = String::from("NLL");

    // 演示一：&T 的 NLL
    let r = &sentence; // 不可变引用开始
    println!("  [NLL 演示] r = {r}"); // ← r 最后一次使用，此后借用结束
    // 在旧 Rust 中，r 的生命周期延伸到函数末尾，下面的 &mut 会报错
    // 在 NLL 下，r 已结束，&mut 完全合法：
    sentence.push_str(" rocks!"); // 直接修改（等价于 (&mut sentence).push_str(...)）
    println!("  修改后: {sentence}");

    // 演示二：&mut T 的 NLL
    let mut nums = vec![3, 1, 4, 1, 5];
    let first = &mut nums[0]; // 可变引用开始
    *first = 100; // 修改第一个元素
    println!("  [NLL] first 修改后: {first}"); // ← first 最后一次使用，借用结束

    // 此后可以整体操作 nums（没有活跃的 &mut 指向其内部）
    nums.sort();
    println!("  排序后 nums: {:?}", nums);

    // 演示三：条件分支中的 NLL
    let mut flag = String::from("off");
    let check = &flag; // 不可变引用
    if check == "off" {
        println!("  flag 为 off"); // ← check 最后一次使用
    }
    // check 在 if 块内已经用完，NLL 分析到此结束
    flag = String::from("on"); // ✅ 可以重新赋值
    println!("  flag 现在是: {flag}");

    // 演示四：循环中的 NLL
    let mut total = 0i32;
    let values = vec![10, 20, 30];
    for v in &values {
        // &values 创建不可变引用（隐式）
        total += v; // v 是 &i32
    } // 循环体内的引用在每次迭代结束时失效
    println!("  [NLL循环] 总和 = {total}");

    println!("小结：NLL 让编译器更精确地追踪借用终点，减少不必要的作用域嵌套");

    // ─────────────────────────────────────────
    println!("\n5、为什么这些规则能消除数据竞争");
    // ─────────────────────────────────────────

    println!("  数据竞争（Data Race）的充要条件（三者同时满足）：");
    println!("    (a) 两个或以上指针「同时」访问同一内存");
    println!("    (b) 至少一个指针正在执行「写」操作");
    println!("    (c) 没有任何「同步机制」协调访问顺序");
    println!();
    println!("  Rust 借用规则如何逐条破解：");
    println!("    → 规则一（多读）：允许 (a) 但排除 (b)，多个读者安全共存");
    println!("    → 规则二（独写）：如果有写操作，同时只有一个指针，排除 (a)+(b) 共存");
    println!("    → 规则三（读写互斥）：有写者时无读者，有读者时无写者，(a)+(b) 不可共存");
    println!("    → 以上规则均在「编译期」静态检查，无需运行时同步原语");
    println!();

    // 用代码直观展示：在 Rust 中，以下「多线程数据竞争场景」
    // 在单线程借用层面的类比——编译器完全拒绝
    let mut shared = vec![1, 2, 3];

    // 模拟"一边遍历（读）一边修改（写）"的竞争场景：
    // ❌ 在很多语言中，迭代时修改集合会导致未定义行为
    // for item in &shared {            // 不可变借用 shared
    //     shared.push(*item * 2);      // cannot borrow `shared` as mutable because it is also borrowed as immutable
    // }

    // ✅ Rust 强迫你先结束读借用，再进行写操作
    let doubled: Vec<i32> = shared.iter().map(|x| x * 2).collect(); // 读：产生新 Vec
    shared.extend(doubled); // 写：读借用已结束，安全扩展
    println!("  安全地先读后写: {:?}", shared);

    println!("小结：借用规则 = 编译期数据竞争检测器；零运行时开销，100% 静态保证");

    // ─────────────────────────────────────────
    println!("\n6、实践技巧：如何组织代码避免借用冲突");
    // ─────────────────────────────────────────

    println!("  {}", "技巧一：先完成读操作，再开始写操作".yellow());
    let mut inventory = vec!["苹果", "香蕉", "橙子"];

    // ❌ 反模式：混合读写意图
    // let first_item = &inventory[0]; // 不可变引用
    // inventory.push("葡萄");         // 可变操作，first_item 还活着 → 报错

    // ✅ 先读完所需信息
    let count = inventory.len(); // 读，但不保留引用
    let has_apple = inventory.contains(&"苹果"); // 读，返回 bool，不保留引用
    // 读操作都通过值（usize/bool）返回，没有遗留的引用
    inventory.push("葡萄"); // 安全写
    println!(
        "  原有 {count} 种，含苹果: {has_apple}，添加后: {:?}",
        inventory
    );

    println!(
        "  {}",
        "技巧二：用 clone 打破借用困境（以内存换便捷）".yellow()
    );
    let mut map: Vec<(String, i32)> = vec![(String::from("a"), 1), (String::from("b"), 2)];
    // 需要读 key 同时修改 value——借用冲突
    // ❌ 直接这么做会产生借用冲突（同时有不可变和可变引用）
    // let key = &map[0].0;            // 不可变引用
    // map[0].1 += 10;                 // 可变操作

    // ✅ clone key，断开与 map 的联系
    let key = map[0].0.clone(); // 复制出 key 的独立副本，与 map 无关
    map[0].1 += 10; // 可以安全修改，因为没有遗留的 map 引用
    println!("  key = {key}, 修改后 map[0].1 = {}", map[0].1);

    println!(
        "  {}",
        "技巧三：用索引代替引用（索引是 Copy，不产生借用）".yellow()
    );
    let mut words = vec!["hello", "world", "rust"];
    // 用索引记录位置，而非引用
    let target_idx = words.iter().position(|&w| w == "world").unwrap();
    words[target_idx] = "Rust"; // 通过索引修改，无借用冲突
    println!("  替换后: {:?}", words);

    println!(
        "  {}",
        "技巧四：将长借用拆分到独立函数中（函数边界天然隔离借用）".yellow()
    );
    let mut text = String::from("  需要处理的文本  ");
    // 把读操作封装在函数中，函数返回后借用自动结束
    let needs_trim = text.trim() != text.as_str(); // trim() 借用在此行结束
    if needs_trim {
        let trimmed = text.trim().to_string(); // trim() 借用在此行结束
        text = trimmed; // 重新赋值，原来的借用已全部结束
    }
    println!("  处理后: '{text}'");

    println!(
        "  {}",
        "技巧五：借用分离（Borrow Splitting）——struct 字段可独立借用".yellow()
    );
    struct Point {
        x: f64,
        y: f64,
    }
    let mut p = Point { x: 1.0, y: 2.0 };
    // 可以同时可变借用不同字段（编译器知道 x 和 y 是不同内存）
    let rx = &mut p.x; // 借用字段 x
    let ry = &mut p.y; // 借用字段 y（不同字段，合法！）
    *rx += 10.0;
    *ry += 20.0;
    println!("  Point 字段独立借用: x={}, y={}", p.x, p.y);

    println!("小结：先读后写、clone 解耦、索引代替引用、函数隔离、字段分离——五大技巧");

    // ─────────────────────────────────────────
    println!("\n【总结】借用规则全景");
    // ─────────────────────────────────────────
    println!("  ┌─────────────────────────────────────────────────────┐");
    println!("  │  规则   │        允许状态        │   编译期保证     │");
    println!("  ├─────────────────────────────────────────────────────┤");
    println!("  │  规则一 │  多个 &T 同时存在      │  读-读无竞争     │");
    println!("  │  规则二 │  仅一个 &mut T 存在    │  写时无其他写者  │");
    println!("  │  规则三 │  &T 与 &mut T 互斥    │  写时无读者       │");
    println!("  └─────────────────────────────────────────────────────┘");
    println!("  NLL：借用结束于「最后一次使用」，而非词法花括号");
    println!("  本质：多读 XOR 单写，两种状态在编译期严格互斥");
    println!("  收益：零运行时开销，彻底消灭数据竞争，内存永远安全");
}
