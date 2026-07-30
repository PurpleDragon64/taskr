use taskr::parser::{self, Commands};

fn main() {
    let command = parser::parse();

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
