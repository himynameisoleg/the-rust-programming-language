use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // Error Handling
    // 2 types of errors: Recoverable and Unrecoverable
    // Unrecoverable - panic! macro
    // Recoverable - Result<T, E> enum

    // panic!("crash and burn");
    //run with  RUST_BACKTRACE=1 cargo run to see full stack trace
    // let v = vec![1, 2, 3];
    // v[99];

    // Recoverable Errors with Result
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    // Handling Different Kinds of Errors
    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem: {:?}", e);
    //             }
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // More consise alternative  to the above
    // let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // Shortcuts for Panic on Error: unwrap
    // let _greeting_file = File::open("hello.txt").unwrap();

    // Preffered way to use unwrap is when you are sure the file exists
    // let _greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");

    // Propoating Errors up to calling method
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let username_file_result = File::open("hello.txt");
    //
    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };
    //
    //     let mut username = String::new();
    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }

    // Shortcuts for Propagating Errors: the ? Operator
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }

    // Even shorter way
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut username)?;
    //     Ok(username)
    // }

    // Using ? with Option
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}
