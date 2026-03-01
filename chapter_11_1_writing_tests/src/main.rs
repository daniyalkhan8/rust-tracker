pub fn add_numbers(a: u64, b: u64) -> u64 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        let result = add_numbers(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was {result}");
    }

    #[test]
    #[should_panic(expected = "value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn add_test_using_result_enum() -> Result<(), String> {
        let result = add_numbers(2,2);
        if result == 4 {
            Ok(())
        } else {
            Err(format!("Expected value 4 but found {}", result))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
