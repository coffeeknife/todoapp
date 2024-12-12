use clap::{Parser, Subcommand};

mod data;

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
        #[arg(value_delimiter=' ', num_args=1.., required=true)]
        items: Option<Vec<String>>
    },
    Done {
        #[arg(value_delimiter=' ', num_args=1.., required=true)]
        ids: Option<Vec<u64>>
    },
    Del {
        #[arg(value_delimiter=' ', num_args=1.., required=true)]
        ids: Option<Vec<u64>>
    },
    Print {
        #[arg(short, long, conflicts_with="full")]
        done: bool,
        #[arg(short, long, conflicts_with="done")]
        full: bool,
        #[arg(value_delimiter=' ', num_args=1..)]
        ids: Option<Vec<u64>>
    }
}

fn main() {
    data::init_database("./db.sqlite");
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add { items }) => {
            println!("Add subcommand found!")
        }
        Some(Commands::Del { ids }) => {
            println!("Del subcommand found!")
        }
        Some(Commands::Done { ids }) => {
            println!("Done subcommand found!")
        }
        Some(Commands::Print { done, full, ids }) => {
            println!("Print subcommand found! Done: {done}, Full: {full}");
        }
        None => {
            println!("Main program (print todo list)")
        }
    }
}
