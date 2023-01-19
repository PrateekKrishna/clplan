use std::fs::File;
use std::io;
use serde_json::json;

pub fn help(){
    println!("--add : To add new task");
    println!("--view : To view task");
    println!("--complete : To change status of existing task");
    println!("--update : To update existing task");
    println!("--delete : To delete task");
}

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
    let _f = File::create("src/db.json");
    let new_json = json!({
        "name": name,
        "email": email,
        "tasks": []
    });
    std::fs::write("src/db.json", serde_json::to_string_pretty(&new_json).unwrap()).unwrap();
    println!("Account has been Initialised!");
}
