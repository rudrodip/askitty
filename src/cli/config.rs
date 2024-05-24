use std::env;
use std::error::Error;

use super::utils::print_help;

#[derive(Debug)]
pub enum Command {
    Help,
    Version,
    Message(String),
    REPL,
    ShowAllSessions,
    ShowSession(String),
    DeleteSession(String),
    ClearAllSessions,
    NewSession,
    GlobalSystemPrompt(String),
    SessionSystemPrompt(String, String),
    ShowGlobalSystemPrompt,
    ClearGlobalSystemPrompt,
    DeleteSessionSystemPrompt(String),
    Imagine(String),
    ViewConfig,
    SetConfig,
}

#[derive(Debug)]
pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            print_help()?;
            return Err("No command provided".into());
        }

        let command = match args[1].as_str() {
            "-h" | "--help" => Command::Help,
            "-v" | "--version" => Command::Version,
            "-m" | "--message" => {
                if args.len() < 3 {
                    return Err("No message provided".into());
                }
                Command::Message(args[2].clone())
            }
            "-r" | "--repl" => {
                if args.len() > 2 {
                    return Err("No prompt should be provided".into());
                }
                Command::REPL
            }
            "-i" | "--imagine" => {
                if args.len() < 3 {
                    return Err("No prompt provided".into());
                }
                Command::Imagine(args[2].clone())
            }
            "-s" | "--sessions" => {
                if args.len() < 3 {
                    Command::ShowAllSessions
                } else {
                    Command::ShowSession(args[2].clone())
                }
            }
            "-d" | "--delete" => {
                if args.len() < 3 {
                    return Err("No session id provided".into());
                }
                Command::DeleteSession(args[2].clone())
            }
            "-c" | "--clear" => Command::ClearAllSessions,
            "-n" | "--new" => Command::NewSession,
            "-p" | "--prompt" => {
                if args.len() < 3 {
                    return Err("No prompt provided".into());
                }
                if args.len() < 4 {
                    Command::GlobalSystemPrompt(args[2].clone())
                } else {
                    Command::SessionSystemPrompt(args[2].clone(), args[3].clone())
                }
            }
            "-ps" | "--prompt-show" => Command::ShowGlobalSystemPrompt,
            "-pc" | "--prompt-clear" => Command::ClearGlobalSystemPrompt,
            "-pd" | "--prompt-delete" => {
                if args.len() < 3 {
                    return Err("No session id provided".into());
                }
                Command::DeleteSessionSystemPrompt(args[2].clone())
            },
            "-vc" | "--view-config" => Command::ViewConfig,
            "-sc" | "--set-config" => Command::SetConfig,
            _ => Command::Help,
        };

        Ok(Config { command })
    }
}
