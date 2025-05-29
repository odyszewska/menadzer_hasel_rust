use clap::{Parser, Subcommand};
use std::{fs::{self, File}, io::{Read, Write}, path::PathBuf,};
use directories::ProjectDirs;
use rpassword::prompt_password;
use anyhow::{Context, Result};
use rand::distr::Alphanumeric;
use rand::Rng;

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
    // wygeneruj losowe hasło
    Generate { length: Option<usize> },
}


fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => init_store()?,
        Commands::Insert { key } => insert(&key)?,
        Commands::Show { key } => show(&key)?,
        Commands::Remove { key } => remove(&key)?,
        Commands::List => list_keys()?,
        Commands::Generate { length } => {
            let len = length.unwrap_or(16);
            generate(len)?
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


fn show(key: &str) -> Result<()> {
    let path = key_to_path(key)?;
    let mut buf = String::new();
    File::open(&path)
        .context("Failed to open password file")?
        .read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}

fn remove(key: &str) -> Result<()> {
    let path = key_to_path(key)?;
    fs::remove_file(&path).context("Failed to remove password file")?;
    println!("Removed password for {}", key);
    Ok(())
}

fn list_keys() -> Result<()> {
    let root = store_dir()?;
    recurse_list(&root, &root)?;
    Ok(())
}

fn recurse_list(base: &PathBuf, dir: &PathBuf) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            recurse_list(base, &p)?;
        } else if p.extension().and_then(|e| e.to_str()) == Some("txt") {
            let rel = p.strip_prefix(base)?
                .with_extension("");
            println!("{}", rel.display());
        }
    }
    Ok(())
}

fn generate(length: usize) -> Result<()> {
    let pwd: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    println!("Wygenerowane: {}", pwd);

    Ok(())
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