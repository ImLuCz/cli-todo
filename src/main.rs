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
}

#[derive(Serialize, Deserialize)]
struct Todo {
    tasks: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let path = "todo.json";

    let mut todo = if Path::new(path).exists() {
        let data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    } else {
        Todo { tasks: vec![] }
    };

    match cli.command {
        Commands::Add { task } => {
            todo.tasks.push(task);
            fs::write(path, serde_json::to_string_pretty(&todo).unwrap()).unwrap();
            println!("Task added!");
        }
        Commands::List => {
            for (i, task) in todo.tasks.iter().enumerate() {
                println!("{}: {}", i + 1, task);
            }
        }
    }
}
