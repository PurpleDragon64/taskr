pub mod parser {
    use clap::{Args, Parser, Subcommand, ValueEnum};
    use serde::{Serialize, Deserialize};

    // todo: move public types to separate module?

    #[derive(Parser)]
    #[command(about)]
    struct Cli {
        #[command(subcommand)]
        command: Option<Commands>
    }

    #[derive(Subcommand)]
    pub enum Commands {
        /// Add task
        Add {
            /// Title of the task
            #[arg(required = true)]
            title: Vec<String>,

            /// Task priority
            #[arg(long, value_enum)]
            priority: Option<Priority>
        },
        /// List tasks
        List {
            /// Which tasks to list
            #[arg(value_enum, default_value_t = ListFilter::All)]
            filter: ListFilter
        },
        /// Complete or uncomplete selected tasks
        Done {
            /// Indices of selected tasks
            indices: Vec<u32>
        },
        /// Edit selected task
        Edit {
            /// Index of selected task
            index: u32,

            /// New title
            title: Vec<String>,

            /// New priority
            #[arg(long, value_enum)]
            priority: Option<Priority>
        },
        /// Remove selected task(s)
        Remove(RemoveTarget)
    }

    // Remove Debug?
    #[derive(Clone, ValueEnum, Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub enum Priority {
        Low,
        Medium,
        High
    }

    // Remove Debug?
    #[derive(Clone, ValueEnum, Debug)]
    pub enum ListFilter {
        All,
        Done,
        Undone
    }

    #[derive(Args)]
    #[group(required = true, multiple = false)]
    pub struct RemoveTarget {
        /// Indices of selected tasks
        pub indices: Vec<u32>,

        /// Remove all completed tasks
        #[arg(long)]
        pub done: bool,

        /// Remove all tasks
        #[arg(long)]
        pub all: bool,
    }

    pub fn parse() -> Option<Commands> {
        let cli = Cli::parse();
        cli.command
    }

}

pub mod task {
    use serde::{Serialize, Deserialize};

    use crate::parser::Priority;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    pub struct Task {
        title: String,
        completed: bool,
        priority: Priority,
    }

    impl Task {
        pub fn new(title: String, priority: Priority) -> Task {
            Task {completed: false, title, priority }
        }

        // accessor or public field
        fn toggle_completed() {}

        // accessor or public field
        fn edit() {}
    }
}
/// Module for visualizing tasks
///
/// Handles formatting and printing
pub mod visualizer {
    use crate::task::Task;

    pub fn visualize(tasks: Vec<Task>) {}
}

/// Module for interacting with persistant storage.
///
/// Handles serialization, deserialization and writing to file and
/// reading from file.
pub mod storage {
    use std::fs;
    use std::io;

    use crate::task::Task;

    const FILE_NAME: &str = "storage";

    // todo: Instead of Vec<Task> make the function generic.
    //  allow it to take anything seriazible as argument.

    /// Store a vector of tasks to persistant storage.
    /// The previous contents of the storage will be overwritten.
    pub fn store(tasks: Vec<Task>) -> Result<(), io::Error> {
        let serialized = serde_json::to_vec(&tasks)?;
        fs::write(FILE_NAME, serialized)
    }

    /// Load tasks from persistant storage into a vector.
    pub fn load() -> Result<Vec<Task>, io::Error> {
        if fs::exists(FILE_NAME)? {
            let serialized = fs::read(FILE_NAME)?;
            let tasks: Vec<Task> = serde_json::from_slice(&serialized)?;
            Ok(tasks)
        } else {
            Ok(Vec::new())
        }
    }
}

/// Module for main bussies logic
///
/// Performs the desired commands. Creates, manipulates and removes tasks.
/// Uses storage module for persistance and visualizer module for generating
/// output.
pub mod taskr {
    use crate::parser::Commands;

    fn add_task() {
        // create new task
        // add task to list
    }

    fn list_tasks() {
        // filter tasks
        // visualize tasks
    }

    fn toggle_tasks() { }

    fn edit_task() { }

    fn remove_tasks() { }

    /// Process command
    ///
    /// Load tasks from storage
    /// Based on command transform the tasks
    /// Optionally store result to storage
    pub fn process_command(command: Option<Commands>) {

        // load tasks
        // handle errors from manipulating storage

        // only placeholder prints
        // todo: replace with real implementation
        match command {
            Some(Commands::Add {title, priority}) => {
                let prio_mess = if let Some(prio) = priority {
                    format!(" with priority {:?}", prio)
                } else {
                    "".to_string()
                };
                println!("Adding: {}{}", title.join(" "), prio_mess)
            },
            Some(Commands::List {filter }) => {
                println!("Listing {:?} tasks (list command)", filter)
            },
            Some(Commands::Done { indices }) => {
                println!("Completing {:?} ", indices)
            },
            Some(Commands::Edit { index, title, priority }) => {
                let title_mess = if !title.is_empty() {
                    format!(" new title: {}", title.join(" "))
                } else {
                    "".to_string()
                };
                let prio_mess = if let Some(prio) = priority {
                    format!(" new priority: {:?}", prio)
                } else {
                    "".to_string()
                };
                println!("Editing task {}:{}{}", index, title_mess, prio_mess)
            },
            Some(Commands::Remove(target)) => {
                if target.all {
                    println!("Removing all tasks");
                } else if target.done {
                    println!("Removing all completed tasks");
                } else {
                    println!("Removing tasks {:?}", target.indices);
                }
            }
            None => println!("Listing all tasks (no command)")
        }
    }
}

pub use taskr::process_command;