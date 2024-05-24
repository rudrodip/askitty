use crate::types::{config::AIConfig, llm::Session};
use crate::Storage;
use std::{error::Error, fs};
use config::{Config, File};
use dirs::config_dir;

pub fn print_help() -> Result<(), Box<dyn Error>> {
    println!("Usage: askitty [FLAG] [MESSAGE]");
    println!();
    println!("Flags:");
    println!(" -h, --help Display help message");
    println!(" -v, --version Display version");
    println!(" -m, --message Message to send to the model");
    println!(" -i, --imagine Generate image from text");
    Ok(())
}

pub fn print_version() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("Cargo.toml")?;
    let version = contents
        .lines()
        .nth(2)
        .ok_or_else(|| "Could not find version in Cargo.toml".to_owned())?
        .split(" = ")
        .nth(1)
        .ok_or_else(|| "Could not parse version from Cargo.toml".to_owned())?;
    println!("Version: {}", version);
    Ok(())
}

pub fn read_line() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn find_latest_session(storage: &impl Storage) -> Option<Session> {
    let mut latest_session: Option<Session> = None;
    let mut latest_timestamp: i64 = 0;

    if let Ok(keys) = storage.iter_keys() {
        for key in keys {
            if key.starts_with("sessions/") {
                if let Ok(Some(session)) = storage.get::<Session>(&key) {
                    if session.created_at > latest_timestamp {
                        latest_session = Some(session.clone());
                        latest_timestamp = session.created_at
                    }
                }
            }
        }
    }

    latest_session
}

pub fn find_all_sessions(storage: &impl Storage) -> Result<Vec<Session>, Box<dyn Error>> {
    let mut sessions: Vec<Session> = vec![];

    if let Ok(keys) = storage.iter_keys() {
        for key in keys {
            if key.starts_with("sessions/") {
                if let Ok(Some(session)) = storage.get::<Session>(&key) {
                    sessions.push(session.clone());
                }
            }
        }
    }

    Ok(sessions)
}

pub fn print_text(text: &str) {
    termimad::print_text(text);
}

pub fn show_all_sessions(storage: &impl Storage) -> Result<(), Box<dyn Error>> {
    match find_all_sessions(storage) {
        Ok(sessions) => {
            for session in sessions {
                println!("{}", session);
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn show_session(storage: &impl Storage, session_id: &str) -> Result<(), Box<dyn Error>> {
    let session_key = format!("sessions/{}", session_id);
    match storage.get::<Session>(&session_key) {
        Ok(Some(session)) => {
            println!("{}", session);
            println!("System prompt: {}", session.system_prompt);

            for message in session.history {
                println!("{}", message);
            }
            Ok(())
        }
        Ok(None) => {
            println!("Session not found");
            Ok(())
        }
        Err(e) => Err(Box::new(e) as Box<dyn Error>),
    }
}

pub fn delete_session(storage: &impl Storage, session_id: &str) -> Result<(), Box<dyn Error>> {
    let session_key = format!("sessions/{}", session_id);
    match storage.delete(&session_key) {
        Ok(_) => {
            println!("Session deleted");
            Ok(())
        }
        Err(e) => Err(Box::new(e) as Box<dyn Error>),
    }
}

pub fn clear_sessions(storage: &impl Storage) -> Result<(), Box<dyn Error>> {
    if let Ok(keys) = storage.iter_keys() {
        for key in keys {
            if key.starts_with("sessions/") {
                storage.delete(&key)?;
            }
        }
    }
    println!("All sessions deleted");
    Ok(())
}

pub fn set_global_system_prompt(
    storage: &impl Storage,
    prompt: &str,
) -> Result<(), Box<dyn Error>> {
    storage
        .set("GLOBAL_SYSTEM_PROMPT", &prompt)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    println!("Global system prompt set to: {}", prompt);
    Ok(())
}

pub fn set_session_system_prompt(
    storage: &impl Storage,
    session_id: &str,
    prompt: &str,
) -> Result<(), Box<dyn Error>> {
    let session_key = format!("sessions/{}", session_id);
    match storage.get::<Session>(&session_key) {
        Ok(Some(mut session)) => {
            session.system_prompt = prompt.to_string();
            storage
                .set(&session_key, &session)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            println!("Session system prompt set to: {}", prompt);
            Ok(())
        }
        Ok(None) => {
            println!("Session not found");
            Ok(())
        }
        Err(e) => Err(Box::new(e) as Box<dyn Error>),
    }
}

pub fn get_global_system_prompt(storage: &impl Storage) -> Result<String, Box<dyn Error>> {
    match storage.get::<String>("GLOBAL_SYSTEM_PROMPT") {
        Ok(Some(prompt)) => Ok(prompt),
        Ok(None) => Ok(String::new()),
        Err(e) => Err(Box::new(e) as Box<dyn Error>),
    }
}

pub fn show_global_system_prompt(storage: &impl Storage) -> Result<(), Box<dyn Error>> {
    match storage.get::<String>("GLOBAL_SYSTEM_PROMPT") {
        Ok(Some(prompt)) => {
            println!("Global system prompt: {}", prompt);
            Ok(())
        }
        Ok(None) => {
            println!("Global system prompt not set");
            Ok(())
        }
        Err(e) => Err(Box::new(e) as Box<dyn Error>),
    }
}

pub fn clear_global_system_prompt(storage: &impl Storage) -> Result<(), Box<dyn Error>> {
    match storage.set("GLOBAL_SYSTEM_PROMPT", &String::new()) {
        Ok(_) => {
            println!("Global system prompt cleared");
            Ok(())
        }
        Err(e) => Err(Box::new(e) as Box<dyn Error>),
    }
}

pub fn delete_session_system_prompt(
    storage: &impl Storage,
    session_id: &str,
) -> Result<(), Box<dyn Error>> {
    let session_key = format!("sessions/{}", session_id);
    match storage.get::<Session>(&session_key) {
        Ok(Some(mut session)) => {
            session.system_prompt = String::new();
            storage
                .set(&session_key, &session)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            println!("Session system prompt cleared");
            Ok(())
        }
        Ok(None) => {
            println!("Session not found");
            Ok(())
        }
        Err(e) => Err(Box::new(e) as Box<dyn Error>),
    }
}

pub fn view_config() -> Result<(), Box<dyn Error>>{
    let config_builder = Config::builder();
    let config_dir = config_dir().expect("Failed to get config directory");
    let config_path = config_dir.join(format!("{}/ai.toml", "askitty"));

    let s = config_builder
        .add_source(File::from(config_path))
        .build()?;

    let config = s.try_deserialize::<AIConfig>()?;
    println!("{}", config);
    Ok(())
}
