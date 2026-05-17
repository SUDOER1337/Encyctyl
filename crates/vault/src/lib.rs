use encyctyl_core::{Error, VaultConfig};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Vault {
    config: VaultConfig,
}

impl Vault {
    pub fn new(config: VaultConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &VaultConfig {
        &self.config
    }

    pub fn root(&self) -> &Path {
        &self.config.root
    }

    pub fn discover_notes(&self) -> Result<Vec<PathBuf>, Error> {
        let mut notes = Vec::new();
        for entry in WalkDir::new(&self.config.root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    notes.push(path.to_path_buf());
                }
            }
        }
        Ok(notes)
    }

    pub fn read_note(&self, path: &Path) -> Result<String, Error> {
        let full_path = self.config.root.join(path);
        if !full_path.exists() {
            return Err(Error::NoteNotFound(full_path));
        }
        Ok(std::fs::read_to_string(&full_path)?)
    }

    pub fn write_note(&self, path: &Path, content: &str) -> Result<(), Error> {
        use std::io::Write;
        let full_path = self.config.root.join(path);

        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut tmp = tempfile::NamedTempFile::new()?;
        tmp.write_all(content.as_bytes())?;
        tmp.persist(&full_path)
            .map_err(|e| Error::AtomicWrite(e.to_string()))?;

        tracing::debug!("wrote note: {}", full_path.display());
        Ok(())
    }
}
