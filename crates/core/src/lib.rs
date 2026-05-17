use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub root: PathBuf,
}

impl VaultConfig {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
    #[error("Note not found: {0}")]
    NoteNotFound(PathBuf),
    #[error("Atomic write failed: {0}")]
    AtomicWrite(String),
    #[error("Database error: {0}")]
    Database(String),
}
