use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "touch", about = "Unix-like touch command for Windows")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Touch {
        /// ファイル名
        files: Vec<PathBuf>,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Touch { files } => {
            for file in files {
                cmdplus::touch_file(file)?;
            }
        }
    }

    Ok(())
}
