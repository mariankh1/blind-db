/*
 * DESCRIPTION: BlindDB Library Root
 * This is the entry point for the BlindDB crate. It links the crypto and 
 * indexing modules and exposes the public API for the VFS layer.
 *
 * HOW TO RUN: `cargo build`
 * * HOW TO TEST:
 * Run all project tests: `cargo test`
 */

// 1. Module Declarations 
pub mod crypto;
pub mod indexer;
pub mod indexer;

// 2. Public Exports
pub use crypto::RowEncryptor;
pub use indexer::BlindIndexer;
pub use vfs::BlindVfs;