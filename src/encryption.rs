use aes_gcm::{
    Aes256Gcm, Key, Nonce, 
    aead::{Aead, AeadCore, KeyInit, OsRng}, aes::Aes256,
};
use base64::{engine::general_purpose, Engine as _};
use std::error::Error;

#[derive(Debug)]
struct AesError(String);

impl std::fmt::Display for AesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AES Error: {}", self.0)
    }
}

impl Error for AesError {}

fn decode_b64(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    general_purpose::STANDARD.decode(input).map_err(|e| e.into())
}

fn encode_b64(input: &[u8]) -> String {
    general_purpose::STANDARD.encode(input)
}

pub fn aes_gen_key() -> String {
    encode_b64(Aes256Gcm::generate_key(&mut OsRng).as_slice())
}

pub fn aes_encrypt(plaintext: &str, key_b64: &str) -> Result<String, Box<dyn Error>> {
    decode_b64(key_b64)
        .map(|k| Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&k)))
        .and_then(|cipher| {
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            cipher
                .encrypt(&nonce, plaintext.as_bytes())
                .map(|ct| [nonce.as_slice(), ct.as_slice()].concat())
                .map_err(|_| Box::new(AesError("Encryption failed".to_string())) as Box<dyn Error>)
        })
        .map(|bytes| encode_b64(&bytes))
}

pub fn aes_decrypt(cipher_b64: &str, key_b64: &str) -> Result<String, Box<dyn Error>> {
    decode_b64(key_b64)
        .map(|k| Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&k)))
        .and_then(|cipher| {
            decode_b64(cipher_b64).and_then(|data| {
                if data.len() < 12 {
                    return Err("Ciphertext too short".into());
                }
                let (nonce_bytes, ct_bytes) = data.split_at(12);
                let nonce = Nonce::from_slice(nonce_bytes);
                cipher
                    .decrypt(nonce, ct_bytes)
                    .map_err(|_| Box::new(AesError("Decryption failed".to_string())) as Box<dyn Error>)
            })
        })
        .and_then(|pt_bytes| {
            String::from_utf8(pt_bytes).map_err(|e| e.into())
        })
}
