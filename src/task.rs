/// Module with the core Task type
///
/// Contains the Task struct, its constructor, filed accessors
/// and an implementation of the Display trait.

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
            Priority::Low => "(!)", // todo low priority is empty string instead
            Priority::Medium => "(!!)",
            Priority::High => "(!!!)"
        };
        write!(f, "{done_str} {title_str} {prio_str}")
    }
}
