//! cli - インタラクティブチャットCLI
//!
//! ターミナルでOpenAI APIと対話するためのCLIツール。

use clap::{Parser, Subcommand};

/// OpenAI Chat CLI
#[derive(Parser)]
#[command(name = "chat-cli")]
#[command(about = "Interactive chat CLI for OpenAI API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive chat session
    Chat {
        /// System prompt to use
        #[arg(short, long)]
        system: Option<String>,
    },
    /// List saved sessions
    Sessions,
}

#[tokio::main]
async fn main() {
    // ロギング初期化
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Chat { system } => {
            println!("Starting interactive chat...");
            if let Some(prompt) = system {
                println!("System prompt: {}", prompt);
            }
            println!("(Interactive REPL will be implemented in Phase 2)");
            println!("Type /help for available commands, /quit to exit.");
        }
        Commands::Sessions => {
            println!("Saved sessions:");
            println!("(Session listing will be implemented in Phase 2)");
        }
    }
}
