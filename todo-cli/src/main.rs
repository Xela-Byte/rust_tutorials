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
        "clear_task" => clear_task_by_index(&args),
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

fn clear_task_by_index(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Please provide an index to clear!");
        return;
    }

    let index: usize = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid index! Please provide a number.");
            return;
        }
    };

    match fs::read_to_string(FILE_NAME) {
        Ok(content) => {
            if content.trim().is_empty() {
                println!("📭 No tasks to clear!");
                return;
            }

            let tasks: Vec<&str> = content.lines().collect();

            if index == 0 || index > tasks.len() {
                eprintln!(
                    "Invalid index! Please provide a number between 1 and {}",
                    tasks.len()
                );
                return;
            }

            let removed_task = tasks[index - 1];
            let remaining_tasks: Vec<&str> = tasks
                .into_iter()
                .enumerate()
                .filter(|(i, _)| *i != index - 1)
                .map(|(_, task)| task)
                .collect();

            fs::write(FILE_NAME, remaining_tasks.join("\n") + "\n")
                .expect("Could not write to file");

            println!("🗑️  Task removed: {}", removed_task);
        }
        Err(_) => println!("📭 No tasks found!"),
    }
}
