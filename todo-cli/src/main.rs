use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

const FILE_NAME: &str = "todos.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: xela-todo <command> [task]");
        return;
    }

    match args[1].as_str() {
        "add" => add_task(&args),
        "list" => list_tasks(),
        "clear" => clear_tasks(),
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}

fn add_task(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Please provide a task to add!");
        return;
    }

    let task = &args[2..].join(" ");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_NAME)
        .expect("Could not open file");

    writeln!(file, "{}", task).expect("Could not write to file");
    println!("✅ Task added: {}", task);
}

fn list_tasks() {
    match fs::read_to_string(FILE_NAME) {
        Ok(content) => {
            if content.trim().is_empty() {
                println!("📭 No tasks yet!");
            } else {
                println!("📝 Your tasks:");
                for (i, task) in content.lines().enumerate() {
                    println!("{}: {}", i + 1, task);
                }
            }
        }
        Err(_) => println!("📭 No tasks found!"),
    }
}

fn clear_tasks() {
    fs::write(FILE_NAME, "").expect("Could not clear tasks");
    println!("🧹 All tasks cleared!");
}
