use core::panic;
use std::fs;

use crate::encryption::{aes_decrypt, aes_encrypt, aes_gen_key};

const CONFIG: &str = "[settings]\n key = ";

pub fn initialize() {
    fs::create_dir_all(".joman").expect("Failed to create journal directory");
    fs::write(".joman/config.toml", CONFIG).expect("failed to write config.toml");
}

fn file_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read file")
}

fn encrypt_string(plaintext: &str, key: &str) -> String {
    aes_encrypt(plaintext, key).expect("Encryption failed")
}

fn write_to_file(file_path: &str, data: &str) {
    fs::write(file_path, data).expect("Failed to write to file");
}

pub fn add_file(file_path: &str, key: Option<&str>) {
    let plaintext_str = file_to_string(file_path);

    let encrypted_str = match key {
        Some(k) => encrypt_string(&plaintext_str, k),
        None => panic!("No key provided for encryption"),
    };

    write_to_file(file_path, &encrypted_str);
}

fn file_to_bytes(file_path: &str) -> Vec<u8> {
    fs::read(file_path).expect("Failed to read file")
}

fn bytes_to_str(data: &[u8]) -> &str {
    std::str::from_utf8(data).expect("Failed to convert bytes to string")
}

pub fn read_file(file_path: &str, key: Option<&str>) -> String {
    let data = fs::read(file_path).expect("Failed to read entry file");

    let encrypted_str = std::str::from_utf8(&data).expect("Failed to convert entry data to string");

    let plaintext_str = match key {
        Some(k) => aes_decrypt(encrypted_str, k).expect("Decryption failed"),
        None => encrypted_str.to_string(),
    };

    plaintext_str
}

pub fn lock_directory(key: Option<&str>) {
    let aes_key = match key {
        Some(k) => k.to_string(),
        None => aes_gen_key(),
    };

    let entries = fs::read_dir(".joman").expect("Failed to read journal directory");

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_file() {
            let data = fs::read_to_string(&path).expect("Failed to read entry file");
            let encrypted_data =
                aes_encrypt(&data, &aes_key).expect("Failed to encrypt entry data");
            fs::write(&path, encrypted_data).expect("Failed to write encrypted data to file");
        }
    }

    println!("Directory locked. AES-256 Key: {}", aes_key);
}
