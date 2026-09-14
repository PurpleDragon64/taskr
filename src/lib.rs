mod parser;
mod task;
mod taskr;
mod storage;

pub use taskr::{process_command, TaskrError};
pub use parser::{parse, Command};