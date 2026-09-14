/// Module for main busiess logic
///
/// Performs the desired commands. Creates, manipulates and removes tasks.
/// Uses storage module for persistance.

use std::fmt;
use std::io;

use crate::{
    parser::{Command, Priority, ListFilter, RemoveTarget},
    storage,
    task::Task,
};

/// Custom error type for functions which take indices as parameters.
#[derive(Debug, Clone)]
pub struct IndexOutOfRangeError {
    index: usize
}

impl fmt::Display for IndexOutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "index {} is out of range", self.index)
    }
}

/// Custom error type for the process_command function.
pub enum TaskrError {
    Storage(io::Error),
    Index(IndexOutOfRangeError),
}

impl fmt::Display for TaskrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Storage(e) => format!("Interacting with the storage failed.\n{e}"),
            Self::Index(e) => e.to_string(),
        };
        write!(f, "{message}")
    }
}

/// Add a new uncompleted task with title *title* and priority *prio* to *tasks*.
fn add_task(tasks: &mut Vec<Task>, title: String, prio: Priority) {
    let task = Task::new(title, false, prio);
    tasks.push(task);
}

/// Print tasks selected from *tasks* by *filter* to stdout.
fn list_tasks(tasks: &[Task], filter: ListFilter) {
    println!("========== Tasks ==========");

    let predicate: fn(&Task) -> bool = match filter {
        ListFilter::All => |_| true,
        ListFilter::Done => |t| t.is_completed(),
        ListFilter::Todo => |t| !t.is_completed(),
    };
    for (index, task) in tasks.iter().enumerate().filter(|(_, t)| predicate(t)) {
        println!("{index} {task}");
    }
}

/// Toggle the completed property of all tasks specified by *indices*.
fn toggle_tasks(tasks: &mut [Task], indices: &[usize]) -> Result<(), IndexOutOfRangeError> {
    for &index in indices {
        let Some(task) = tasks.get_mut(index) else {
            return Err(IndexOutOfRangeError { index });
        };
        task.toggle_completed();
    }
    Ok(())
}

/// Change the title and/or priority of task selected by *index* to the given values.
fn edit_task(tasks: &mut [Task], index: usize, title: Option<String>, prio: Option<Priority>) -> Result<(), IndexOutOfRangeError> {
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

/// Remove tasks specified by *target* from *tasks*.
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
/// Load tasks from storage.
/// Based on command transform the tasks.
/// Store result to storage.
pub fn process_command(command: Option<Command>) -> Result<(), TaskrError> {
    let mut tasks: Vec<Task> = storage::load().map_err(TaskrError::Storage)?;
    match command {
        Some(Command::Add {title, priority}) => {
            let title = title.join(" ");
            let priority = priority.unwrap_or(Priority::Low);
            add_task(&mut tasks, title, priority);
        },
        Some(Command::List {filter }) => {
            list_tasks(&tasks, filter);
        },
        Some(Command::Done { indices }) => {
            toggle_tasks(&mut tasks, &indices).map_err(TaskrError::Index)?
        },
        Some(Command::Edit { index, title, priority }) => {
            let title = if title.is_empty() {
                None
            } else {
                Some(title.join(" "))
            };
            edit_task(&mut tasks, index, title, priority).map_err(TaskrError::Index)?
        },
        Some(Command::Remove(target)) => {
            remove_tasks(&mut tasks, &target).map_err(TaskrError::Index)?
        }
        None => {
            list_tasks(&tasks, ListFilter::All);
        }
    };
    storage::store(&tasks).map_err(TaskrError::Storage)?;
    Ok(())
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
    fn test_list_todo_tasks() {
        let t1 = Task::new("one".to_string(), true, Priority::Low);
        let t2 = Task::new("two".to_string(), false, Priority::Medium);
        let t3 = Task::new("three".to_string(), true, Priority::High);
        let tasks = vec![t1, t2, t3];

        let filter = ListFilter::Todo;
        list_tasks(&tasks, filter);

        // note: this test does not assert anything, one must check the output manually in the terminal

    }

    #[test]
    fn test_complete_task() {
        let t = Task::new("title".to_string(), false, Priority::Medium);
        let mut tasks = vec![t];

        let res = toggle_tasks(&mut tasks, &vec![0]);

        assert!(res.is_ok());
        assert!(tasks[0].is_completed())
    }

    #[test]
    fn test_uncomplete_task() {
        let t = Task::new("title".to_string(), true, Priority::Medium);
        let mut tasks = vec![t];

        let res = toggle_tasks(&mut tasks, &vec![0]);

        assert!(res.is_ok());
        assert!(!tasks[0].is_completed())
    }

    #[test]
    fn test_complete_multiple_tasks() {
        let t1 = Task::new("one".to_string(), false, Priority::Low);
        let t2 = Task::new("two".to_string(), false, Priority::Medium);
        let t3 = Task::new("three".to_string(), false, Priority::High);
        let mut tasks = vec![t1, t2, t3];

        let res = toggle_tasks(&mut tasks, &vec![0, 2]);

        assert!(res.is_ok());
        assert!(tasks[0].is_completed());
        assert!(!tasks[1].is_completed());
        assert!(tasks[2].is_completed());
    }

    #[test]
    fn test_complete_out_of_range() {
        let task = Task::new("title".to_string(), false, Priority::Medium);
        let mut tasks = vec![task];

        let res = toggle_tasks(&mut tasks, &vec![1]);

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
