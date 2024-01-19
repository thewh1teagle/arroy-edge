mod vector_store;
mod gpt;
mod config;
mod store;
use env_logger;
use log;
use dotenv::dotenv;
use anyhow::{Result, Context};
use crate::vector_store::VectorStore;
use uuid;


fn main() -> Result<()> {
    dotenv().ok();
    env_logger::try_init()?;
    log::debug!("App started");

    let api_key = std::env::var("OPENAI_TOKEN")?;
    log::debug!("OPENAI_TOKEN is {api_key}");
    let mut gpt = gpt::Gpt::try_create(api_key)?;
    let engine = VectorStore::try_create(config::VECTOR_DB_PATH, config::VECTOR_DIMENSION)?;
    let mut db = store::Store::new(config::DB_PATH);

    loop {
        println!("Action:");
        println!("1. add text input");
        println!("2. search text");
        println!("3. exit");

        let choice = get_user_input("Enter your choice: ")?;

        match choice.as_str() {
            "1" => {
                let text = get_user_input("Enter text: ")?;
                let vector = gpt.create_vectors(&text)?;
                let id = uuid::Uuid::new_v4();
                let id = id.as_u128() as u32;
                log::debug!("vecotr length is {}", vector.len());
                log::debug!("config vector length is {}", config::VECTOR_DIMENSION);
                engine.upsert(id, &vector)?;
                db.set(&id.to_string(), &text)?;
                log::debug!("Added text: {}", text);
            }
            "2" => {
                let search_text = get_user_input("Enter text to search: ")?;
                let search_vector = gpt.create_vectors(&search_text)?;
                // Handle searching text
                let result = engine.find(&search_vector)?;
                let first = result.first().context("empty result")?;
                log::trace!("found first result: {result:?}");
                log::debug!("found id {}", first.0);
                let stored_value = db.get(first.0.to_string())?;
                match stored_value  {
                    Some(val) => {
                        println!("Found: {val}")
                    }
                    _ => {
                        println!("Not found!");
                    }
                }

            }
            "3" => {
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option.");
            }
        }
    }
    
    Ok(())
}

fn get_user_input(prompt: &str) -> Result<String> {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Remove trailing newline
    Ok( input.trim().to_string() )
}