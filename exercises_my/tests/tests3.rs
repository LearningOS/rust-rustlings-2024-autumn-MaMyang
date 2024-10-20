pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 测试偶数，期望结果为 true
        assert!(is_even(4)); // 4 是偶数，应该返回 true
    }

    #[test]
    fn is_false_when_odd() {
        // 测试奇数，期望结果为 false
        assert!(!is_even(5)); // 5 是奇数，应该返回 false
    }
}
