use std::collections::HashMap;

fn main() {
    // needs type devinition if empty
    // let v: Vec<i32> = Vec::new();

    // macro for quick init
    // let v = vec![1, 2, 3];

    // add elements to vector
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);

    // read elements
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);
    //
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // panics if out of bounds but get returns None with get()
    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // immutable borrow while reference to element is alive
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // print!("fist element is: {}", first);

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    // for i in &v {
    //     println!("{i}");
    // }

    // use enum to store multiple types
    // enum SPreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    //
    // let row = vec![
    //     SPreadsheetCell::Int(3),
    //     SPreadsheetCell::Text(String::from("blue")),
    //     SPreadsheetCell::Float(10.12),
    // ];

    // Strings
    // let mut s = Stting::new();
    // let data = "initials contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");
    // let mut s = String::from("foo");
    // s.push_str("bar");

    //concatenating 2 existing Strings
    // let s1 = String::from("foo");
    // let s2 = String::from("bar");
    // let s3 = s1 + &s2; // + operators uses the add method which requires a reference to the second string

    // accessing char in string by index is invalid in rust
    // Stored as a byte, scalar or grapheme and O(1) time could not be garunteend
    // let s1 = String::from("hello");
    // let h = s1[0];

    // Dangerous! but possible to access with a slice. (Will crash if tries to slice part of a
    // characters byte)
    // let hello = "hello";
    // let s = &hello[1..3];

    // use chars() and bytes() to iterate over individual element
    // for c in "привіт".chars() {
    //     println!("{c}")
    // }

    // HashMaps
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    //
    // // checking if key exists
    // scores.entry(String::from("Green")).or_insert(50);

    // updating a value based on the old value
    // let text = "hello world wonderful world";
    // let mut map = HashMap::new();
    //
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{:?}", map);
}
