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
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Action
    #[command(subcommand)]
    action: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Add text to embedding
    Add { text: String },

    /// Search text from pharse
    Search { text: String },
}

fn main() -> Result<()> {
    dotenv().ok();
    env_logger::try_init()?;
    log::debug!("App started");
    let args: Args = Args::parse();

    let api_key = std::env::var("OPENAI_TOKEN")?;
    log::debug!("OPENAI_TOKEN is {api_key}");
    let mut gpt = gpt::Gpt::try_create(api_key)?;
    let engine = VectorStore::try_create(config::VECTOR_DB_PATH, config::VECTOR_DIMENSION)?;
    let mut db = store::Store::new(config::DB_PATH);

    match args.action {
        Command::Add { text } => {
            let vector = gpt.create_vectors(&text)?;
            let id = uuid::Uuid::new_v4();
            let id = id.as_u128() as u32;
            log::debug!("vecotr length is {}", vector.len());
            log::debug!("config vector length is {}", config::VECTOR_DIMENSION);
            engine.upsert(id, &vector)?;
            db.set(&id.to_string(), &text)?;
            log::debug!("Added text: {}", text);
            println!("✅ Saved");
        }
        Command::Search { text } => {
            let search_vector = gpt.create_vectors(&text)?;
            // Handle searching text
            let result = engine.find(&search_vector)?;
            let first = result.first().context("empty result")?;
            log::trace!("found first result: {result:?}");
            log::debug!("found id {}", first.0);
            let stored_value = db.get(first.0.to_string())?;
            match stored_value  {
                Some(val) => {
                    println!("✅ Found: {val}")
                }
                _ => {
                    println!("❌ Not found!");
                }
            }
        }
    }
    Ok(())
}