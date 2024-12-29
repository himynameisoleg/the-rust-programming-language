// Documenting lib code.
// Run with `cargo doc --open`
//
// //! # Sample Crate
// //!
// //! `samples` is a collection of utilities to make ...
// //! this is a style of comment that describes the contained item,
// //! rather than the thing just below the comment.
//
// /// Adds one to number given.
// ///
// /// # Example
// ///
// /// ```
// /// let arg = 5;
// /// let answer = samples::add_one(arg);
// ///
// /// assert_eq!(6, answer);
// /// ```
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }

//! # Art
//!
//! A library for modeling artistic concepts

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// Th primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in qual amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        todo!();
    }
}
