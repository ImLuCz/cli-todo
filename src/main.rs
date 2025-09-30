use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "todo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Remove { task: String },
}

#[derive(Serialize, Deserialize)]
struct Todo {
    tasks: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let path = "todo.json";

    // if file exists, the vector is loaded from the json
    let mut todo = if Path::new(path).exists() {
        let data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    } else {
        // else it is created empty
        Todo { tasks: vec![] }
    };

    match cli.command {
        Commands::Add { task } => {
            todo.tasks.push(task);
            fs::write(path, serde_json::to_string_pretty(&todo).unwrap()).unwrap();
            println!("Task added");
        }
        Commands::Remove { task } => {
            // if an index number was passed, remove the task at that index
            if task.parse::<i32>().is_ok() {
                // remove task by index
                todo.tasks.remove(task.parse::<usize>().unwrap() - 1); // -1 because tasks are displayed starting from one
            } else {
                // remove task by value
                todo.tasks.retain(|s| s != &task);
            }
            fs::write(path, serde_json::to_string_pretty(&todo).unwrap()).unwrap();
            println!("Task removed");
        }
        Commands::List => {
            for (i, task) in todo.tasks.iter().enumerate() {
                // prints tasks starting from 1 to be more user-friendly
                println!("{}: {}", i + 1, task);
            }
        }
    }
}
