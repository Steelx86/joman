use core::fmt;
use std::fs;
use std::path::Path;
use std::error::Error;

use crate::encryption::{rsa_decrypt, rsa_encrypt, rsa_gen_keypair};

#[derive(Debug)]
struct JournalError(String);

impl fmt::Display for JournalError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JournalError: {}", self.0)
    }
}

impl Error for JournalError {}

pub fn initialize() -> Result<String, Box<dyn Error>> {
    if Path::new(".journal").exists() {
        println!(".journal already initialized in this directory.");
        return Err(Box::new(JournalError("Journal already initialized".to_string())));
    }

    fs::create_dir_all(".journal").expect("Failed to create journal directory");

    let (priv_key, pub_key) = rsa_gen_keypair().expect("Failed to generate RSA keypair");

    let pub_key_path = format!(".journal/public.pem");
    fs::write(pub_key_path, &pub_key).expect("Failed to save public key");

    Ok(priv_key)
}

pub fn add_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let plaintext = fs::read_to_string(file_path).expect("Failed to read file");

    let pub_key = fs::read_to_string(file_path).expect("Failed to read file");

    let ciphertext = rsa_encrypt(&plaintext, &pub_key).expect("failed to encrypt");

    let file_path = format!(
        ".journal/{}",
        Path::new(file_path).file_name().unwrap().to_str().unwrap()
    );

    fs::write(file_path, &ciphertext).expect("Failed to write to file");

    Ok(())
}

pub fn read_file(file_path: &str, key: Option<&str>) -> Result<String, Box<dyn Error>> {
    let data = fs::read(file_path).expect("Failed to read entry file");

    let encrypted_str = std::str::from_utf8(&data).expect("Failed to convert entry data to string");

    let plaintext_str = match key {
        Some(k) => rsa_decrypt(encrypted_str, k).expect("Decryption failed"),
        None => encrypted_str.to_string(),
    };

    Ok(plaintext_str)
}

pub fn lock_directory(key: Option<&str>) -> Result<(), Box<dyn Error>> {
    let dir_path = ".journal";
    let entries = fs::read_dir(dir_path).expect("Failed to read journal directory");

    for entry in entries {
        let entry = entry.expect("Failed to get directory entry");
        let path = entry.path();

        if path.is_file() {
            let data = fs::read_to_string(&path).expect("Failed to read file");

            let encrypted_data = match key {
                Some(k) => rsa_encrypt(&data, k).expect("Encryption failed"),
                None => data,
            };

            fs::write(&path, &encrypted_data).expect("Failed to write encrypted data");
        }
    }

    Ok(())
}
