pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn build_greeter(name: &str) -> String {
    let base = format!("hello {} 欢迎来到 Rust 世界", name);

    #[cfg(feature = "loud")]
    {
        base.to_uppercase()
    }

    #[cfg(not(feature = "loud"))]
    {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_build_greeter() {
        let name = "clin";
        let message = build_greeter(name);
        // loud 特性会把名字转大写，这里统一小写比较，两种配置下都成立
        assert!(
            message.to_lowercase().contains(name),
            "greeter message should contain the name, got: {message}"
        );
    }
}
