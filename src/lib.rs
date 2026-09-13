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
            indices: Vec<usize>
        },
        /// Edit selected task
        Edit {
            /// Index of selected task
            index: usize,

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
    #[derive(Clone, Copy, ValueEnum, Serialize, Deserialize, Debug, PartialEq, Eq)]
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
        pub indices: Vec<usize>,

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
    use std::fmt::Display;
    use serde::{Serialize, Deserialize};

    use crate::parser::Priority;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
    pub struct Task {
        pub title: String,
        completed: bool, // not pub, but exposed via accessors
        pub priority: Priority,
    }

    impl Task {
        pub fn new(title: String, completed: bool, priority: Priority) -> Task {
            Task {title, completed, priority}
        }

        pub fn toggle_completed(&mut self) {
            self.completed = !self.completed;
        }

        pub fn is_completed(&self) -> bool {
            self.completed
        }
    }

    impl Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let done_str = if self.is_completed() {"[x]"} else {"[ ]"};
            let title_str = &self.title;
            let prio_str = match self.priority {
                Priority::Low => "(!)",
                Priority::Medium => "(!!)",
                Priority::High => "(!!!)"
            };
            write!(f, "{done_str} {title_str} {prio_str}")
        }
    }
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

    // todo: change FILE_NAME to make module more testable

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

/// Module for main busiess logic
///
/// Performs the desired commands. Creates, manipulates and removes tasks.
/// Uses storage module for persistance.
pub mod taskr {
    use std::fmt;
    use crate::{
        parser::{Commands, ListFilter, Priority, RemoveTarget},
        task::Task,
    };

    /// Custom error type for functions which take indices as parameters
    #[derive(Debug, Clone)]
    struct IndexOutOfRangeError {
        index: usize
    }

    impl fmt::Display for IndexOutOfRangeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "index {} is out of range", self.index)
        }
    }

    // todo: change Vec types to slice types

    // Add a new uncompleted task with title *title* and priority *prio* to *tasks*.
    fn add_task(tasks: &mut Vec<Task>, title: String, prio: Priority) {
        let task = Task::new(title, false, prio);
        tasks.push(task);
    }

    // // alt 1
    // fn list_tasks(tasks: &Vec<Task>, filter: ListFilter) {

    //     for (index, task) in tasks.iter().enumerate() {
    //         if match filter {
    //             ListFilter::All => false,
    //             ListFilter::Done => !task.is_completed(),
    //             ListFilter::Undone => task.is_completed(),
    //         } {
    //             continue;
    //         }
    //         let idx_str = index.to_string();
    //         println!("{idx_str} {task}");
    //     }
    // }

    // alt 2
    // - match logic done on each task inside the single closure
    // fn list_tasks(tasks: &Vec<Task>, filter: ListFilter) {
    //     let matches = |task: &Task| match filter {
    //         ListFilter::All => true,
    //         ListFilter::Done => task.is_completed(),
    //         ListFilter::Undone => !task.is_completed(),
    //     };

    //     for (index, task) in tasks.iter().enumerate().filter(|(_,t)| matches(t)) {
    //         println!("{index} {task}");
    //     }
    // }

    // alt 3
    // - match logic done once to build a specific closure
    // - the type of each match arms must be the same. in rust, however,
    //   each closure has a unique anonymout type. These concrete closures, however,
    //   can be coerced into a function pointer type (fn(...) -> ...), because they
    //   do not capture anything from their environment.

    // Print tasks selected from *tasks* by *filter* to stdout.
    fn list_tasks(tasks: &Vec<Task>, filter: ListFilter) {
        let predicate: fn(&Task) -> bool = match filter {
            ListFilter::All => |_| true,
            ListFilter::Done => |t| t.is_completed(),
            ListFilter::Undone => |t| !t.is_completed(),
        };

        for (index, task) in tasks.iter().enumerate().filter(|(_, t)| predicate(t)) {
            println!("{index} {task}");
        }
    }

    // alt 4
    // - using iterator chain and for_each method
    // fn list_tasks(tasks: &Vec<Task>, filter: ListFilter) {
    //     let predicate: fn(&Task) -> bool = match filter {
    //         ListFilter::All => |_| true,
    //         ListFilter::Done => |t| t.is_completed(),
    //         ListFilter::Undone => |t| !t.is_completed(),
    //     };

    //     tasks
    //         .iter()
    //         .enumerate()
    //         .filter(|(_, task)| match filter {
    //             ListFilter::All => true,
    //             ListFilter::Done => task.is_completed(),
    //             ListFilter::Undone => !task.is_completed()
    //         })
    //         .for_each(|(index, task)| println!("{index} {task}"))
    // }

    // Toggle the completed property of all tasks specified by *indices*.
    fn toggle_tasks(tasks: &mut Vec<Task>, indices: Vec<usize>) -> Result<(), IndexOutOfRangeError> {
        for index in indices {
            let Some(task) = tasks.get_mut(index) else {
                return Err(IndexOutOfRangeError { index });
            };
            task.toggle_completed();
        }
        Ok(())
    }

    // Change the title and/or priority of task selected by *index* to the given values.
    fn edit_task(tasks: &mut Vec<Task>, index: usize, title: Option<String>, prio: Option<Priority>) -> Result<(), IndexOutOfRangeError> {
        let Some(task) = tasks.get_mut(index) else {
            return Err(IndexOutOfRangeError { index });
        };
        if let Some(new_title) = title {
            task.title = new_title;
        }
        if let Some(new_prio) = prio {
            task.priority = new_prio;
        }
        Ok(())
    }

    // Remove tasks specified by *target* from *tasks*.
    fn remove_tasks(tasks: &mut Vec<Task>, target: &RemoveTarget) -> Result<(), IndexOutOfRangeError> {
        if target.all {
            tasks.clear();
            return Ok(());
        }
        if !target.indices.is_empty() {
            let mut idxs = target.indices.clone();
            idxs.sort_unstable_by(|a, b| b.cmp(a));
            idxs.dedup();
            let largest = idxs[0];
            if largest >= tasks.len() {
                return Err(IndexOutOfRangeError { index: largest });
            }
            for idx in idxs {
                tasks.remove(idx);
            }
        }
        if target.done {
            tasks.retain(|t| !t.is_completed());
        }
        Ok(())
    }

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

    #[cfg(test)]
    mod tests {

    use super::*;

        #[test]
        fn test_add_to_empty() {
            let mut tasks: Vec<Task> = Vec::new();
            let title = "title".to_string();
            let prio = Priority::Medium;

            add_task(&mut tasks, title.clone(), prio.clone());

            assert_eq!(tasks.len(), 1);
            let task = &tasks[0];
            assert_eq!(task.title, title);
            assert!(!task.is_completed());
            assert_eq!(task.priority, prio);
        }

        #[test]
        fn test_add_task() {
            let first = Task::new("first".to_string(), false, Priority::Low);
            let mut tasks = vec![first];
            let title = "second".to_string();
            let prio = Priority::High;

            add_task(&mut tasks, title.clone(), prio.clone());

            assert_eq!(tasks.len(), 2);
            let task = tasks
                .iter()
                .find(|t| t.title == title);
            assert!(task.is_some());
            assert!(!task.unwrap().is_completed());
            assert_eq!(task.unwrap().priority, Priority::High);
        }

        #[test]
        fn test_list_all_tasks() {
            let t1 = Task::new("one".to_string(), true, Priority::Low);
            let t2 = Task::new("two".to_string(), false, Priority::Medium);
            let t3 = Task::new("three".to_string(), true, Priority::High);
            let tasks = vec![t1, t2, t3];

            let filter = ListFilter::All;
            list_tasks(&tasks, filter);

            // note: this test does not assert anything, one must check the output manually in the terminal
        }

        #[test]
        fn test_list_done_tasks() {
            let t1 = Task::new("one".to_string(), true, Priority::Low);
            let t2 = Task::new("two".to_string(), false, Priority::Medium);
            let t3 = Task::new("three".to_string(), true, Priority::High);
            let tasks = vec![t1, t2, t3];

            let filter = ListFilter::Done;
            list_tasks(&tasks, filter);

            // note: this test does not assert anything, one must check the output manually in the terminal

        }

        #[test]
        fn test_list_undone_tasks() {
            let t1 = Task::new("one".to_string(), true, Priority::Low);
            let t2 = Task::new("two".to_string(), false, Priority::Medium);
            let t3 = Task::new("three".to_string(), true, Priority::High);
            let tasks = vec![t1, t2, t3];

            let filter = ListFilter::Undone;
            list_tasks(&tasks, filter);

            // note: this test does not assert anything, one must check the output manually in the terminal

        }

        #[test]
        fn test_complete_task() {
            let t = Task::new("title".to_string(), false, Priority::Medium);
            let mut tasks = vec![t];

            let res = toggle_tasks(&mut tasks, vec![0]);

            assert!(res.is_ok());
            assert!(tasks[0].is_completed())
        }

        #[test]
        fn test_uncomplete_task() {
            let t = Task::new("title".to_string(), true, Priority::Medium);
            let mut tasks = vec![t];

            let res = toggle_tasks(&mut tasks, vec![0]);

            assert!(res.is_ok());
            assert!(!tasks[0].is_completed())
        }

        #[test]
        fn test_complete_multiple_tasks() {
            let t1 = Task::new("one".to_string(), false, Priority::Low);
            let t2 = Task::new("two".to_string(), false, Priority::Medium);
            let t3 = Task::new("three".to_string(), false, Priority::High);
            let mut tasks = vec![t1, t2, t3];

            let res = toggle_tasks(&mut tasks, vec![0, 2]);

            assert!(res.is_ok());
            assert!(tasks[0].is_completed());
            assert!(!tasks[1].is_completed());
            assert!(tasks[2].is_completed());
        }

        #[test]
        fn test_complete_out_of_range() {
            let task = Task::new("title".to_string(), false, Priority::Medium);
            let mut tasks = vec![task];

            let res = toggle_tasks(&mut tasks, vec![1]);

            assert!(res.is_err_and(|e| e.index == 1));
        }

        #[test]
        fn test_edit_title() {
            let old_title = "old title".to_string();
            let old_priority = Priority::Medium;
            let task = Task::new(old_title.clone(), false, old_priority);
            let mut tasks = vec![task];

            let new_title = "new title".to_string();
            let res = edit_task(&mut tasks, 0, Some(new_title.clone()), None);

            assert!(res.is_ok());
            assert_eq!(tasks[0].title, new_title);
            assert_eq!(tasks[0].priority, old_priority);
        }

        #[test]
        fn test_edit_priority() {
            let old_title = "title".to_string();
            let old_priority = Priority::Low;
            let task = Task::new(old_title.clone(), false, old_priority);
            let mut tasks = vec![task];

            let new_priority = Priority::High;
            let res = edit_task(&mut tasks, 0, None, Some(new_priority));

            assert!(res.is_ok());
            assert_eq!(tasks[0].priority, new_priority);
            assert_eq!(tasks[0].title, old_title);
        }

        #[test]
        fn test_edit_title_and_priority() {
            let old_title = "old title".to_string();
            let old_priority = Priority::Low;
            let task = Task::new(old_title.clone(), false, old_priority);
            let mut tasks = vec![task];

            let new_title = "new title".to_string();
            let new_priority = Priority::High;
            let res = edit_task(&mut tasks, 0, Some(new_title.clone()), Some(new_priority));

            assert!(res.is_ok());
            assert_eq!(tasks[0].title, new_title);
            assert_eq!(tasks[0].priority, new_priority);
        }

        #[test]
        fn test_edit_out_of_range() {
            let task = Task::new("title".to_string(), true, Priority::Medium);
            let mut tasks = vec![task];

            let res = edit_task(&mut tasks, 2, Some("new title".to_string()), Some(Priority::Low));

            assert!(res.is_err_and(|e| e.index == 2));
        }

        #[test]
        fn test_remove_all_tasks() {
            let t1 = Task::new("first".to_string(), false, Priority::Low);
            let t2 = Task::new("second".to_string(), true, Priority::High);
            let mut tasks = vec![t1, t2];

            let target = RemoveTarget { indices: Vec::new(), done: false, all: true };
            let res = remove_tasks(&mut tasks, &target);

            assert!(res.is_ok());
            assert!(tasks.is_empty());
        }

        #[test]
        fn test_remove_completed_tasks() {
            let uncompleted = Task::new("uncompleted".to_string(), false, Priority::Medium);
            let completed = Task::new("completed".to_string(), true, Priority::Medium);
            let mut tasks = vec![uncompleted.clone(), completed.clone()];

            let target = RemoveTarget { indices: Vec::new(), done: true, all: false };
            let res = remove_tasks(&mut tasks, &target);

            assert!(res.is_ok());
            assert!(tasks.contains(&uncompleted));
            assert!(!tasks.contains(&completed));
        }

        #[test]
        fn test_remove_tasks_by_index() {
            let t1 = Task::new("one".to_string(), true, Priority::Low);
            let t2 = Task::new("two".to_string(), true, Priority::Medium);
            let t3 = Task::new("three".to_string(), false, Priority::High);
            let t4 = Task::new("four".to_string(), false, Priority::High);
            let mut tasks = vec![t1.clone(), t2.clone(), t3.clone(), t4.clone()];

            let target = RemoveTarget { indices: vec![0, 3, 2, 2, 0], done: false, all: false};
            let res = remove_tasks(&mut tasks, &target);

            assert!(res.is_ok());
            assert!(!tasks.contains(&t1));
            assert!(tasks.contains(&t2));
            assert!(!tasks.contains(&t3));
            assert!(!tasks.contains(&t4));
        }

        #[test]
        fn test_remove_by_idx_and_done() {
            let t1 = Task::new("one".to_string(), false, Priority::Low);
            let t2 = Task::new("two".to_string(), true, Priority::Medium);
            let t3 = Task::new("three".to_string(), false, Priority::High);
            let mut tasks = vec![t1.clone(), t2.clone(), t3.clone()];

            let target = RemoveTarget { indices: vec![0], done: true, all: false };
            let res = remove_tasks(&mut tasks, &target);

            assert!(res.is_ok());
            assert!(!tasks.contains(&t1));
            assert!(!tasks.contains(&t2));
            assert!(tasks.contains(&t3));
        }

        #[test]
        fn test_remove_preserves_order() {
            let t1 = Task::new("one".to_string(), false, Priority::Low);
            let t2 = Task::new("two".to_string(), false, Priority::Low);
            let t3 = Task::new("three".to_string(), false, Priority::Low);
            let mut tasks = vec![t1.clone(), t2.clone(), t3.clone()];

            let target = RemoveTarget { indices: vec![0], done: false, all: false };
            let res = remove_tasks(&mut tasks, &target);

            assert!(res.is_ok());
            assert_eq!(tasks[0], t2);
            assert_eq!(tasks[1], t3);
        }

        #[test]
        fn test_remove_out_of_range_error() {
            let task = Task::new("title".to_string(), false, Priority::Medium);
            let mut tasks = vec![task];

            let target = RemoveTarget { indices: vec![1], done: false, all: false };
            let res = remove_tasks(&mut tasks, &target);

            assert!(res.is_err_and(|e| e.index == 1));
        }
    }
}

pub use taskr::process_command;