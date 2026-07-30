pub mod parser {
    use clap::{Args, Parser, Subcommand, ValueEnum};

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

pub mod visualizer {
    use crate::task::Task;

    pub fn visualize(tasks: Vec<Task>) {}
}

pub mod storage {
    use std::io;

    use crate::task::Task;

    pub fn store(tasks: Vec<Task>) -> Result<(), io::Error> {
        Ok(())
    }

    pub fn load() -> Result<Vec<Task>, io::Error> {
        Ok(Vec::new())
    }
}

pub mod tasker {
    pub fn add_task() {}

    pub fn list_tasks() {}

    pub fn toggle_tasks() {}

    pub fn edit_task() {}

    pub fn remove_tasks() {}
}
