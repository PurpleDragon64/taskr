/// Module for interacting with persistant storage.
///
/// Handles serialization, deserialization and writing to file and
/// reading from file.

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
