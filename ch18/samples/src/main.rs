// fn main() {
// complex if-let, else if, else match
//
// let favorite_color: Option<&str> = None;
// let is_tuesday = false;
// let age: Result<u8, _> = "34".parse();
//
// if let Some(color) = favorite_color {
//     println!("Useing your favorite color, {}, as background", color);
// } else if is_tuesday {
//     println!("Tuesday is green day!");
// } else if let Ok(age) = age {
//     if age > 30 {
//         println!("Using purple as the background color");
//     } else {
//         println!("Using orange as the background color");
//     }
// } else {
//     println!("Using blue as the background color");
// }

// while-let example
// let mut stack = Vec::new();
//
// stack.push(1);
// stack.push(2);
// stack.push(3);
//
// while let Some(top) = stack.pop() {
//     println!("{}", top);
// }

// for loop match
// let v = vec!['a', 'b', 'c'];
//
// for (index, value) in v.iter().enumerate() {
//     println!("{} is an index {}", value, index);
// }

// let point = (3, 5);
// print_coordinates(&point);

//Refutable pattern example
// let Some(x) = some_option_value; // this failes to comlpile
// if let Some(x) = some_option_value {
//     // do something... this coveres the None optoin
// }

// Irrefutable example
// if let x = 5 {
//     //this will fail to compile since this is an Irrefutable match pattern
// }

// Matching Literals
// let x = 1;
//
// match x {
//     1 => println!("one"),
//     2 => println!("two"),
//     _ => println!("any"),
// }

// matching named vars
// let x = Some(5);
// let y = 10;
//
// match x {
//     Some(50) => println!("Got 50"),
//     Some(y) => println!("Matched y = {:?}", y),
//     _ => println!("default case"),
// }
//
// println!("At the end: x={:?}, y={:?}", x, y);

// multiple patterns
// let x = 1;
// match x {
//     1 | 2 => println!("one or two"),
//     3 => println!("three"),
//     _ => println!("default"),
// }

// matching ranges (only for `int` and `char`)
// let x = 5;
// match x {
//     1..=5 => println!("one though five (inclusive)"), // 1...5 was deprecated
//     _ => println!("default"),
// }
// }

// function param matching with tuple
// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("current location: ({}, {})", x, y);
// }

// Destructuring Structs
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// fn main() {
//     let p = Point { x: 0, y: 7 };
//
//     // // let Point { x: a, y: b } = p; // longform
//     // let Point { x, y } = p; // shorthand
//     // assert_eq!(0, x);
//     // assert_eq!(7, y);
//
//     match p {
//         Point { x, y: 0 } => println!("on x axis at {}", x),
//         Point { x: 0, y } => println!("on y axis at {}", y),
//         Point { x, y } => println!("on neither axis: ({} {})", x, y),
//     }
// }

// Destructuring Enums
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msg = Message::ChangeColor(0, 160, 255);
//
//     match msg {
//         Message::Quit => {
//             println!("Quit has no data to destructure")
//         }
//         Message::Move { x, y } => {
//             println!("move x {}, and y {}", x, y);
//         }
//         Message::Write(text) => println!("text message: {}", text),
//         Message::ChangeColor(r, g, b) => {
//             println!("change color to red {}, green {}, blue {}", r, g, b)
//         }
//     }
// }

// Destructuring Nested Structs and Enums
// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }
//
// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
//
//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!("change color to r {}, g {}, b{}", r, g, b)
//         }
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!("change color to h {}, s {}, v {}", h, s, v)
//         }
//         _ => (),
//     }
// }

// Ignoring Values in a Pattern
// fn foo(_: i32, y: i32) {
//     println!("only uses y: {}", y);
// }
//
// fn main() {
//     foo(3, 4);
// }

// Ignoring remaining parts with `..`
// struct Point {
//     x: i32,
//     y: i32,
//     z: i32,
// }
//
// fn main() {
//     let origin = Point { x: 0, y: 0, z: 0 };
//
//     match origin {
//         Point { x, .. } => print!("x is {}", x),
//     }
// }

// fn main() {
//     let numbers = (2, 4, 8, 16, 32);
//
//     match numbers {
//         (first, .., last) => {
//             println!("nums : {} ,{}", first, last);
//         }
//     }
// }

// Match Guards = 'additional if after a pattern'
// fn main() {
//     let num = Some(4);
//
//     match num {
//         Some(x) if x < 5 => println!("less than 5: {}", x),
//         Some(x) => println!("{}", x),
//         None => (),
//     }
// }

// fn main() {
//     let x = Some(10);
//     let y = 10;
//
//     match x {
//         Some(50) => println!("got 50"),
//         Some(n) if n == y => println!("Matched, n {:?}", n),
//         _ => println!("Default x = {:?}", x),
//     }
//
//     println!("at the end x = {:?}, y = {:?}", x, y);
// }

// @ bindings (updatd for deprecated ... syntax)
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
