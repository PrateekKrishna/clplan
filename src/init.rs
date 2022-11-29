use std::fs::File;
use std::io;

pub fn init() {
    let mut name = String::new();
    let mut email = String::new();

    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Expected your name");
    println!("Hello {} welcome to clplan", name);
    println!("Enter your email: ");
    io::stdin()
        .read_line(&mut email)
        .expect("Expected your email");
    println!("Your email is: {} ", email);
    let _f = File::create("db.json");
}
