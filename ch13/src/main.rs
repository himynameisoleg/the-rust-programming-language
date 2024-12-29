// use std::collections::HashMap;
// use std::thread;
// use std::time::Duration;
//
// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;
//
//     generate_workout(simulated_user_specified_value, simulated_random_number);
// }
//
// struct Catcher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     values: HashMap<u32, u32>,
// }
//
// impl<T> Catcher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Catcher<T> {
//         Catcher {
//             calculation,
//             values: HashMap::new(),
//         }
//     }
//
//     fn value(&mut self, arg: u32) -> u32 {
//         if let Some(&v) = self.values.get(&arg) {
//             v
//         } else {
//             let v = (self.calculation)(arg);
//             self.values.insert(arg, v);
//             v
//         }
//     }
// }
// fn generate_workout(intensity: u32, random_number: u32) {
//     let mut expensive_result = Catcher::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });
//
//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_result.value(intensity));
//         println!("Next, do {} situps!", expensive_result.value(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Stay hydrated!");
//         } else {
//             println!("Today, run for {} mins!", expensive_result.value(intensity));
//         }
//     }
// }
//
// fn simlulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }
//
// #[test]
// fn call_with_different_values() {
//     let mut c = Catcher::new(|a| a);
//
//     let v1 = c.value(1);
//     let v2 = c.value(2);
//
//     assert_eq!(v2, 2);
// }

// fn main() {
//     let v1 = vec![1, 2, 3];
//
//     for i in v1 {
//         println!("{}", i);
//     }
//
//     let v2 = vec![4, 5, 6];
//     let v2_iter = v2.iter();
//     for i in v2_iter {
//         println!("{}", i);
//     }
// }
//
// #[test]
// fn interator_demo() {
//     let v1 = vec![1, 2, 3];
//
//     let mut v1_iter = v1.iter();
//
//     assert_eq!(v1_iter.next(), Some(&1));
//     assert_eq!(v1_iter.next(), Some(&2));
//     assert_eq!(v1_iter.next(), Some(&3));
//     assert_eq!(v1_iter.next(), None);
// }
//
// #[test]
// fn interator_sum() {
//     let v1 = vec![1, 2, 3];
//
//     let mut v1_iter = v1.iter();
//
//     let total: i32 = v1_iter.sum();
//
//     assert_eq!(total, 6);
// }

// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];
//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
//
//     assert_eq!(v2, vec![2, 3, 4]);
// }

 e
