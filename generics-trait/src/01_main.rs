// 引数に型を指定する必要がある。
// 整数にすると、i32にする必要がある。
// print関数が共通化できそう？

fn main() {
    let message = String::from("Hello, world!");
    print_text(message);
}

fn print_text(text: String) {
    println!("{}", text);
}
