use hmac::{Hmac, Mac};
use sha2::Sha256;

// This is our "Blind Indexer"
pub struct BlindIndexer {
    pepper: Vec<u8>, // The "secret salt"
}

impl BlindIndexer {
    pub fn new(pepper: Vec<u8>) -> Self {
        Self { pepper }
    }

    /// This turns a name like "Alice" into a random-looking token
    pub fn tokenize(&self, input: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.pepper)
            .expect("HMAC keys can be any size");
        mac.update(input.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let indexer = BlindIndexer::new(vec![1, 2, 3]);
        let token = indexer.tokenize("Alice");
        
        // This confirms "Alice" is now hidden!
        println!("The token for Alice is: {}", token);
        assert_ne!("Alice", token); 
    }
}