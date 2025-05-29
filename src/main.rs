use clap::{Parser, Subcommand};
use std::{fs::{self, File}, io::{Read, Write}, path::PathBuf,};
use directories::ProjectDirs;
use rpassword::prompt_password;


#[derive(Parser)]
#[command(name = "pass-mng", about = "Menadzer hasel")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // inicjalizacja katalogu haseł
    Init, 
    // dodanie i zmiana hasłą
    Insert { key: String },
    // pokaż hasło
    Show { key: String },
    // usun hasło
    Remove { key: String },
    // Wypisz klucze
    List,
    // wygeneruj losowe hasło i zapisz je
    Generate { key: String, length: Option<usize> },
}


fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => init_store()?,
        Commands::Insert { key } => insert(&key)?,
        Commands::Show { key } => show(&key)?,
        Commands::Remove { key } => remove(&key)?,
        Commands::List => list_keys()?,
        Commands::Generate { key, length } => {
            let len = length.unwrap_or(16);
            generate(&key, len)?
        }
    }
    Ok(())
}

fn init_store() -> _ {
    todo!()
}

fn insert(p0: &String) -> _ {
    todo!()
}

fn show(p0: &String) -> _ {
    todo!()
}

fn remove(p0: &String) -> _ {
    todo!()
}

fn list_keys() -> _ {
    todo!()
}

fn generate(p0: &String, p1: usize) -> _ {
    todo!()
}