use samples::Button;
use samples::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        todo!();
    }
}

use samples::Screen;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // NOTE: this wont compile becuase we dont implment `Draw` on `String`
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };

    screen.run();
}
