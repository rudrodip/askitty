use crate::ai::llm::traits::LLM;
use crate::cli::utils;
use crate::storage::Storage;
use crate::types::llm::{Message, Session};
use std::error::Error;
use std::io::{self, Write};
use termimad;
use whoami;

pub async fn start_repl<L: LLM>(
    llm_client: &L,
    storage: &impl Storage,
) -> Result<(), Box<dyn Error>> {
    let mut session: Session = match utils::find_latest_session(storage) {
        Some(session) => session,
        None => {
            let new_session = Session::new();
            storage
                .set(&format!("sessions/{}", &new_session.id), &new_session)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            new_session
        }
    };

    repl_loop(llm_client, storage, &mut session).await?;

    Ok(())
}

pub async fn start_new_session<L: LLM>(
    llm_client: &L,
    storage: &impl Storage,
) -> Result<(), Box<dyn Error>> {
    let mut session = Session::new();
    storage
        .set(&format!("sessions/{}", &session.id), &session)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    repl_loop(llm_client, storage, &mut session).await?;

    Ok(())
}

async fn repl_loop<L: LLM>(
    llm_client: &L,
    storage: &impl Storage,
    session: &mut Session,
) -> Result<(), Box<dyn Error>> {
    let username = whoami::username();
    let global_system_prompt = utils::get_global_system_prompt(storage)?;
    loop {
        print!("{}: ", username);
        io::stdout().flush()?;
        let prompt = utils::read_line()?;
        if prompt == "exit" {
            print!("Goodbye! 👋\n");
            break;
        }

        print!("kitty 🐱: ");
        io::stdout().flush()?;
        let chat = Message {
            role: String::from("user"),
            content: prompt.clone(),
        };
        session.history.push(chat);
        let mut chats = vec![];
        let system_message = Message {
            role: "system".to_string(),
            content: global_system_prompt.clone() + &session.system_prompt.clone(),
        };
        chats.push(system_message);
        chats.extend(session.history.clone());
        let response = llm_client.completion(chats).await?;

        session.history.push(Message {
            role: String::from("assistant"),
            content: response.clone(),
        });

        storage
            .set(format!("sessions/{}", &session.id).as_str(), &session)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        termimad::print_text(&response);
    }
    Ok(())
}
