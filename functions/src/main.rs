fn main() {
    print_message();
    
    // 例1: 値渡しによる所有権の移動
    let message = String::from("Hello, world!");
    print_message_by_arg(message); // messageの所有権が関数に移動
    // println!("{}", message); // エラー！messageはもう使用できない

    // 例2: 不変参照（借用）- 所有権は移動しない
    let message2 = String::from("Hello, world!");
    print_message_by_arg_ref(&message2); // 借用なので所有権は移動しない
    println!("{}", message2); // OK！message2はまだ使用可能

    // 例3: 可変な値渡し - 所有権が移動する
    let mut message3 = String::from("Hello, world!");
    println!("関数呼び出し前: {}", message3);
    print_message_by_arg_mut(message3.clone()); // message3の所有権が関数に移動するので、cloneする
    println!("{}", message3); // エラー！message3はもう使用できない
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

// 所有権を受け取る版（moveが発生する）
fn print_message_by_arg_mut(mut message: String) {
    message = String::from("hoge");
    println!("{}", message);
}
