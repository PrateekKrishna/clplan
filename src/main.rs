mod init;
use std::path::Path;

fn main() {
    let args = std::env::args().nth(1).expect("NO COMMAND FOUND!");
    // println!("{}", args);

    
    let c = Path::new("db.json").exists();
    if args == "init" && c == false{
        init::init();
    }

    let b = Path::new("db.json").exists();

    if b==true{
        println!("database exists");
    }
    else if b==false{
        // init::init();
        println!("Your first need to initialise app using '-- init' command");
    }
    else if args == "help"{
        println!("--add : To add new task");
        println!("--completed : To change status of existing task");
        println!("--update : To update existing task");
        println!("--delete : To delete task");
    }
    else{
        println!("invalid command. Enter '-- help' to get list of avaiable commands");
    }
}
