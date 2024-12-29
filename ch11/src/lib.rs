// #[cfg(test)]
// mod tests {
//     #[test]
//     fn exploration() {
//         let result = 2 + 2;
//         assert_eq!(result, 4)
//     }
//
//     #[test]
//     fn another() {
//         panic!("Make ths test fail!");
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//
//         assert!(larger.can_hold(&smaller));
//     }
//
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//
//         assert!(!smaller.can_hold(&larger));
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
// }

// pub fn greeting(name: &str) -> String {
//     // format!("Hello {name}!")
//     String::from("HELLO")
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carl");
//         assert!(
//             result.contains("Carl"),
//             "Greeting did not contain name, value was `{result}`"
//         );
//     }
// }

// pub struct Guess {
//     value: i32,
// }
//
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("Guess value mus be >= 1, got {}", value);
//         } else if value > 100 {
//             panic!("Guess value must be <= 100, got {}", value)
//         }
//
//         Guess { value }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     #[should_panic(expected = "less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal 4"))
//         }
//     }
// }

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn on_hundred() {
        assert_eq!(102, add_two(100));
    }

    // single test with full test name: `cargo test test_two_and_two`
    // multiple tests based on name: `cargo test add`

    #[test]
    #[ignore = "long running test, include with `cargo test -- --ignored`"]
    fn expensive_test() {
        assert_eq!(true, true);
    }
}
