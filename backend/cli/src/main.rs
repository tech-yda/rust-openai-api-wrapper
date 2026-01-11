//! cli - インタラクティブチャットCLI
//!
//! ターミナルでOpenAI APIと対話するためのCLIツール。

mod config;
mod repl;
mod session;

use clap::{Parser, Subcommand};
use colored::Colorize;

use backend_core::OpenAIService;

use crate::config::Config;
use crate::session::{list_sessions, Session};

/// OpenAI Chat CLI
#[derive(Parser)]
#[command(name = "cli")]
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

        /// Load an existing session
        #[arg(short, long)]
        load: Option<String>,
    },

    /// Ask a single question (one-shot mode)
    Ask {
        /// The question to ask
        question: String,

        /// System prompt to use
        #[arg(short, long)]
        system: Option<String>,
    },

    /// Manage saved sessions
    Sessions {
        #[command(subcommand)]
        command: SessionsCommand,
    },
}

#[derive(Subcommand)]
enum SessionsCommand {
    /// List saved sessions
    List,
    /// Delete a session
    Delete {
        /// Session name to delete
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数読み込み
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    // OpenAIサービス初期化
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        eprintln!("{}", "Error: OPENAI_API_KEY environment variable not set".red());
        std::process::exit(1);
    })?;
    let openai = OpenAIService::new(api_key);

    match cli.command {
        Commands::Chat { system, load } => {
            repl::run_repl(&openai, system, load).await?;
        }

        Commands::Ask { question, system } => {
            let config = Config::load();
            let system_prompt = system.or(Some(config.default.system_prompt));

            let messages = vec![backend_core::models::Message {
                role: "user".to_string(),
                content: question,
            }];

            match openai.chat_with_history(messages, system_prompt).await {
                Ok(response) => {
                    println!("{}", response.response);
                }
                Err(e) => {
                    eprintln!("{}", format!("Error: {}", e).red());
                    std::process::exit(1);
                }
            }
        }

        Commands::Sessions { command } => match command {
            SessionsCommand::List => {
                Config::ensure_dirs()?;
                match list_sessions() {
                    Ok(sessions) => {
                        if sessions.is_empty() {
                            println!("{}", "No saved sessions.".yellow());
                        } else {
                            println!("{}", "Saved sessions:".cyan().bold());
                            for s in sessions {
                                println!(
                                    "  {} ({} messages, {})",
                                    s.name.green(),
                                    s.message_count,
                                    s.updated_at.format("%Y-%m-%d")
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", format!("Error: {}", e).red());
                        std::process::exit(1);
                    }
                }
            }
            SessionsCommand::Delete { name } => {
                match Session::delete(&name) {
                    Ok(_) => {
                        println!("{}", format!("Deleted session: {}", name).green());
                    }
                    Err(e) => {
                        eprintln!("{}", format!("Error: {}", e).red());
                        std::process::exit(1);
                    }
                }
            }
        },
    }

    Ok(())
}
