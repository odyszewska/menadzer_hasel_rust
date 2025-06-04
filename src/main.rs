use clap::{Parser, Subcommand};
use std::{fs::{self, File}, io::{Read, Write}, path::PathBuf,};
use std::process::{Command, Stdio};
use directories::ProjectDirs;
use rpassword::prompt_password;
use anyhow::{ensure, Context, Result};
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
    Generate { length: Option<usize>, #[arg(short, long)] special: bool},
}


fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => init_store()?,
        Commands::Insert { key } => insert(&key)?,
        Commands::Show { key } => show(&key)?,
        Commands::Remove { key } => remove(&key)?,
        Commands::List => list_keys()?,
        Commands::Generate { length , special} => {
            let len = length.unwrap_or(16);
            generate(len, special)?
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
    let recipient = std::env::var("RECIPIENT").context("RECIPIENT not set")?;
    let ciphertext = encrypt_with_gpg(pass.as_bytes(), &recipient)?;
    let path = key_to_path(key)?;
    let tmp = path.with_extension("tmp");
    let mut f = File::create(&tmp).context("Failed to create temp file")?;
    f.write_all(&ciphertext)?;
    f.sync_all()?;
    fs::rename(&tmp, &path)?;
    println!("Saved password for {}", key);
    Ok(())

}


fn show(key: &str) -> Result<()> {
    let path = key_to_path(key)?;
    let mut buf = Vec::new();
    File::open(&path)
        .context("Failed to open password file")?
        .read_to_end(&mut buf)?;
    let plaintext = decrypt_with_gpg(&buf)?;
    let s = String::from_utf8(plaintext).context("Decrypted data not valid UTF-8")?;
    println!("{}", s);
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
        } else if p.extension().and_then(|e| e.to_str()) == Some("gpg") {
            let rel = p.strip_prefix(base)?
                .with_extension("");
            println!("{}", rel.display());
        }
    }
    Ok(())
}

fn generate(length: usize, use_special: bool) -> Result<()> {
    ensure!(length > 5, "Password length must be bigger than 5");

    if !use_special {
        let pwd: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        println!("Generated: {}", pwd);

        return Ok(())
    }

    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                 abcdefghijklmnopqrstuvwxyz\
                 0123456789\
                 !@#$%^&*()";
    let charset: Vec<char> = chars.chars().collect();
    let mut rng = rand::rng();

    let pwd: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx]
        })
        .collect();

    println!("Generated: {}", pwd);
    Ok(())
}

fn sanitize_key_part(part:&str) -> Result<()> {
    if part.is_empty() || part == "." || part == ".." {
        anyhow::bail!("Key contains invalid character: {}", part);
    }
    Ok(())
}

fn key_to_path(key: &str) -> Result<PathBuf> {
    let mut p = store_dir()?;
    for part in key.split('/') {
        sanitize_key_part(part)?;
        p.push(part);
    }
    p.set_extension("gpg");
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).context("Failed to create directory for key")?;
    }
    Ok(p)
    
}

fn encrypt_with_gpg(plaintext: &[u8], recipient: &str) -> Result<Vec<u8>> {
    let mut child = Command::new("gpg")
        .args(&["--encrypt","--armor","--recipient", recipient])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .with_context(||"Gpg program not found.")?;
    child.stdin.as_mut().context("Failed to open gpg stdin")?
        .write_all(plaintext).context("Failed to write to gpg stdin")?;
    let output = child.wait_with_output().context("Failed to read gpg output")?;
    if !output.status.success(){
        anyhow::bail!("Gpg exited with status {}", output.status);
    }
    Ok(output.stdout)
}

fn decrypt_with_gpg(ciphertext: &[u8]) -> Result<Vec<u8>> {
    let mut child = Command::new("gpg")
        .arg("--decrypt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to spawn gpg process")?;
    child.stdin.as_mut().context("Failed to open gpg stdin")?
        .write_all(ciphertext).context("Failed to write to gpg stdin")?;
    let output = child.wait_with_output().context("Failed to read gpg output")?;
    if !output.status.success(){
        anyhow::bail!("Gpg exited with status {}", output.status);
    }
    Ok(output.stdout)

}