use aes::cipher::generic_array::GenericArray;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

// Salt for PBKDF2
const SALT: &[u8] = b"unique_salt_value";

// Derive the AES-256 encryption key from the master password
fn derive_key_from_password(master_password: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(master_password.as_bytes(), SALT, 100_000, &mut key);
    key
}

// Encrypt a password using AES-256 and a key derived from the master password
pub fn encrypt_password(master_password: &str, password: &str) -> String {
    let key = derive_key_from_password(master_password);

    let cipher = ChaCha20Poly1305::new(&GenericArray::from_slice(&key));
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, password.as_bytes()).unwrap();

    // prepend nonce
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    hex::encode(result)
}

// Decrypt an encrypted password
pub fn decrypt_password(master_password: &str, encrypted_password: &str) -> String {
    let key = derive_key_from_password(master_password);
    let cipher = ChaCha20Poly1305::new(&GenericArray::from_slice(&key));
    let decoded = hex::decode(encrypted_password).unwrap();
    let nonce = GenericArray::from_slice(&decoded[..12]);
    let ciphertext = &decoded[12..];
    let decrypted = cipher.decrypt(&nonce, &ciphertext[..]).unwrap();

    String::from_utf8(decrypted).unwrap()
}
