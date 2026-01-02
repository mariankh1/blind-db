/*
 * DESCRIPTION: Task T1.1 - Row-Level Encryption
 * Implements AES-256-GCM-SIV for forensic-resistant storage.
 */

use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Nonce, Key
};
use rand::{RngCore, thread_rng};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop, Clone)] 
pub struct RowEncryptor {
    key_material: [u8; 32],
}

impl RowEncryptor {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key_material: key }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<([u8; 12], Vec<u8>), String> {
        let cipher = Aes256GcmSiv::new(Key::<Aes256GcmSiv>::from_slice(&self.key_material));
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failure: {:?}", e))?;
            
        Ok((nonce_bytes, ciphertext))
    }

    pub fn decrypt(&self, nonce_bytes: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes256GcmSiv::new(Key::<Aes256GcmSiv>::from_slice(&self.key_material));
        let nonce = Nonce::from_slice(nonce_bytes);
        
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failure: {:?}", e))
    }
}