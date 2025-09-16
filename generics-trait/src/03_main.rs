// Displayトレイトを使用する
use std::fmt::Display;

fn main() {
    let message = String::from("Hello, world!");
    let message_number = 42;
    print_text_custom(message);
    print_text_custom(message_number);
}

fn print_text_custom<T: Display>(text: T) {
    println!("{}", text);
}
