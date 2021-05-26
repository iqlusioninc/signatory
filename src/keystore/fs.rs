//! Filesystem-backed keystore

use crate::{Error, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::{fs::Permissions, os::unix::fs::PermissionsExt};

/// Key labels.
// TODO(tarcieri): extract this into a proper type that validates its contents
pub type Label = str;

/// Required filesystem mode for keystore directories (Unix-only)
#[cfg(unix)]
const REQUIRED_DIR_MODE: u32 = 0o700;

/// Filesystem-backed keystore.
pub struct FsKeyStore {
    path: PathBuf,
}

impl FsKeyStore {
    /// Create a filesystem-backed keystore at the given path, creating a new
    /// directory and setting its file permissions.
    pub fn create(dir_path: &Path) -> Result<Self> {
        fs::create_dir_all(&dir_path)?;

        #[cfg(unix)]
        fs::set_permissions(&dir_path, Permissions::from_mode(REQUIRED_DIR_MODE))?;

        Self::open(&dir_path)
    }

    /// Initialize filesystem-backed keystore, opening the directory at the
    /// provided path and checking that it has the correct filesystem
    /// permissions.
    pub fn open(dir_path: &Path) -> Result<Self> {
        let path = dir_path.canonicalize()?;
        let st = path.metadata()?;

        if !st.is_dir() {
            return Err(Error::NotADirectory);
        }

        #[cfg(unix)]
        if st.permissions().mode() & 0o777 != REQUIRED_DIR_MODE {
            return Err(Error::Permissions);
        }

        Ok(Self { path })
    }

    /// Import a PKCS#8 key into the keystore.
    pub fn store(&self, label: &Label, der: &pkcs8::PrivateKeyDocument) -> Result<()> {
        der.write_pem_file(&self.key_path(label))?;
        Ok(())
    }

    /// Load a PKCS#8 key from the keystore.
    pub fn load(&self, label: &Label) -> Result<pkcs8::PrivateKeyDocument> {
        Ok(pkcs8::PrivateKeyDocument::read_pem_file(
            &self.key_path(label),
        )?)
    }

    /// Delete a PKCS#8 key from the keystore.
    pub fn delete(&self, label: &Label) -> Result<()> {
        fs::remove_file(&self.key_path(label))?;

        Ok(())
    }

    /// Compute the path for a key with a given label.
    fn key_path(&self, label: &Label) -> PathBuf {
        // TODO(tarcieri): extract `Label` type and validate label
        let mut path = self.path.join(label);
        path.set_extension("pem");
        path
    }
}

#[cfg(test)]
#[allow(unused_imports)] // TODO(tarcieri): always use imports
mod tests {
    use super::FsKeyStore;
    use crate::keystore::GeneratePkcs8;

    #[cfg(feature = "secp256k1")]
    #[test]
    fn import_key() {
        use crate::ecdsa::secp256k1;

        let dir = tempfile::tempdir().unwrap();
        let keystore = FsKeyStore::create(&dir.path().join("keys")).unwrap();

        let label = "example_key";
        let example_key = secp256k1::SigningKey::generate_pkcs8();
        keystore.store(label, &example_key).unwrap();

        let example_key2 = keystore.load(label).unwrap();
        assert_eq!(example_key.as_ref(), example_key2.as_ref());

        keystore.delete(label).unwrap();
    }
}
