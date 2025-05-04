//! # CLI Parser and Dispatcher
//!
//! ## English
//! This module defines the command-line interface for `cmdplus`, using `clap` for parsing.
//! It includes subcommands like `touch`, `which`, and others defined in `commands`.
//!
//! ## 日本語
//! このモジュールは `cmdplus` のコマンドラインインターフェースを定義し、`clap` を使用してパースを行います。  
//! `touch` や `which` などのサブコマンドは `commands` モジュールに定義されています。
use clap::{Parser, Subcommand};

use crate::commands;

/// Defines CLI arguments using `clap`.
///
/// ## English
/// Defines the overall command structure for cmdplus.
///
/// ## 日本語
/// `cmdplus` の全体的なコマンド構造を定義します。
#[derive(Parser)]
#[command(name = "cmdplus")]
#[command(version = "0.1")]
#[command(about = "A cross-platform shell tools suite", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Subcommand enum for the CLI interface.
///
/// ## English
/// Lists available commands such as `touch`, `which`, etc.
///
/// ## 日本語
/// 利用可能なコマンド（例: `touch`, `which` など）を列挙します。
#[derive(Subcommand)]
enum Commands {
    /// Create file (and directories if needed)
    /// - ファイルおよび必要なディレクトリを作成します。
    Touch {
        /// File to create - 作成するファイル名
        file: String,
    },

    /// Find the full path of a command
    /// - コマンドのフルパスを表示します
    Which {
        /// Name of the command - 対象となるコマンド名
        program: String,
    },

    /// List directory sizes
    /// - ディレクトリをサイズ順に一覧表示します
    Lsz {
        /// Target directory (optional) - 対象ディレクトリ（省略可能）
        dir: Option<String>,
    },
}

/// Entry point for CLI logic.
///
/// ## English
/// Dispatches parsed CLI arguments to appropriate handlers.
///
/// ## 日本語
/// パース済みの CLI 引数を適切なハンドラに振り分けます。
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
        Commands::Lsz { dir } => {
            let dir_path = dir.unwrap_or_else(|| ".".to_string());
            if let Err(e) = commands::lsz::lsz_command(&dir_path) {
                eprintln!("lsz failed: {e}");
                std::process::exit(1);
            }
        }
    }
}
