use std::fs::File;
use std::io;
use std::io::Read;
use serde::{Deserialize, Serialize};
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
	title = title.trim().to_string();
	println!("Desciption");
	io::stdin()
		.read_line(&mut desc)
		.expect("Description of the task is mandatory");
	desc = desc.trim().to_string();
	println!("Due Date");
	io::stdin()
		.read_line(&mut due_date)
		.expect("");
	due_date = due_date.trim().to_string();
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
	std::fs::write("src/db.json", serde_json::to_string_pretty(&new_json).unwrap()).unwrap();
}

pub fn view() {
	let mut file = File::open("src/db.json").expect("Unable to Open file");
	let mut data = String::new();
	file.read_to_string(&mut data).expect("Unable to Read file");
	let v : Response = serde_json::from_str(&data).expect("Unable to read");

	println!("Displaying all Tasks");
	for task in v.tasks {
		println!("ID: {}", task.id);
		println!("Title: {}", task.title);
		println!("Description: {}", task.desc);
		println!("Due Date: {}", task.due_date);
		println!("Status: {}", task.status);
	}
}

pub fn update() {
	view();
	println!("Enter the ID of the task that you want to update");
	let mut input_line = String::new();
	io::stdin()
		.read_line(&mut input_line)
		.expect("Error");
	let id: u64 = input_line.trim().parse().expect("Input not an integer");
	let mut file = File::open("src/db.json").expect("Unable to Open file");
	let mut data = String::new();
	file.read_to_string(&mut data).expect("Unable to Read file");
	let v : Response = serde_json::from_str(&data).expect("Unable to read");

	let mut upd_task = Task {
		id: 0,
		title: "".to_string(),
		desc: "".to_string(),
		due_date: "".to_string(),
		status: false
	};
	let mut task_vec = Vec::new();

	for task in v.tasks {
		if task.id == id {
			println!("Title: {}", task.title);
			let mut title = String::new();
			io::stdin()
				.read_line(&mut title)
				.expect("No Title");
			if !title.trim().is_empty() {
				upd_task.title = title.trim().to_string();
			} else {
				upd_task.title = task.title.trim().to_string();
			}
			println!("Description: {}", task.desc);
			let mut desc = String::new();
			io::stdin()
				.read_line(&mut desc)
				.expect("No Description");
			if !desc.trim().is_empty() {
				upd_task.desc = desc.trim().to_string();
			} else {
				upd_task.desc = task.desc.trim().to_string();
			}
			println!("Due Date: {}", task.due_date);
			let mut due_date = String::new();
			io::stdin()
				.read_line(&mut due_date)
				.expect("No Due Date");
			if !due_date.trim().is_empty() {
				upd_task.due_date = due_date.trim().to_string();
			} else {
				upd_task.due_date = task.due_date.trim().to_string();
			}
			upd_task.id = task.id;
			upd_task.status = task.status;
		} else {
			task_vec.push(task);
		}
	}
	task_vec.push(upd_task);
	let new_json = json!({
		"name": v.name,
		"email": v.email,
		"tasks": task_vec
	});
	std::fs::write("src/db.json", serde_json::to_string_pretty(&new_json).unwrap()).unwrap();
}

pub fn complete(id: u64) {
	let mut file = File::open("src/db.json").expect("Unable to Open file");
	let mut data = String::new();
	file.read_to_string(&mut data).expect("Unable to Read file");
	let v : Response = serde_json::from_str(&data).expect("Unable to read");

	let mut upd_task = Task {
		id: 0,
		title: "".to_string(),
		desc: "".to_string(),
		due_date: "".to_string(),
		status: false
	};
	let mut task_vec = Vec::new();
	for task in v.tasks {
		if task.id == id {
			upd_task.id = id;
			upd_task.title = task.title.trim().to_string();
			upd_task.desc = task.desc.trim().to_string();
			upd_task.due_date = task.due_date.trim().to_string()
			upd_task.status = true;
			task_vec.push(upd_task);
		} else {
			task_vec.push(task);
		}
	}
	let new_json = json!({
		"name": v.name,
		"email": v.email,
		"tasks": task_vec
	});
	std::fs::write("src/db.json", serde_json::to_string_pretty(&new_json).unwrap()).unwrap();
}

pub fn delete(id: u64) {
	let mut file = File::open("src/db.json").expect("Unable to Open file");
	let mut data = String::new();
	file.read_to_string(&mut data).expect("Unable to Read file");
	let v : Response = serde_json::from_str(&data).expect("Unable to read");

	let mut task_vec = Vec::new();
	for task in v.tasks {
		if task.id != id {
			task_vec.push(task);
		}
	}
	let new_json = json!({
		"name": v.name,
		"email": v.email,
		"tasks": task_vec
	});
	std::fs::write("src/db.json", serde_json::to_string_pretty(&new_json).unwrap()).unwrap();
}