use taskr::{parser, process_command};

fn main() {
    let command = parser::parse();
    if let Err(e) = process_command(command) {
        println!("Error: {e}")
    }
}
