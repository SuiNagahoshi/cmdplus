//! # cmdplus - Cross-platform Shell Tools Suite
//!
//! ## English
//! This is the entry point of the cmdplus CLI application.
//! It delegates control to the CLI handler defined in `cli.rs`.
//!
//! ## 日本語
//! これは cmdplus CLI アプリケーションのエントリーポイントです。  
//! 実際の引数解析およびコマンドの分岐処理は `cli.rs` に委譲されます。

mod cli;
mod commands;

fn main() {
    cli::run();
}
