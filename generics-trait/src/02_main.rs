// 文字列と整数のprint関数を共通化する

fn main() {
    let message = String::from("Hello, world!");
    let message_number = 42;
    print_text_custom(message);
    print_text_custom(message_number);
}

fn print_text_custom<T: StringOrInteger>(text: T) {
    text.print_text();
}

trait StringOrInteger {
    fn print_text(&self);
}

// Stringに対するimpl
impl StringOrInteger for String {
    fn print_text(&self) {
        println!("{}", self);
    }
}

// i32に対するimpl
impl StringOrInteger for i32 {
    fn print_text(&self) {
        println!("{}", self);
    }
}
