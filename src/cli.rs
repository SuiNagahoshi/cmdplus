use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[command(name = "cmdplus")]
#[command(version = "0.1")]
#[command(about = "A cross-platform shell tools suite", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create file (and directories if needed)
    Touch {
        /// File to create
        file: String,
    },

    /// Find the full path of a command
    Which {
        /// Name of the command
        program: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Touch { file } => {
            if let Err(e) = commands::touch::touch_command(&file) {
                eprintln!("touch failed: {e}");
                std::process::exit(1);
            }
        }
        Commands::Which { program } => match commands::which::which_command(&program) {
            Some(path) => println!("{}", path.display()),
            None => {
                eprintln!("{} not found in PATH", program);
                std::process::exit(1);
            }
        },
    }
}
