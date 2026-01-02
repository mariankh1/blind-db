/*
 * DESCRIPTION: Task T1.2 - Searchable Symmetric Encryption (SSE)
 * This module converts plaintext keywords into opaque "Blind Tokens" using HMAC-SHA256.
 * This allows for database searching without revealing the search term to the disk.
 *
 * HOW TO RUN: Included via `mod indexer` in lib.rs.
 * * HOW TO TEST:
 * Run specific tests: `cargo test indexer`
 */
 
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop};

type HmacSha256 = Hmac<Sha256>;

/// The BlindIndexer creates 'Search Tokens' (Trapdoors).
/// It allows the database to index a field without knowing its value.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct BlindIndexer {
    /// The secret pepper derived from hardware. 
    /// Wiped from RAM when this struct is dropped.
    pepper: Vec<u8>, 
}

impl BlindIndexer {
    pub fn new(pepper: Vec<u8>) -> Self {
        Self { pepper }
    }

    /// Generates a deterministic hex token for a given keyword.
    /// This is used for both INSERT and SELECT operations.
    pub fn tokenize(&self, input: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.pepper)
            .expect("HMAC-SHA256 accepts keys of any size");
        
        mac.update(input.as_bytes());
        let result = mac.finalize();
        
        // Hex encoding ensures it is safe for SQL TEXT columns
        hex::encode(result.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() {
        let pepper = vec![42u8; 32];
        let indexer = BlindIndexer::new(pepper);
        
        // Searching for the same name must always return the same token
        let t1 = indexer.tokenize("Alice");
        let t2 = indexer.tokenize("Alice");
        assert_eq!(t1, t2, "Tokens must be identical for search to work");
    }

    #[test]
    fn test_uniqueness() {
        let indexer = BlindIndexer::new(vec![42u8; 32]);
        
        let t1 = indexer.tokenize("Alice");
        let t2 = indexer.tokenize("Bob");
        assert_ne!(t1, t2, "Different inputs must have different tokens");
    }
}