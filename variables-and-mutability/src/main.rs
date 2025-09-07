fn main() {
    // スタックに参照が保存される
    // name変数の値が変わらないなら、String型を使う
    let name = "keigo"
    println!("name is: {}", name);

    // ヒープに参照が保存される
    // name2変数の値が動的に変わっていくなら、String型を使う
    let name2 = String::from("keigo");
    println!("name2 is: {}", name2);
}
