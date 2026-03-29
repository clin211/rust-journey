fn main() {
    // 从数组创建切片
    {
        let arr = [1, 2, 3, 4, 5];
        let slice: &[i32] = &arr; // 完整切片
        let slice: &[i32] = &arr[1..3]; // [2, 3]，左闭右开, 从索引 1 到 3（不含）
        let slice: &[i32] = &arr[2..]; // [3, 4, 5]，从索引 2 到末尾，不包含 2
        let slice: &[i32] = &arr[..3]; // [1, 2, 3]，从开头到索引 3（不含）

        println!("第一个元素：{}", slice[0]); // 索引访问

        // 字符串切片 &str 本质上就是对 [u8] 的封装
        let s: &str = "hello";
        let bytes: &[u8] = s.as_bytes(); // 获取底层字节切片
    }

    // 切片作为函数参数
    {
        fn sum(slice: &[i32]) -> i32 {
            slice.iter().sum()
        }

        let arr = [1, 2, 3];
        let vec = vec![4, 5, 6];
        println!("arr 数组所有元素的和 {}", sum(&arr)); // arr 数组所有元素的和 6
        println!("vec 中所有元素的和 {}", sum(&vec)); // vec 中所有元素的和 15
    }

    // &str vs String 大小
    {
        let s: &str = "hello";
        println!("&str 大小：{} 字节", std::mem::size_of_val(&s)); // &str 大小：16 字节

        let s = String::from("hello");
        println!("String 大小：{} 字节", std::mem::size_of_val(&s)); // String 大小：24 字节
        println!("String 容量：{}", s.capacity()); // String 容量：5
    }
}
