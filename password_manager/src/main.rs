// src/crypto_gcm.rs
//use aes_gcm::aead::{Aead, KeyInit, OsRng};
//use aes_gcm::{Aes256Gcm, Nonce};
//
//pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
//    // Utilisez la même fonction PBKDF2 ou une clé aléatoire
//    let mut key = [0u8; 32];
//    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
//    key
//}
//
//pub fn encrypt(data: &[u8], key_bytes: &[u8; 32]) -> (Vec<u8>, [u8; 12]) {
//    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key_bytes);
//    let cipher = Aes256Gcm::new(key);
//    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96 bits
//    let ciphertext = cipher.encrypt(&nonce, data).expect("encryption failed");
//    (ciphertext, nonce.into())
//}
//
//pub fn decrypt(ciphertext: &[u8], key_bytes: &[u8; 32], nonce_bytes: &[u8; 12]) -> Vec<u8> {
//    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key_bytes);
//    let cipher = Aes256Gcm::new(key);
//    let nonce = Nonce::from_slice(nonce_bytes);
//    cipher.decrypt(nonce, ciphertext).expect("decryption failed")
//}
//

mod cli;
mod random_generator;

fn main() {
    cli::run_cli();
}
