// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//
//     println!("The largest number is {}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//
//     println!("The largest number is {result}");
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
//
// fn main() {
//     let integer = Point { x: 1, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     let mixed = Point { x: 1, y: 4.0 };
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.x());
// }

// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }
//
// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
//
// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };
//
//     let p3 = p1.mixup(p2);
//
//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

//Traits
// use ch10::{NewsArticle, Summary, Tweet};
//
// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebook"),
//         content: String::from("as you already know"),
//         reply: false,
//         retweet: false,
//     };
//
//     println!("1 new tweet: {}", tweet.summarize());
//
//     // let article = NewsArticle {
//     //     headline: String::from("Penguins with the Stanley Cup!"),
//     //     location: String::from("Pitsburgh, PA, USA"),
//     //     author: String::from("Iceburgh"),
//     //     content: String::from(
//     //         "The Pittsburgh Penguins once again are the best \
//     //     hokey team in the NHL.",
//     //     ),
//     // };
//     //
//     // println!("New article available {}", article.summarize());
// }

// Lifetimes
// this does not compile since x goes out of scope by the time it is used
// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     println!("r: {r}");
// }

// Generic lifetimes in functions
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//     let result = longest(string1.as_str(), string2);
//     println!("The longest is: {result}");
// }
//
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// this should fail since string2 goes out of scope
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz ");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("longest string is {result}");
// }
//
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Lifetime annotations in structs
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split(".").next().expect("Could not find a '.'");
//     let _i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// Lifetime Elision - all unctions and structs with references have lifetimes but dont necesarrily
// need explicit lifetime annotations
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }
//
// // explicit annotations
// fn first_wordd<'a> (s: &'a str) -> &'a str {
//     todo!()
// }

// Lifetime annotation in Method Definitions
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
//
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention: {announcement}");
//         self.part
//     }
// }
//
// fn main() {
//     todo!()
// }
//

// Static lifetimes - live for the duration of the program
// let s: &'static str = "I have a static lifetime";

// Putting it all togeter: Generics, Trait Bounds and Lifetimes
use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ana: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ana}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("short");
    let b = String::from("longest");
    let res = longest_with_announcement(a.as_str(), b.as_str(), 45);
    println!("long: {res}");
}
