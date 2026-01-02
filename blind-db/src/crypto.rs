/*
 * DESCRIPTION: Task T1.1 - Row-Level Encryption
 * This module provides authenticated encryption using AES-256-GCM-SIV.
 * It is designed to be "nonce-reuse resistant" and memory-safe.
 *
 * HOW TO RUN: This is a library module. It is compiled via `cargo build`.
 * * HOW TO TEST:
 * Run specific tests: `cargo test crypto`
 */
 
use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Nonce, Key
};
use rand::{RngCore, thread_rng};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// RowEncryptor handles the forensic-resilient encryption of individual database rows.
/// It uses AES-256-GCM-SIV to ensure that even in cases of nonce reuse (common in 
/// complex mobile sync environments), the underlying encryption key remains secure.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct RowEncryptor {
    /// The 256-bit master key material. 
    /// ZeroizeOnDrop ensures this is physically wiped from RAM when the struct is dropped.
    key_material: [u8; 32],
}

impl RowEncryptor {
    /// Creates a new encryptor instance from a 32-byte key.
    /// In production, this key should be derived from the hardware TEE.
    pub fn new(key: [u8; 32]) -> Self {
        Self { key_material: key }
    }

    /// Encrypts a plaintext message.
    /// Returns a tuple containing the 12-byte nonce and the resulting ciphertext.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<([u8; 12], Vec<u8>), String> {
        let cipher = Aes256GcmSiv::new(Key::<Aes256GcmSiv>::from_slice(&self.key_material));
        
        // Generate a cryptographically secure random 12-byte nonce
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failure: {:?}", e))?;

        Ok((nonce_bytes, ciphertext))
    }

    /// Decrypts a ciphertext using the provided 12-byte nonce.
    /// This will fail if the ciphertext has been tampered with or the wrong key is used.
    pub fn decrypt(&self, nonce_bytes: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes256GcmSiv::new(Key::<Aes256GcmSiv>::from_slice(&self.key_material));
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failure: {:?}", e))?;

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_encryption_cycle() {
        let key = [0u8; 32];
        let encryptor = RowEncryptor::new(key);
        let original_text = b"This is a sensitive message for BlindDB.";

        // 1. Encrypt
        let (nonce, ciphertext) = encryptor.encrypt(original_text).expect("Should encrypt");
        assert_ne!(original_text.to_vec(), ciphertext, "Ciphertext must be different from plaintext");
        assert_eq!(nonce.len(), 12, "Nonce must be 12 bytes");

        // 2. Decrypt
        let decrypted_text = encryptor.decrypt(&nonce, &ciphertext).expect("Should decrypt");
        assert_eq!(original_text.to_vec(), decrypted_text, "Decrypted text must match original");
    }

    #[test]
    fn test_tamper_resistance() {
        let key = [1u8; 32];
        let encryptor = RowEncryptor::new(key);
        let (nonce, mut ciphertext) = encryptor.encrypt(b"Integrity test").unwrap();

        // Manipulate one byte of the ciphertext (tampering)
        ciphertext[0] ^= 0xFF;

        // Decryption should fail because GCM-SIV is an Authenticated Encryption mode
        let result = encryptor.decrypt(&nonce, &ciphertext);
        assert!(result.is_err(), "Decryption should fail on tampered data");
    }

    #[test]
    fn test_wrong_key_failure() {
        let (nonce, ciphertext) = RowEncryptor::new([1u8; 32]).encrypt(b"Secret").unwrap();
        
        // Try to decrypt with a different key
        let wrong_encryptor = RowEncryptor::new([2u8; 32]);
        let result = wrong_encryptor.decrypt(&nonce, &ciphertext);
        
        assert!(result.is_err(), "Decryption should fail with an incorrect key");
    }

    #[test]
    fn test_zeroize_memory_safety() {
        // This test verifies at compile-time that our struct implements the necessary
        // safety traits to resist forensic RAM extraction.
        fn assert_zeroize<T: zeroize::ZeroizeOnDrop>() {}
        assert_zeroize::<RowEncryptor>();
    }
}