// mod init;
// mod scripts;
// use std::path::Path;
// use std::io;

// use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    bson::doc,
    sync::Client,
    sync::Collection,
    bson::Document,
    
};

// use crate::models::user_model::User;

// pub struct MongoRepo {
//     col: Collection<User>,
// }


fn init(client: mongodb::sync::Client){
    let db = client.database("clplan");
    let coll = db.collection::<Document>("users");
    coll.insert_one(doc! {"name": "Raj Aryan", "password": "pass123*"}, None);
}


fn main(){
    println!("Hello clplan");
    dotenv().ok();
    let client = Client::with_uri_str("mongodb+srv://clplan:clplan123*@clplan.dbnpcj8.mongodb.net/test").unwrap();
    // let db = client.database("clplan");
    init(client);
    
}

// fn main() {
//     let args = std::env::args().nth(1).expect("OOPS!! NO COMMAND FOUND!");
    
//     let c = Path::new("src/db.json").exists();

//     //check if argument is to initialise clplan, init is only called when db.jsom doesn't exists
//     if args == "init" && !c {
//         init::init();
//     }

//     //if argumet is -- help
//     else if args == "help"{
//         init::help();
//     }



//     let b = Path::new("src/db.json").exists();
//     if b {
//         let mut argument = String::new();

//         println!("you can enter --help to get list of commands");

//         io::stdin().read_line(&mut argument).expect("expected command");
//         let input = argument.trim_end();
        
//         if input == "help" || input == "--help"{
//             init::help();
//         }
//         else if input == "add" || input == "--add" {
//             scripts::add();
//         }
//         else if input == "view" || input == "--view" {
//             scripts::view();
//         }
//         else if input == "update" || input == "--update" {
//             scripts::update();
//         }
//         else if input == "complete" || input == "--complete" {
//             scripts::view();

//             println!("Enter the ID of the task that you want to delete");
//             let mut input_line = String::new();
//             io::stdin()
//                 .read_line(&mut input_line)
//                 .expect("Error");
//             let id: u64 = input_line.trim().parse().expect("Input not an integer");
//             scripts::complete(id);
//         }
//         else if input == "delete" || input == "--delete" {

//             scripts::view();

//             println!("\nEnter the ID of the task that you want to delete");
//             let mut input_line = String::new();
//             io::stdin()
//                 .read_line(&mut input_line)
//                 .expect("Error");
//             let id: u64 = input_line.trim().parse().expect("Input not an integer");
//             scripts::delete(id);
//             println!("successfully deleted task with id: {}", id);
//         }

//     }
//     else if !b {
//         eprintln!("Your first need to initialise app using '-- init' command");
//     }
//     else{
//         println!("invalid command. Enter '-- help' to get list of avaiable commands");
//     }
// }
