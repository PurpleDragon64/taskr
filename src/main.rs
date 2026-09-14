use taskr::{parse, process_command};

fn main() {
    let command = parse();
    if let Err(e) = process_command(command) {
        println!("Error: {e}")
    }
}
