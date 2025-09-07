fn main() {
    print_message();
    let message = String::from("Hello, world!");
    print_message_by_arg(message);
    // println!("{}", message); messageがmoveされているので、エラーになる

    let message2 = String::from("Hello, world!");
    print_message_by_arg_ref(&message2);
    println!("{}", message2);
}

fn print_message() {
    println!("Hello, world!");
}

fn print_message_by_arg(message: String) {
    println!("{}", message);
}

fn print_message_by_arg_ref(message: &String) {
    println!("{}", message);
}
