// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.


struct ColorClassicStruct {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

struct ColorTupleStruct(i32, i32, i32);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_classic_struct() {
        let color = ColorClassicStruct {
            red: 255,
            green: 0,
            blue: 0,
        };
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
    }

    #[test]
    fn test_color_tuple_struct() {
        let color = ColorTupleStruct(255, 0, 0);
        assert_eq!(color.0, 255);
        assert_eq!(color.1, 0);
        assert_eq!(color.2, 0);
    }

    #[test]
    fn test_unit_like_struct() {
        let unit = UnitLikeStruct;
        println!("{:?}", unit); // This will print "UnitLikeStruct"
    }
}