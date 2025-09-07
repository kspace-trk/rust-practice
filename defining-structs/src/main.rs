struct Profile {
    name: String,
    age: u32
}

impl Profile {
    fn print_profile(&self) {
        println!("name: {}", &self.name);
        println!("age: {}", &self.age);
    }
}

fn main() {
    let user = Profile {
        name: String::from("keigo"),
        age: 26,
    };
    println!("name is: {}", user.name);
    println!("age is : {}", user.age);
    user.print_profile();
}
