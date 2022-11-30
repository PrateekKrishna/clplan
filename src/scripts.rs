use std::fs::File;
use std::io;
use std::io::Read;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct Response {
	name: String,
	email: String,
    tasks: Vec<Task>
}

#[derive(Deserialize, Serialize)]
struct Task {
    id: u64,
    title: String,
    desc: String,
    due_date: String,
    status: bool
}


pub fn add() {
	let mut title = String::new();
	let mut desc = String::new();
	let mut due_date = String::new();

	let mut file = File::open("src/db.json").expect("Unable to Open file");
	println!("Title");
	io::stdin()
		.read_line(&mut title)
		.expect("Title is Expected");
	println!("Desciption");
	io::stdin()
		.read_line(&mut desc)
		.expect("Description of the task is mandatory");
	println!("Due Date");
	io::stdin()
		.read_line(&mut due_date)
		.expect("");

	let mut data = String::new();
	file.read_to_string(&mut data).expect("Unable to Read file");
	let v : Response = serde_json::from_str(&data).expect("Unable to read");
	let mut task_vec = Vec::new();
	let mut itr : u64 = 0;
	for task in v.tasks {
		itr += 1;
		task_vec.push(task);
	}
	let new_task = Task {
		id: itr,
		title: title,
		desc: desc,
		due_date: due_date,
		status: false
	};
	task_vec.push(new_task);
	let new_json = json!({
		"name": v.name,
		"email": v.email,
		"tasks": task_vec
	});
	let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &new_json);
    writer.flush();

}