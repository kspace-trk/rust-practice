fn main() {
    let s1 = String::from("message1");
    hoge(&s1);
    println!("{}", s1);
}

fn hoge(message: &String) {
    println!("message is: {}", message);
}
