// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    let wrapped_integer: Wrapper<i32> = Wrapper::new(42);
    let wrapped_string: Wrapper<String> = Wrapper::new(String::from("Hello, world!"));

    println!("Wrapped integer: {}", wrapped_integer.value);
    println!("Wrapped string: {}", wrapped_string.value);
}