use rsa::{
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, rand_core::OsRng,
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
};
use base64::{engine::general_purpose, Engine as _};
use std::error::Error;

fn decode_b64(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    general_purpose::STANDARD.decode(input).map_err(|e| e.into())
}

fn encode_b64(input: &[u8]) -> String {
    general_purpose::STANDARD.encode(input)
}

pub fn rsa_gen_keypair() -> Result<(String, String), Box<dyn Error>> {
    let mut rng = OsRng;
    let bits = 2048;

    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    let private_pem = private_key.to_pkcs8_pem(Default::default())?;
    let public_pem = public_key.to_public_key_pem(Default::default())?;

    Ok((private_pem.to_string(), public_pem.to_string()))
}

pub fn rsa_encrypt(plaintext: &str, pub_key: &str) -> Result<String, Box<dyn Error>> {
    let public_key = RsaPublicKey::from_public_key_pem(pub_key)?;
    let mut rng = OsRng;

    let encrypted_data = public_key.encrypt(
        &mut rng,
        Pkcs1v15Encrypt,
        plaintext.as_bytes(),
    )?;

    Ok(encode_b64(&encrypted_data))
}

pub fn rsa_decrypt(ciphertext_b64: &str, priv_key: &str) -> Result<String, Box<dyn Error>> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(priv_key)?;
    let ciphertext = decode_b64(ciphertext_b64)?;

    let decrypted_data = private_key.decrypt(
        Pkcs1v15Encrypt,
        &ciphertext,
    )?;

    Ok(String::from_utf8(decrypted_data)?)
}
