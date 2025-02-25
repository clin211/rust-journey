mod mock {
    pub static API: &str = "api.domain.com";
}

static mut API_URL: &str = "https://api.baidu.com";
// println!("API URL:{}", API_URL); // 直接使用会报错，需要在 unsafe 代码块或函数里面使用!!!

fn get_data() {
    unsafe {
        println!("请求数据: {}", API_URL);
        println!("模块的 API URL {}", mock::API); // 获取模块的静态变量
    }
}

// 修改静态变量
fn rest_url() {
    unsafe {
        API_URL = "https://new-api.baidu.com";
        println!("fetch data {}", API_URL);
    }
}

fn main() {
    // 代码块，用于隔离作用域
    // 语法： let 变量名: 类型 = 值;
    let a: i32 = 10;
    println!("a is {}", a);

    let num = 200;
    println!("num is {}", num);

    let num1 = 200.1;
    println!("num2 is {}", num1);

    let name = "rust";
    println!("name is {}", name);

    let is_true = true;
    println!("is_true is {}", is_true);

    let _age = 18; // 如果下文不使用，编译时会报错，可以通过在变量名前面加下划线来消除警告

    // let v: i32;
    // println!("{}", v); // 使用未初始化的变量会报错

    let v;
    v = 200;
    println!("{}", v); // 先声明后初始化，容易使用未初始化的变量，不推荐！！！

    let mut num2 = 200; // 要加上 mut 这个变量才能改变
    println!("before {}", num2);
    num2 = 201; // 编译时这里会报错
    println!("after {}", num2);

    // let num3 = 300;
    // let num3 = 400; // 相同的变量会导致变量遮蔽（后面的覆盖前面的）
    // println!("num3 -> {}", num3); // num3 -> 400

    // 变量解构
    let (a, b) = (200, true);
    println!("a -> {}, b -> {}", a, b); // a -> 200, b -> true

    // 全局静态变量使用 static 关键字生命，需要显示的声明类型
    // 它声明的时候，必须初始化
    // 可变静态变量不是线程安全的，需要在 unsafe 代码块或函数 unsafe函数里面做修改
    // 它初始化的值必须是编译器就确定的值，不能是运行期才确定的值，比如运行期函数表达式的结果
    // 调用 const fn 是可以的，它是编译期执行的

    rest_url();
    get_data();
}
