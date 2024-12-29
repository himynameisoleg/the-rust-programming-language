// use core::slice;
//
// // Unsafe Rust
// fn main() {
//     // let mut num = 5;
//     //
//     // let r1 = &num as *const i32;
//     // let r2 = &mut num as *const i32;
//     //
//     // // raw pointer with arbitrary memory address
//     // let address = 0x12345usize;
//     // let r = address as *const i32;
//     //
//     // unsafe {
//     //     println!("r1 is {}", *r1);
//     //     println!("r2 is: {}", *r2);
//     // }
//     //
//     // // unsafe functions
//     // unsafe fn dangerous() {}
//     //
//     // unsafe {
//     //     dangerous();
//     // }
//     //
//     // safe abstraction around Unsafe
//     let mut v = vec![1, 2, 3, 4, 5, 6];
//
//     let r = &mut v[..];
//
//     let (a, b) = r.split_at_mut(3);
//
//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }
//
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     let ptr = slice.as_mut_ptr();
//
//     assert!(mid <= len);
//
//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
//         )
//     }
// }

// // Using `extern` to call external code
// extern "C" {
//     fn abs(input: i32) -> i32;
// }
//
// fn main() {
//     unsafe {
//         println!("abs value of -3 according to C: {}", abs(-3));
//     }
// }

// Access / Modify a mutable static variable
// static mut COUNTER: u32 = 0;
//
// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }
//
// fn main() {
//     add_to_count(3);
//
//     unsafe {
//         println!("COUNTER: {}", COUNTER);
//     }
// }

// // Fully qualifyed syntax for calling methods with same name
// trait Pilot {
//     fn fly(&self);
// }
//
// trait Wizard {
//     fn fly(&self);
// }
//
// struct Human;
//
// impl Pilot for Human {
//     fn fly(&self) {
//         println!("Pilot");
//     }
// }
//
// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Wizard");
//     }
// }
//
// impl Human {
//     fn fly(&self) {
//         println!("Human");
//     }
// }
//
// fn main() {
//     let person = Human;
//     Pilot::fly(&person);
//     Wizard::fly(&person);
//     person.fly();
// }

// Adanced Function and Closures
// fn add_one(x: i32) -> i32 {
//     x + 1
// }
//
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }
//
// fn main() {
//     let answer = do_twice(add_one, 5);
//     println!("ans: {}", answer);
// }

// Returning a closure
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
