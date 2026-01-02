/*
 * DESCRIPTION: Task T1.3 - Blind VFS (Virtual File System)
 * This module creates a wrapper around SQLite's filesystem operations.
 * It intercepts data before it is written to disk to apply encryption.
 *
 * HOW TO RUN: Integrated via `mod vfs` in lib.rs.
 * HOW TO TEST: `cargo test vfs`
 */

use sqlite_vfs::{Vfs, DatabaseHandle};
use crate::crypto::RowEncryptor;
use std::io::{Read, Write, Seek};

pub struct BlindVfs {
    pub encryptor: RowEncryptor,
}

impl Vfs for BlindVfs {
    type Handle = BlindDbHandle;

    fn open(&self, _db_name: &str, _opts: sqlite_vfs::OpenOptions) -> Result<Self::Handle, std::io::Error> {
        // Logic to open the underlying file goes here
        Ok(BlindDbHandle)
    }
}

pub struct BlindDbHandle;

impl DatabaseHandle for BlindDbHandle {
    fn read_at(&mut self, _offset: u64, _buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        // T1.3: Intercept read and apply decryption using RowEncryptor
        Ok(0)
    }

    fn write_at(&mut self, _offset: u64, _buffer: &[u8]) -> Result<usize, std::io::Error> {
        // T1.3: Intercept write and apply encryption using RowEncryptor
        Ok(0)
    }

    fn sync(&mut self) -> Result<(), std::io::Error> { Ok(()) }
    fn set_len(&mut self, _size: u64) -> Result<(), std::io::Error> { Ok(()) }
    fn get_len(&mut self) -> Result<u64, std::io::Error> { Ok(0) }
}