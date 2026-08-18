mod mock {
    pub static API: &str = "api.domain.com";
}

static mut API_URL: &str = "https://api.baidu.com";

// 刻意在 unsafe 块内读取 mutable static（本节教学点：可变静态变量需 unsafe），
// rustc 2024 edition 的 static_mut_refs lint 会建议改用原子类型/加锁
#[allow(static_mut_refs)]
fn get_data() {
    unsafe {
        println!("请求数据: {}", API_URL);
        println!("模块的 API URL {}", mock::API);
    }
}

#[allow(static_mut_refs)]
fn reset_url() {
    unsafe {
        API_URL = "https://new-api.baidu.com";
        println!("fetch data {}", API_URL);
    }
}

fn main() {
    // 全局静态变量使用 static 关键字声明，需要显式声明类型
    // 它声明的时候，必须初始化
    // 可变静态变量不是线程安全的，需要在 unsafe 代码块或 unsafe 函数里面做修改
    // 它初始化的值必须是编译期就确定的值，不能是运行期才确定的值
    // 调用 const fn 是可以的，它是编译期执行的

    reset_url();
    get_data();

    // 常量使用 const 声明，必须标注类型，命名惯例全大写
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
}
