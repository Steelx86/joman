use std::fs;
use std::path::Path;
use std::error::Error;

use crate::encryption::{hyb_decrypt, hyb_encrypt, rsa_gen_keypair};

pub fn initialize() -> Result<(), Box<dyn Error>> {
    if Path::new("Journal").exists() {
        println!("Journal already initialized in this directory.");
        return Err("Journal already initialized".into());
    }

    fs::create_dir_all("Journal").expect("Failed to create journal directory");

    let (priv_key, pub_key) = rsa_gen_keypair().expect("Failed to generate RSA keypair");

    let pub_key_path = format!("Journal/public.pem");
    let priv_key_path = format!("./private.pem");
    fs::write(pub_key_path, &pub_key).expect("Failed to save public key");
    fs::write(priv_key_path, &priv_key).expect("Failed to save private key");

    Ok(())
}

pub fn add_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let plaintext = fs::read_to_string(file_path).expect("Failed to read file");

    let pub_key = fs::read_to_string("Journal/public.pem").expect("Failed to read file");

    let ciphertext = hyb_encrypt(&plaintext, &pub_key).expect("failed to encrypt");

    let encrypted_file = format!(
        "Journal/{}.enc",
        Path::new(file_path).file_name().unwrap().to_str().unwrap()
    );

    fs::write(encrypted_file, &ciphertext).expect("Failed to write to file");

    Ok(())
}

pub fn add_directory(dir_path: &str) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(dir_path).expect("Failed to read directory");

    let pub_key = fs::read_to_string("Journal/public.pem").expect("Failed to read public key");

    for entry in entries {
        let entry = entry.expect("Failed to get directory entry");

        let path = entry.path();

        if path.is_file() {
            let plaintext = fs::read_to_string(&path).expect("Failed to read file");

            let ciphertext = hyb_encrypt(&plaintext, &pub_key).expect("Failed to encrypt file");

            let dest_path = format!(
                "Journal/{}.enc",
                path.file_name().unwrap().to_str().unwrap()
            );

            fs::write(dest_path, &ciphertext).expect("Failed to write encrypted file");
        }
    }

    Ok(()) 
}

pub fn read_file(file_path: &str, key_path: &str) -> Result<String, Box<dyn Error>> {
    let data = fs::read(file_path).expect("Failed to read entry file");

    let priv_key = fs::read_to_string(key_path).expect("Failed to read private key file");

    let encrypted_str = std::str::from_utf8(&data).expect("Failed to convert entry data to string");

    let plaintext_str = hyb_decrypt(encrypted_str, &priv_key).expect("Failed to decrypt entry");

    Ok(plaintext_str)
}

