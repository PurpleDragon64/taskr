/// Module for interacting with persistant storage.
///
/// Handles serialization, deserialization and writing to file and
/// reading from file.

use std::fs;
use std::io;

use crate::task::Task;

// todo: change FILE_NAME to make module more testable
const FILE_NAME: &str = "storage";

/// Store a tasks slice to persistant storage.
/// The previous contents of the storage will be overwritten.
pub fn store(tasks: &[Task]) -> Result<(), io::Error> {
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
