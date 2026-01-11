//! インタラクティブREPL

use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use backend_core::OpenAIService;

use crate::config::Config;
use crate::session::{list_sessions, Session};

/// REPLを実行
pub async fn run_repl(
    openai: &OpenAIService,
    system_prompt: Option<String>,
    load_session: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    Config::ensure_dirs()?;

    // セッション初期化
    let mut session = if let Some(name) = load_session {
        match Session::load(&name) {
            Ok(s) => {
                println!(
                    "{}",
                    format!("Loaded session: {} ({} messages)", s.name, s.messages.len()).green()
                );
                s
            }
            Err(e) => {
                eprintln!("{}", format!("Failed to load session: {}", e).red());
                return Err(e.into());
            }
        }
    } else {
        let prompt = system_prompt.unwrap_or(config.default.system_prompt);
        Session::new(None, prompt)
    };

    // 履歴ファイルパス
    let history_path = Config::history_dir().join(format!("{}.txt", session.name));

    // Rustylineエディタ初期化
    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history(&history_path);

    println!("{}", "Welcome to Chat CLI! Type /help for commands.".cyan());
    println!();

    loop {
        let readline = rl.readline(&format!("{} ", ">".green().bold()));
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(input);

                // コマンド処理
                if input.starts_with('/') {
                    match handle_command(input, &mut session).await {
                        CommandResult::Continue => continue,
                        CommandResult::Exit => break,
                        CommandResult::Error(e) => {
                            eprintln!("{}", format!("Error: {}", e).red());
                            continue;
                        }
                    }
                }

                // チャット送信
                session.add_message("user", input);

                print!("{}", "Assistant: ".blue().bold());
                let messages = session.to_api_messages();
                let instructions = session.system_prompt();
                match openai.chat_with_history(messages, instructions).await {
                    Ok(response) => {
                        println!("{}", response.response);
                        session.add_message("assistant", &response.response);
                    }
                    Err(e) => {
                        eprintln!("{}", format!("API Error: {}", e).red());
                        // 失敗したメッセージを削除
                        session.messages.pop();
                    }
                }
                println!();
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "Interrupted. Type /exit to quit.".yellow());
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye!".cyan());
                break;
            }
            Err(err) => {
                eprintln!("{}", format!("Error: {}", err).red());
                break;
            }
        }
    }

    // 履歴を保存
    let _ = rl.save_history(&history_path);

    Ok(())
}

enum CommandResult {
    Continue,
    Exit,
    Error(String),
}

async fn handle_command(input: &str, session: &mut Session) -> CommandResult {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0];
    let arg = parts.get(1).map(|s| s.trim());

    match cmd {
        "/help" => {
            println!("{}", "Commands:".cyan().bold());
            println!("  {}  - Save current session", "/save <name>".green());
            println!("  {}  - Load a session", "/load <name>".green());
            println!("  {}         - List saved sessions", "/list".green());
            println!("  {}        - Clear current session", "/clear".green());
            println!("  {}         - Show this help", "/help".green());
            println!("  {}         - Exit REPL", "/exit".green());
            CommandResult::Continue
        }
        "/save" => {
            if let Some(name) = arg {
                session.name = name.to_string();
            }
            match session.save() {
                Ok(path) => {
                    println!(
                        "{}",
                        format!("Session saved to {}", path.display()).green()
                    );
                    CommandResult::Continue
                }
                Err(e) => CommandResult::Error(format!("Failed to save: {}", e)),
            }
        }
        "/load" => {
            if let Some(name) = arg {
                match Session::load(name) {
                    Ok(loaded) => {
                        *session = loaded;
                        println!(
                            "{}",
                            format!(
                                "Loaded session: {} ({} messages)",
                                session.name,
                                session.messages.len()
                            )
                            .green()
                        );
                        CommandResult::Continue
                    }
                    Err(e) => CommandResult::Error(format!("Failed to load: {}", e)),
                }
            } else {
                CommandResult::Error("Usage: /load <session-name>".to_string())
            }
        }
        "/list" => {
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
                    eprintln!("{}", format!("Failed to list sessions: {}", e).red());
                }
            }
            CommandResult::Continue
        }
        "/clear" => {
            session.clear();
            println!("{}", "Session cleared.".green());
            CommandResult::Continue
        }
        "/exit" | "/quit" => {
            println!("{}", "Goodbye!".cyan());
            CommandResult::Exit
        }
        _ => CommandResult::Error(format!("Unknown command: {}. Type /help for help.", cmd)),
    }
}
