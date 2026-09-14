use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Subcommand)]
pub enum Command {
    /// Add task
    Add {
        /// Title of the task
        #[arg(required = true)]
        title: Vec<String>,

        /// Task priority
        #[arg(short, long, value_enum)]
        priority: Option<Priority>
    },
    /// List tasks
    List {
        /// Which tasks to list
        #[arg(value_enum, default_value_t = ListFilter::All)]
        filter: ListFilter
    },
    /// Complete or uncomplete selected task(s)
    Done {
        /// Indices of selected tasks
        #[arg(required = true)]
        indices: Vec<usize>
    },
    /// Edit selected task
    Edit {
        /// Index of selected task
        index: usize,

        /// New title
        title: Vec<String>,

        /// New priority
        #[arg(short, long, value_enum)]
        priority: Option<Priority>
    },
    /// Remove selected task(s)
    Remove(RemoveTarget)
}

#[derive(Clone, Copy, ValueEnum, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High
}

#[derive(Clone, ValueEnum)]
pub enum ListFilter {
    All,
    Done,
    Undone
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct RemoveTarget {
    /// Indices of selected tasks
    pub indices: Vec<usize>,

    /// Remove all completed tasks
    #[arg(long)]
    pub done: bool,

    /// Remove all tasks
    #[arg(long)]
    pub all: bool,
}

/// Parse the command line arguments and return
/// the selected command (if any).
pub fn parse() -> Option<Command> {
    let cli = Cli::parse();
    cli.command
}