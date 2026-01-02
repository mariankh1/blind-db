# BlindDB 🛡️

**Forensic Resilience & Plausible Deniability for Secure Applications.**

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Active_Research-green.svg)]()

## 🕰️ Breaking a 5-Year Deadlock
Discussions regarding app-level encryption and "forced disclosure" protection have been active in the privacy community (notably the Delta Chat forums) since **August 2021**. Implementation has often remained "stuck" due to the architectural complexity of managing decoy states without leaking metadata or compromising performance.

**BlindDB** is a Rust-based cryptographic library designed to break this deadlock. It provides a standalone, pluggable storage layer that allows messaging apps to implement forensic resilience and plausible deniability without rebuilding their entire core.

---

## 🚀 Key Pillars

### 1. Blind Indexing (Forensic Resilience)
Traditional "at-rest" encryption often fails when an app is "hot" (active in memory). BlindDB implements **Searchable Symmetric Encryption (SSE)**.
- **The Tech:** Data is indexed using cryptographic "blind tokens" (HMAC-based).
- **The Result:** Even if a device is seized while the app is active, a memory dump reveals only opaque tokens, not your plaintext message history.



### 2. Multi-State Indexing (Plausible Deniability)
To resist social coercion (being forced to unlock a device), BlindDB supports a **Dual-PIN Architecture**. 
- **Decoy State:** Entering a "Safety PIN" derives keys for a separate index containing harmless, plausible data.
- **True State:** Entering the "True PIN" unlocks the hardware-gated keys for the actual sensitive history.
The database structure is designed so that the existence of a "hidden" state is cryptographically unprovable.



### 3. Existence Hiding (Probabilistic Privacy)
Hiding message content is insufficient if metadata (like contact lists) is leaked. BlindDB utilizes **Bloom Filters**.
- **The Tech:** A probabilistic data structure used to verify membership without storing the actual keys.
- **The Result:** If an adversary searches for a hidden contact, the system returns "Not Found." Because Bloom Filters allow for rare false positives, any unexpected match can be plausibly denied as a mathematical artifact.



---

## 🏗️ Ecosystem Impact
This project is an open-source **Digital Common**. We are currently researching integration paths for:
- **Delta Chat:** Hardening the `deltachat-core-rust` storage layer against forensic extraction.
- **Matrix:** Providing a Zero-Trust storage option for the Matrix Rust SDK.

## 🤝 Collaboration: Join the Side Project!
We are looking for researchers and developers to help with:
- **TEE Integration:** Implementing hardware-backed key derivation (Secure Enclave / StrongBox).
- **Database Engineering:** Wrapping this logic into a custom SQLite VFS (Virtual File System).
- **Audit:** Cryptographic review of our "Deniable SSE" and lease-token models.

## 🛠️ Technical Setup

### Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
blind-db = { git = "[https://github.com/mariankh1/blind-db](https://github.com/mariankh1/blind-db)" }
hex = "0.4"
```

### Basic Usage
```rust
use blind_db::BlindIndexer;

fn main() {
    // 1. Initialize with a hardware-derived secret (pepper)
    let secret_pepper = vec![0u8; 32]; 
    let indexer = BlindIndexer::new(secret_pepper);

    // 2. Tokenize a sensitive field for an opaque search index
    let contact_name = "Alice";
    let blind_token = indexer.tokenize(contact_name);

    println!("Searchable Opaque Token: {}", blind_token);
}
```

## 📜 Roadmap
- [x] Core HMAC-based Blind Indexing (Rust)
- [ ] Multi-State "Decoy" logic
- [ ] Bloom-filter based metadata skipping
- [ ] Hardware key derivation (TEE) bridge for iOS/Android

## ⚖️ License
Licensed under the Apache License, Version 2.0.