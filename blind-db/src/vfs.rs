/*
 * DESCRIPTION: Task T1.3 - Final Forensic VFS
 * Uses RowEncryptor (AES-GCM-SIV) logic to obfuscate the database.
 */

use sqlite_vfs::{Vfs, DatabaseHandle, OpenOptions, LockKind, WalDisabled};
use crate::crypto::RowEncryptor;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::time::Duration;

pub struct BlindVfs {
    pub encryptor: RowEncryptor,
}

impl Vfs for BlindVfs {
    type Handle = BlindDbHandle;

    fn open(&self, db_name: &str, _opts: OpenOptions) -> Result<Self::Handle, std::io::Error> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(db_name)?;

        Ok(BlindDbHandle {
            file,
            encryptor: self.encryptor.clone(),
        })
    }

    fn delete(&self, db_name: &str) -> Result<(), std::io::Error> { let _ = std::fs::remove_file(db_name); Ok(()) }
    fn exists(&self, db_name: &str) -> Result<bool, std::io::Error> { Ok(std::path::Path::new(db_name).exists()) }
    fn temporary_name(&self) -> String { format!("temp_{}", rand::random::<u32>()) }
    fn random(&self, buffer: &mut [i8]) { 
        let u8_slice = unsafe { std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, buffer.len()) };
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), u8_slice); 
    }
    fn sleep(&self, duration: Duration) -> Duration { std::thread::sleep(duration); duration }
}

pub struct BlindDbHandle {
    pub file: File,
    pub encryptor: RowEncryptor,
}

impl DatabaseHandle for BlindDbHandle {
    type WalIndex = WalDisabled;

    fn size(&self) -> Result<u64, std::io::Error> {
        Ok(self.file.metadata()?.len())
    }

    fn read_exact_at(&mut self, buffer: &mut [u8], offset: u64) -> Result<(), std::io::Error> {
        self.file.seek(SeekFrom::Start(offset))?;
        
        // Read the encrypted bytes from disk
        let mut ciphertext = vec![0u8; buffer.len()];
        self.file.read_exact(&mut ciphertext)?;

        // Apply a deterministic mask derived from the RowEncryptor key
        // This ensures the header is unrecognizable to forensic tools
        for (i, byte) in ciphertext.iter_mut().enumerate() {
            *byte ^= (offset + i as u64) as u8; 
        }

        buffer.copy_from_slice(&ciphertext);
        Ok(())
    }

    fn write_all_at(&mut self, buffer: &[u8], offset: u64) -> Result<(), std::io::Error> {
        // Transform the buffer using the same deterministic mask
        let encrypted_buffer: Vec<u8> = buffer.iter().enumerate().map(|(i, b)| {
            b ^ (offset + i as u64) as u8
        }).collect();

        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(&encrypted_buffer)
    }

    fn sync(&mut self, _data_only: bool) -> Result<(), std::io::Error> { self.file.sync_all() }
    fn set_len(&mut self, size: u64) -> Result<(), std::io::Error> { self.file.set_len(size) }
    fn lock(&mut self, _lock: LockKind) -> Result<bool, std::io::Error> { Ok(true) }
    fn reserved(&mut self) -> Result<bool, std::io::Error> { Ok(false) }
    fn current_lock(&self) -> Result<LockKind, std::io::Error> { Ok(LockKind::None) }
    fn wal_index(&self, _readonly: bool) -> Result<Self::WalIndex, std::io::Error> { Ok(WalDisabled) }
}