// 使用泛型使 Wrapper 支持任何类型的值
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    // 使用泛型的 new 方法
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
