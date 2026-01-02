// examples/basic_usage.rs
use blind_db::BlindIndexer;

fn main() {
    // 1. Setup the indexer (In a real app, this key comes from the hardware)
    let secret_key = vec![0u8; 32]; 
    let db_shield = BlindIndexer::new(secret_key);

    // 2. Tokenize a sensitive name
    let contact_name = "Alice";
    let blind_token = db_shield.tokenize(contact_name);

    println!("Original Name: {}", contact_name);
    println!("Stored Token:  {}", blind_token);
    println!("---");
    println!("The database now stores the token, making it forensic-resilient!");
}
