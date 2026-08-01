pub mod parser {
    use clap::{Args, Parser, Subcommand, ValueEnum};

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
    #[derive(Clone, ValueEnum, Debug)]
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
    use crate::parser::Priority;

    pub struct Task {
        title: String,
        completed: bool,
        priority: Priority,
    }

    impl Task {
        fn new() {}

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
    use std::io;

    use crate::task::Task;

    // todo: Instead of Vec<Task> make the function generic.
    //  allow it to take anything seriazible as argument.

    /// Store a vector of tasks to persistant storage.
    pub fn store(tasks: Vec<Task>) -> Result<(), io::Error> {
        Ok(())
    }

    /// Load tasks from persistant storage into a vector.
    pub fn load() -> Result<Vec<Task>, io::Error> {
        Ok(Vec::new())
    }
}

/// Module for main bussies logic
///
/// Performs the desired commands. Creates, manipulates and removes tasks.
/// Uses storage module for persistance and visualizer module for generating
/// output.
pub mod tasker {
    use crate::parser::Commands;

    pub fn add_task() {
        // create new task
        // add task to list
    }

    pub fn list_tasks() {
        // filter tasks
        // visualize tasks
    }

    pub fn toggle_tasks() { }

    pub fn edit_task() { }

    pub fn remove_tasks() { }

    /// Process command
    ///
    /// Load tasks from storage
    /// Based on command transform the tasks
    /// Optionally store result to storage
    pub fn process_command(command: Commands) {}
}
