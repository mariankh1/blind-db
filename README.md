# BlindDB

**A Forensic-Resilient Storage SDK for Secure Applications.**

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

## Executive Summary
BlindDB is an open-source side project exploring the **"Local Decryption Paradox"**: the vulnerability where local message history is exposed to forensic tools (e.g., Cellebrite, GrayKey) whenever a secure application is active.

By implementing **Searchable Symmetric Encryption (SSE)** and hardware-gated key derivation, BlindDB allows applications to search and retrieve data without ever "unlocking" the entire database in memory.


## 🚀 Key Innovations

### 1. Blind Indexing (SSE)
Unlike traditional full-disk encryption, BlindDB utilizes cryptographic "blind tokens" for searchable fields. 
- **Privacy:** The database engine retrieves records by matching hashes; it never sees the plaintext names or metadata.
- **Opacity:** An attacker with a database dump sees only useless, high-entropy random tokens.

### 2. Hardware-Backed Security
BlindDB is designed to integrate with a device's **Trusted Execution Environment (TEE)**:
- **iOS:** Secure Enclave
- **Android:** StrongBox / Keymaster
The secret "pepper" used for indexing remains isolated within the hardware security module.

### 3. Lease-Token Performance
To ensure a snappy user experience, we utilize a **Lease Token** model. The hardware releases short-lived keys to a protected memory region, allowing thousands of HMAC operations per second while ensuring the data is "re-blinded" the moment the app is backgrounded.



## 🏗️ Ecosystem Impact
This project is developed as an open-source **Digital Common**. Our goal is to provide a pluggable storage trait for:
- **Delta Chat:** Hardening the `deltachat-core-rust` storage layer.
- **Matrix:** Enhancing the Matrix Rust SDK with forensic-resilient storage.

## 🤝 Call for Collaboration
**This is a side project and we are looking for contributors!** We are specifically looking for help from researchers and developers in the following areas:
- **TEE Integration:** Developers with experience in `Apple Secure Enclave` or `Android StrongBox`.
- **Database Engineering:** Experience with SQLite custom functions or VFS layers.
- **Audit:** Cryptographers to review our SSE implementation and lease-token model.

## 🛠️ Technical Setup

### Installation
Add the following to your `Cargo.toml`:
```toml
[dependencies]
blind-db = { git = "[https://github.com/YOUR_USERNAME/blind-db](https://github.com/YOUR_USERNAME/blind-db)" }
hex = "0.4"
```

### Basic Usage
```rust
use blind_db::BlindIndexer;

fn main() {
    // 1. Initialize with a hardware-derived secret
    let secret_pepper = vec![0u8; 32]; 
    let indexer = BlindIndexer::new(secret_pepper);

    // 2. Tokenize a sensitive field (e.g., a contact name)
    let contact_name = "Alice";
    let blind_token = indexer.tokenize(contact_name);

    println!("Search Token: {}", blind_token);
}
```
## 🗺️ Project Roadmap

### Milestone 1: Core Foundation (Current)
- [x] T1.1: AES-GCM-SIV Encryption Core
- [x] T1.2: Blind Indexer (SSE)
- [x] T1.3: VFS Interception Layer

### Milestone 2: Hardware Security
- [ ] T2.1: Android StrongBox Integration
- [ ] T2.2: iOS Secure Enclave Wrapper
- [ ] T2.3: Master Key Derivation Function (PBKDF2/Argon2)

## ⚖️ License
Licensed under the Apache License, Version 2.0.
