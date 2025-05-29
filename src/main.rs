use clap::{Parser, Subcommand};
use std::{fs::{self, File}, io::{Read, Write}, path::PathBuf,};
use directories::ProjectDirs;
use rpassword::prompt_password;
use anyhow::{Context, Result};


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


fn store_dir() -> Result<PathBuf> {
    let proj = ProjectDirs::from("io", "MyOrg", "pass-mng")
        .context("Failed to get project directories")?;
    Ok(proj.data_local_dir().join("password-store"))
}


fn init_store() -> Result<()> {
    let dir = store_dir()?;
    fs::create_dir_all(&dir).context("Failed to create store directory")?;
    println!("Store directory created: {}", dir.display());
    Ok(())
}

//atomowy wzorzec zapisu plików
fn insert(key: &str) -> Result<()> {
    let pass = prompt_password("Password: ").context("Failed to read password")?;
    let path = key_to_path(key)?;
    let tmp = path.with_extension("tmp");
    let mut f = File::create(&tmp)?;
    f.write_all(pass.as_bytes())?;
    f.sync_all()?;
    fs::rename(&tmp, &path)?;
    println!("Saved password for {}", key);
    Ok(())

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

fn key_to_path(key: &str) -> Result<PathBuf> {
    let mut p = store_dir()?;
    for part in key.split('/') {
        p.push(part);
    }
    p.set_extension("txt");
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(p)
    
}