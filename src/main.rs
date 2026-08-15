use taskr::{parser, process_command};

fn main() {
    let command = parser::parse();
    process_command(command);
}
