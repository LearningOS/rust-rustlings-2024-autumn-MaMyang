// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("This is licensed software.")
    }
}

struct SomeSoftware {}
struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_license() {
        let software = SomeSoftware {};
        assert_eq!(software.licensing_info(), "This is licensed software.");
        let other_software = OtherSoftware {};
        assert_eq!(other_software.licensing_info(), "This is licensed software.");
    }
}