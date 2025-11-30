use std::fs;
use std::path::Path;

use crate::encryption::{rsa_gen_keypair, rsa_encrypt, rsa_decrypt};

pub fn initialize(name: &str) {
    if Path::new(name).exists() {
        println!("Journal already initialized in this directory.");
        return;
    }

    fs::create_dir_all(name).expect("Failed to create journal directory");

    let (priv_key, pub_key) = rsa_gen_keypair().expect("Failed to generate RSA keypair");

    let pub_key_path = format!("{}/public.pem", name);
    fs::write(pub_key_path, &pub_key).expect("Failed to save public key");

    println!("directory initialized!\n {}", &priv_key)

}

pub fn add_file(file_path: &str) {
    let plaintext = fs::read_to_string(file_path).expect("Failed to read file");

    let pub_key = fs::read_to_string(file_path).expect("Failed to read file");

    let ciphertext = rsa_encrypt(&plaintext, &pub_key).expect("failed to encrypt");

    fs::write(file_path, &ciphertext).expect("Failed to write to file");
}

pub fn read_file(file_path: &str, key: Option<&str>) -> String {
    let data = fs::read(file_path).expect("Failed to read entry file");

    let encrypted_str = std::str::from_utf8(&data).expect("Failed to convert entry data to string");

    let plaintext_str = match key {
        Some(k) => rsa_decrypt(encrypted_str, k).expect("Decryption failed"),
        None => encrypted_str.to_string(),
    };

    plaintext_str
}

pub fn lock_directory(key: Option<&str>) {
    let dir_entries = fs::read_dir("./.joman").expect("Failed to read current directory");


}
