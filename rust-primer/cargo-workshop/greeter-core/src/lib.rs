pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
        let message = build_greeter(&name);
        assert!(message.contains(name));
    }
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
