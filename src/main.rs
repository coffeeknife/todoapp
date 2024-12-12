use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "TodoApp")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Add {     
    },
    Done {
    },
    Del {
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add {}) => {
            println!("Add subcommand found!")
        },
        Some(Commands::Del {}) => {
            println!("Del subcommand found!")
        },
        Some(Commands::Done {}) => {
            println!("Done subcommand found!")
        }
        None => {
            println!("Main program (print todo list)")
        }
    }
}
