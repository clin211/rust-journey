use colored::*;

// ─────────────────────────────────────────────────────────────────────────────
// 可变引用 &mut T
//
// 可变引用允许通过引用修改原始数据，但有严格限制：
//
//   · 变量本身必须声明为 mut（let mut x = ...）
//   · 创建可变引用时必须写 &mut（let r = &mut x）
//   · 同一时刻，对同一数据只能存在「唯一」一个可变引用
//   · 可变引用存活期间，不能有任何其他引用（不管可变还是不可变）
//   · &mut T 不实现 Copy，不能被复制（防止出现两个可变访问）
//
// 为什么这么严格？
//   这条规则在编译期消灭了「数据竞争」：
//   数据竞争 = 多个访问者同时存在 + 至少一个在写 + 没有同步机制
//   Rust 直接不允许「写时还有其他人」，问题从根本上不存在
//
// NLL（Non-Lexical Lifetimes，非词法生命周期）：
//   借用的结束点是「最后一次使用」，而非词法作用域的花括号
//   这让很多看似冲突的借用在 NLL 下完全合法
// ─────────────────────────────────────────────────────────────────────────────

// 通过 &mut String 在函数内修改调用方的字符串
fn append_suffix(s: &mut String) {
    s.push_str("_modified"); // push_str 需要可变访问，&mut 提供了写权限
} // 借用在此结束，所有权归还给调用方

// 通过 &mut i32 将值翻倍
fn double_value(n: &mut i32) {
    *n *= 2; // 必须用 * 显式解引用才能修改数值
}

// 通过 &mut Vec<i32> 向 Vec 中追加元素
fn fill_vec(v: &mut Vec<i32>, count: usize) {
    for i in 0..count {
        v.push(i as i32 * 10); // push 需要可变访问，通过 &mut 实现
    }
}

// 通过 &mut Vec<i32> 将所有元素翻倍（原地修改）
fn double_all(v: &mut Vec<i32>) {
    for elem in v.iter_mut() {
        // iter_mut() 返回对每个元素的可变引用
        *elem *= 2; // 解引用后修改元素值
    }
}

fn main() {
    println!("{}", "=== 可变引用 &mut T ===".green().bold());

    // ─────────────────────────────────────────
    println!("\n1、基础用法：变量必须 mut，引用必须 &mut");
    // ─────────────────────────────────────────

    // ❌ 错误：变量没有 mut，无法创建可变引用
    // let immutable = String::from("不可变");
    // let r = &mut immutable;          // cannot borrow `immutable` as mutable, as it is not declared as mutable

    // ❌ 错误：变量有 mut，但引用没写 &mut
    // let mut s = String::from("hello");
    // let r: &mut String = &s;         // mismatched types: expected `&mut String`, found `&String`

    // ✅ 正确：变量有 mut，引用也写 &mut
    let mut text = String::from("hello"); // 变量必须是 mut
    let r = &mut text; // 引用也必须是 &mut
    r.push_str(", world"); // 通过可变引用修改数据
    println!("  修改后: {r}");

    // r 的最后一次使用在上方 println!，借用在此结束（NLL）
    println!("  原变量 text: {text}"); // text 重新可用

    println!("小结：两个 mut 缺一不可，变量不可变则无法借出可变引用");

    // ─────────────────────────────────────────
    println!("\n2、函数通过 &mut T 修改原数据");
    // ─────────────────────────────────────────

    let mut name = String::from("alice"); // name 必须是 mut
    println!("  修改前: {name}");

    append_suffix(&mut name); // 传入可变引用，函数内部修改了 name
    println!("  修改后: {name}"); // name 已被函数修改，且 name 仍是 owner

    let mut count = 5i32;
    println!("  double 前: {count}");
    double_value(&mut count); // 传入 &mut i32，函数内翻倍
    println!("  double 后: {count}"); // count 已被修改

    println!("小结：&mut T 参数让函数拥有写权限，又不夺走调用方的所有权");

    // ─────────────────────────────────────────
    println!("\n3、同一时刻只能有一个 &mut（独占写）");
    // ─────────────────────────────────────────

    let mut data = String::from("独占");

    // ❌ 错误：同时存在两个可变引用，编译器拒绝
    // let r1 = &mut data;
    // let r2 = &mut data;              // cannot borrow `data` as mutable more than once at a time
    // println!("{r1}, {r2}");          // r1 的借用仍活跃时，r2 试图再次可变借用

    // ✅ 正确：同一时刻只创建一个可变引用
    let r1 = &mut data; // 唯一的可变引用，合法
    r1.push_str("！"); // 通过 r1 修改
    println!("  通过 r1 修改后: {r1}");
    // r1 的最后一次使用在上面，借用在此结束（NLL）

    let r2 = &mut data; // r1 已结束，现在可以创建 r2
    r2.push_str("！！");
    println!("  通过 r2 修改后: {r2}");

    println!("小结：独占写保证修改时没有其他人在读或写，彻底消灭数据竞争");

    // ─────────────────────────────────────────
    println!("\n4、花括号作用域：强制串行使用多个 &mut");
    // ─────────────────────────────────────────

    let mut buffer = String::from("start");

    {
        let writer1 = &mut buffer; // writer1 在这个花括号内存活
        writer1.push_str(" -> step1"); // 第一次修改
        println!("  writer1 写入后: {writer1}");
    } // writer1 在此离开作用域，借用结束

    {
        let writer2 = &mut buffer; // writer1 已消失，writer2 可以创建
        writer2.push_str(" -> step2"); // 第二次修改
        println!("  writer2 写入后: {writer2}");
    } // writer2 在此离开作用域

    println!("  最终 buffer: {buffer}"); // buffer 所有修改都已生效

    println!("小结：花括号显式控制借用范围，适合需要清晰边界的场景");

    // ─────────────────────────────────────────
    println!("\n5、NLL：不需要花括号也能串行（借用在最后一次使用后结束）");
    // ─────────────────────────────────────────

    // NLL = Non-Lexical Lifetimes（非词法生命周期）
    // 在 Rust 2018+ 中，借用的结束点由编译器精确分析到「最后一次使用」
    // 而不是简单地等到词法作用域（花括号）结束

    let mut score = 0i32;

    let inc1 = &mut score; // 第一个可变引用
    *inc1 += 10; // 最后一次使用 inc1
    // inc1 的借用在这里结束（NLL 分析）——尽管没有花括号

    let inc2 = &mut score; // ✅ inc1 已结束，可以创建 inc2
    *inc2 += 20; // 最后一次使用 inc2
    // inc2 的借用在这里结束

    let inc3 = &mut score; // ✅ inc2 已结束，可以创建 inc3
    *inc3 += 30;
    println!("  score = {score}"); // 10 + 20 + 30 = 60

    // 对比"旧 Rust"（词法借用）：在旧版本中，下面写法会报错
    // 因为 inc1/inc2 的作用域词法上延伸到函数末尾
    // NLL 让 Rust 更智能地识别借用的真实结束位置

    let mut greeting = String::from("hi");

    let part_a = &mut greeting; // 借用开始
    part_a.push_str(", "); // 使用 part_a
    println!("  part_a: {part_a}"); // ← part_a 最后一次使用，借用结束

    let part_b = &mut greeting; // ✅ NLL 让这里合法
    part_b.push_str("NLL rocks!");
    println!("  part_b: {part_b}");
    println!("最终的 greeting：{greeting}"); // 最终的 greeting：hi, NLL rocks!
    println!("小结：NLL 让借用规则更灵活，只要不「同时」使用就不会冲突");

    // ─────────────────────────────────────────
    println!("\n6、&mut T 不实现 Copy：可变引用不能被复制");
    // ─────────────────────────────────────────

    // 回顾：不可变引用 &T 实现了 Copy，可以随意复制
    let x = 42i32;
    let r_immut = &x;
    let r_copy = r_immut; // ✅ &i32 是 Copy，这里是复制
    println!("  不可变引用复制: r_immut={r_immut}, r_copy={r_copy}"); // 两者都能用

    // 可变引用 &mut T 不实现 Copy：
    let mut y = 42i32;
    let r_mut = &mut y;

    // ❌ 错误：可变引用不能 Copy，赋值会 move（且 move 后原引用失效）
    // let r_mut2 = r_mut;              // r_mut 被 move 到 r_mut2
    // println!("{r_mut}");             // error: use of moved value `r_mut`

    // ❌ 错误：也不能同时借用两次（哪怕是"复制"引用这种形式）
    // let r_mut2 = &mut *r_mut;        // 重新借用：在 r_mut 活跃时又产生第二个 &mut
    // println!("{r_mut}, {r_mut2}");   // 两个可变引用同时存在，编译器拒绝

    // ✅ 正确：直接使用唯一的可变引用
    *r_mut += 100;
    println!("  通过 r_mut 修改: y = {y}");

    println!("  设计原因：如果 &mut T 能 Copy，就能轻易产生两个可变引用，破坏独占性");
    println!("小结：&mut T 不能 Copy，赋值是 move；确保可变引用永远独占");

    // ─────────────────────────────────────────
    println!("\n7、实际应用：Vec<i32> 的可变借用");
    // ─────────────────────────────────────────

    let mut numbers: Vec<i32> = Vec::new(); // 创建空 Vec，必须 mut

    // 场景一：通过函数填充 Vec
    fill_vec(&mut numbers, 5); // 传入 &mut Vec，函数向其中追加元素
    println!("  fill_vec 后: {:?}", numbers); // [0, 10, 20, 30, 40]

    // 场景二：通过函数原地修改所有元素
    double_all(&mut numbers); // 每个元素翻倍
    println!("  double_all 后: {:?}", numbers); // [0, 20, 40, 60, 80]

    // 场景三：直接通过可变引用操作 Vec
    let r_vec = &mut numbers; // 获取可变引用
    r_vec.push(100); // 追加元素
    r_vec.sort(); // 排序（原地）
    println!("  追加并排序后: {:?}", r_vec);

    // 场景四：通过索引修改单个元素
    if let Some(first) = numbers.first_mut() {
        // first_mut() 返回 Option<&mut i32>
        *first = 999; // 修改第一个元素
    }
    println!("  修改首元素后: {:?}", numbers);

    // 场景五：用可变迭代器做筛选式修改（只修改偶数）
    let mut values = vec![1, 2, 3, 4, 5, 6];
    for v in values.iter_mut() {
        // iter_mut() 产生 &mut i32
        if *v % 2 == 0 {
            *v *= 10; // 只把偶数放大 10 倍
        }
    }
    println!("  偶数放大 10 倍: {:?}", values); // [1, 20, 3, 40, 5, 60]

    println!("小结：&mut Vec 是修改集合的标准方式，配合 iter_mut() 可优雅地原地变换元素");

    // ─────────────────────────────────────────
    println!("\n【总结】可变引用 &mut T 的核心要点");
    // ─────────────────────────────────────────
    println!("  · 前提：变量必须 let mut，引用必须 &mut，两者缺一不可");
    println!("  · 独占：同一时刻对同一数据只能有一个 &mut（写时无其他访问）");
    println!("  · 串行：花括号或 NLL 都能让多个 &mut 安全地先后存在");
    println!("  · 非 Copy：&mut T 赋值是 move，不能复制出第二个可变引用");
    println!("  · 解引用：修改值必须用 * 解引用（方法调用时 . 自动解引用）");
    println!("  · 零开销：借用检查在编译期完成，运行时没有任何额外开销");
}
