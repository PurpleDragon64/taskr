use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
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
        indices: Vec<u32>
    },
    /// Edit selected task
    Edit {
        /// Index of selected task
        index: u32,

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
#[derive(Clone, ValueEnum, Debug)]
enum Priority {
    Low,
    Medium,
    High
}

// Remove Debug?
#[derive(Clone, ValueEnum, Debug)]
enum ListFilter {
    All,
    Done,
    Undone
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct RemoveTarget {
    /// Indices of selected tasks
    indices: Vec<u32>,

    /// Remove all completed tasks
    #[arg(long)]
    done: bool,

    /// Remove all tasks
    #[arg(long)]
    all: bool,
}

fn main() {
    let cli = Cli::parse();

    // only placeholder prints
    // todo: replace with real implementation
    match cli.command {
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
