// use std::fmt::{Debug, Display};
//
// pub trait Summary {
//     fn summarize_author(&self) -> String;
//
//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }
//
// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }
//
// // impl Summary for NewsArticle {
// //     // fn summarize(&self) -> String {
// //     //     format!("{}, by {} ({})", self.headline, self.author, self.location)
// //     // }
// // }
//
// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }
//
// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }
//
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize())
// }
// // the above is syntactic sugar for this:
// pub fn notifyy<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize())
// }
//
// // specifying multiple trait bounds with +
// pub fn notifyyy(item: &(impl Summary + Display)) {
//     todo!()
// }
// // or alternatively
// pub fn notifyyyy<T: Summary + Display>(item: &T) {
//     todo!()
// }
//
// // alternate syntax for when multiple trait bounds start getting out of hand:
// fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     todo!()
// }
// // this is more clear (i guess? 🤨)
// fn some_fnn<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     todo!()
// }
//
// // Retunrning types that implement traits
// fn returns_summerizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebook"),
//         content: String::from("as you already know"),
//         reply: false,
//         retweet: false,
//     }
// }
//
// // Trait Bounds to conditionally implement methods
// struct Pair<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }
//
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y ={}", self.y);
//         }
//     }
// }
