// src/crypto_cbc.rs
use aes::Aes256;
use block_modes::{Cbc, BlockMode, block_padding::Pkcs7};
use block_modes::BlockModeError;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::RngCore;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn derive_key(password: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = vec![0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
    key
}

pub fn generate_iv() -> Vec<u8> {
    let mut iv = vec![0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    iv
}

pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, BlockModeError> {
    let cipher = Aes256Cbc::new_from_slices(key, iv)?;
    Ok(cipher.encrypt_vec(data))
}

pub fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, BlockModeError> {
    let cipher = Aes256Cbc::new_from_slices(key, iv)?;
    cipher.decrypt_vec(ciphertext)
}
