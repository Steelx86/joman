use std::error::Error;
use std::fs;
use std::path::Path;

use crate::encryption::{hyb_decrypt, hyb_encrypt, rsa_gen_keypair};

pub fn initialize() -> Result<(), Box<dyn Error>> {
    if Path::new("Journal").exists() {
        println!("Journal already initialized in this directory.");
        return Err("Journal already initialized".into());
    }

    fs::create_dir_all("Journal")
        .map_err(|e| format!("Failed to create journal directory: {}", e))?;

    let (priv_key, pub_key) =
        rsa_gen_keypair().map_err(|e| format!("Failed to generate RSA keypair: {}", e))?;

    let pub_key_path = format!("Journal/public.pem");
    let priv_key_path = format!("./private.pem");

    fs::write(pub_key_path, &pub_key).map_err(|e| format!("Failed to save public key: {}", e))?;

    fs::write(priv_key_path, &priv_key)
        .map_err(|e| format!("Failed to save private key: {}", e))?;

    Ok(())
}

pub fn add_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    let plaintext =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let pub_key = fs::read_to_string("Journal/public.pem")
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let ciphertext =
        hyb_encrypt(&plaintext, &pub_key).map_err(|e| format!("failed to encrypt: {}", e))?;

    let encrypted_file = format!(
        "Journal/{}.enc",
        Path::new(file_path).file_name().unwrap().to_str().unwrap()
    );

    fs::write(encrypted_file, &ciphertext)
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

pub fn add_directory(dir_path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(dir_path).exists() {
        return Err(format!("Directory not found: {}", dir_path).into());
    }

    let entries = fs::read_dir(dir_path).map_err(|e| format!("Failed to read directory: {}", e))?;

    let pub_key = fs::read_to_string("Journal/public.pem")
        .map_err(|e| format!("Failed to read public key: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to get directory entry: {}", e))?;

        let path = entry.path();

        if path.is_file() {
            let plaintext =
                fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

            let ciphertext =
                hyb_encrypt(&plaintext, &pub_key).map_err(|e| format!("Failed to encrypt file"))?;

            let dest_path = format!(
                "Journal/{}.enc",
                path.file_name().unwrap().to_str().unwrap()
            );

            fs::write(dest_path, &ciphertext)
                .map_err(|e| format!("Failed to write encrypted file: {}", e))?;
        }
    }

    Ok(())
}

pub fn read_file(file_path: &str, key_path: &str) -> Result<String, Box<dyn Error>> {
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    let data = fs::read(file_path).map_err(|e| format!("Failed to read entry file: {}", e))?;

    let priv_key = fs::read_to_string(key_path)
        .map_err(|e| format!("Failed to read private key file: {}", e))?;

    let encrypted_str = std::str::from_utf8(&data)
        .map_err(|e| format!("Failed to convert entry data to string: {}", e))?;

    let plaintext_str = hyb_decrypt(encrypted_str, &priv_key)
        .map_err(|e| format!("Failed to decrypt entry: {}", e))?;

    Ok(plaintext_str)
}
