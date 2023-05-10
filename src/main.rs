use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::todos::Todos;

mod todo;
mod todos;
mod cli;


fn main() -> Result<(), Box<dyn Error>> {
    let mut todos = Todos::new();
    todos.import().expect("Can't import todos");

    let matches = Cli::parse();

    match matches.command {
        Commands::List => {}
        Commands::Add { name } => {
            todos.add_todo(&name);
            todos.export().unwrap();
        }
        Commands::Complete { name_or_index } => {
            // TODO find a more robust way to know if index or name
            if let Ok(index) = name_or_index.parse::<usize>() {
                todos.complete_by_index(index)?;
            } else {
                todos.complete_by_name(&name_or_index);
            }
        }
        Commands::Remove { name_or_index } => {
            // TODO find a more robust way to know if index or name
            if let Ok(index) = name_or_index.parse::<usize>() {
                todos.delete_todo_by_index(index)?;
            } else {
                todos.delete_todo_by_name(&name_or_index);
            }
        }
    }

    todos.display_in_console();
    todos.export().expect("Can't export todos");

    Ok(())
}

