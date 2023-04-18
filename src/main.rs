use clap::{Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs::{write, File};
use std::io::{stdin, BufReader};
use std::path::Path;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    completed: bool,
}

#[derive(Parser)]
#[clap(about = "CLI for creating and managing TODO lists")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List your todo-lists
    List,
    /// Add new tasks for previously created lists
    Add { task_name: String },
    /// Complete tasks
    Complete,
}

const CONFIG_PATH: &str = "/home/beicker/.config/todo-list/config.json";

fn main() {
    let args = Cli::parse();
    let command = args.command;

    match command {
        Some(Commands::Add { task_name }) => add_task(task_name),
        Some(Commands::List) => show_list(),
        Some(Commands::Complete) => complete_task(),
        _ => panic!("Invalid argument"),
    };
}

fn complete_task() {
    if let Ok(reader) = get_config_file() {
        let mut task_list: TaskList = serde_json::from_reader(reader).unwrap();

        for (i, task) in task_list.tasks.iter().enumerate() {
            println!("{}: {:?}, completed: {}", i, task.name, task.completed);
        }

        if let Ok(selected_task_index) = get_selected_task_index() {
            task_list.tasks[selected_task_index].completed = true;
            override_config_file(task_list);
            show_list();
        }
    }
}

fn get_selected_task_index() -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = String::new();
    if let Ok(_) = stdin().read_line(&mut input) {
        let input_string = input.trim().parse::<usize>()?;
        Ok(input_string)
    } else {
        Err("Invalid input".into())
    }
}

fn show_list() {
    if let Ok(reader) = get_config_file() {
        let task_list: TaskList = serde_json::from_reader(reader).unwrap();

        for task in task_list.tasks {
            println!("{:?}", task);
        }
    }
}

fn add_task(task_name: String) {
    if let Ok(reader) = get_config_file() {
        let mut task_list: TaskList = serde_json::from_reader(reader).unwrap();
        let new_task = Task {
            name: task_name,
            completed: false,
        };

        task_list.tasks.push(new_task);

        override_config_file(task_list);
    }
}

fn get_config_file() -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let config_path = CONFIG_PATH.to_string();
    let path = Path::new(&config_path);

    if path.exists() {
        let file = File::open(&config_path).unwrap();
        let reader = BufReader::new(file);
        Ok(reader)
    } else {
        println!("Config file does not exist, creating new one");
        write(&config_path, "{\"tasks\": []}")?;
        if let Ok(file) = File::open(&config_path) {
            println!("Config file created successfully");
            let reader = BufReader::new(file);
            return Ok(reader);
        } else {
            return Err("Could not create config file".into());
        }
    }
}

fn override_config_file(tasks: TaskList) {
    if let Ok(_) = get_config_file() {
        match serde_json::to_string(&tasks) {
            Ok(tasks_json) => {
                if let Ok(_) = write(CONFIG_PATH.to_string(), tasks_json) {
                    println!("Config file updated successfully");
                } else {
                    panic!("Could not write to config file")
                }
            }
            Err(_) => panic!("Could not serialize tasks"),
        }
        return;
    }
    panic!("Could not open config file")
}
