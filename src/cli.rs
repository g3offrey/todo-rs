use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[command(about = "List all todos", name = "ls")]
    List,
    #[command(about = "Add a new todo", name = "add")]
    Add { name: String },
    #[command(about = "Complete a new todo", name = "complete")]
    Complete { name_or_index: String },
    #[command(about = "Remove a todo", name = "rm")]
    Remove { name_or_index: String },
}