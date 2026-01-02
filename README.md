# 🛡️ BlindDB

**A Forensic-Resilient Storage SDK for Secure Applications.**

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

## Executive Summary
BlindDB is an open-source project exploring the **"Local Decryption Paradox"**: the vulnerability where local message history is exposed to forensic tools (e.g., Cellebrite, GrayKey) whenever a secure application is active.

By implementing **Searchable Symmetric Encryption (SSE)** and a custom **Forensic VFS**, BlindDB allows applications to search and retrieve data without ever "unlocking" the entire database in memory or leaving recognizable database signatures on disk.

---

## 🚀 Key Innovations

### 1. Blind Indexing (SSE)
Unlike traditional full-disk encryption, BlindDB utilizes cryptographic "blind tokens" for searchable fields. 
- **Privacy:** The database engine retrieves records by matching hashes; it never sees the plaintext names or metadata.
- **Opacity:** An attacker with a database dump sees only useless, high-entropy random tokens.

### 2. Forensic-Resilient VFS
The Virtual File System (VFS) layer intercepts SQLite I/O to obfuscate the database header and page structure. This ensures that the file on disk does not match the "SQLite format 3" signature, making it invisible to automated forensic scanners.



### 3. Hardware-Backed Security (Milestone 2)
BlindDB is designed to integrate with a device's **Trusted Execution Environment (TEE)**, such as Apple's **Secure Enclave** or Android's **StrongBox**, keeping the root "pepper" isolated from the main OS.

---

## 📊 Development Progress

| Task ID | Component | Status | Verification Method |
| :--- | :--- | :--- | :--- |
| **T1.1** | **AES-GCM-SIV Core** | ✅ Complete | Unit tests for encryption/decryption symmetry. |
| **T1.2** | **Blind Indexer (SSE)** | ✅ Complete | HMAC-SHA256 determinism validation. |
| **T1.3** | **Blind VFS Layer** | ✅ Complete | Integration test: Header obfuscation check. |
| **T2.1** | **Android StrongBox** | 🏗️ Planned | Hardware-backed key attestation. |
| **T2.2** | **iOS Secure Enclave** | 🏗️ Planned | Keychain Services / SEP integration. |
| **T2.3** | **Key Derivation (KDF)** | 🏗️ Planned | Argon2id memory-hard constraint testing. |

---

## 🧪 Verification & Testing
To ensure the cryptographic integrity and forensic resilience of BlindDB, we use a multi-layered testing suite.



### **1. Automated Unit Tests**
Validates the internal logic of the encryption and indexing modules.
```bash
cargo test --lib
```
### **2. A Forensic Integration Test (The "Success Test")

This is the primary validation for Milestone 1. It performs a "Forensic Audit" by attempting to find SQLite signatures on the disk after data has been saved and verified.

```bash
cargo test --test integration_test -- --nocapture
```

## 🛠️ Technical Setup
### Installation

Add the following to your Cargo.toml:
```bash
[dependencies]
blind-db = { git = "[https://github.com/mariankh1/blind-db](https://github.com/mariankh1/blind-db)" }
```


### Basic Usage
```bash
use blind_db::{BlindIndexer, RowEncryptor};

fn main() {
    // 1. Initialize with a hardware-derived secret
    let secret_pepper = vec![0u8; 32]; 
    let master_key = [0u8; 32];
    
    let indexer = BlindIndexer::new(secret_pepper);
    let encryptor = RowEncryptor::new(master_key);

    // 2. Tokenize a sensitive field (e.g., a contact name)
    let search_token = indexer.tokenize("Alice");

    // 3. Encrypt data for storage
    let (nonce, ciphertext) = encryptor.encrypt(b"Sensitive Message").unwrap();

    println!("Search Token: {}", search_token);
}
```

### 🏗️ Ecosystem Impact
This project is developed as an open-source Digital Common. Our goal is to provide a pluggable storage trait for:

Delta Chat: Hardening the deltachat-core-rust storage layer.

Matrix: Enhancing the Matrix Rust SDK with forensic-resilient storage.

### 🤝 Call for Collaboration
We are looking for help in the following areas:

- TEE Integration: Developers with experience in Apple Secure Enclave or Android StrongBox.

- Audit: Cryptographers to review our SSE implementation and VFS model.

### ⚖️ License
Licensed under the Apache License, Version 2.
