use anyhow::{Context, Result, anyhow, bail};
use chacha20poly1305::aead::{Aead, Generate};
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use keyring_core::{Entry, Error as KeyringError};

const SERVICE: &str = "TodoDesktop";
const ACCOUNT: &str = "tasks-key";
const NONCE_LEN: usize = 12;

pub fn init_keyring() -> Result<()> {
    #[cfg(target_os = "macos")]
    keyring_core::set_default_store(apple_native_keyring_store::keychain::Store::new()?);

    #[cfg(target_os = "windows")]
    keyring_core::set_default_store(windows_native_keyring_store::Store::new()?);

    #[cfg(target_os = "linux")]
    keyring_core::set_default_store(zbus_secret_service_keyring_store::Store::new()?);

    Ok(())
}

pub fn get_or_create_key() -> Result<Key> {
    let entry = Entry::new(SERVICE, ACCOUNT).context("Failed to open keychain entry")?;

    match entry.get_secret() {
        Ok(bytes) => {
            Key::try_from(bytes.as_slice()).map_err(|_| anyhow!("Stored key has wrong length"))
        }
        Err(KeyringError::NoEntry) => {
            let key = Key::generate();
            entry.set_secret(&key).context("Failed to store key")?;
            Ok(key)
        }
        Err(e) => Err(e).context("Failed to read key from keychain"),
    }
}

pub fn encrypt(key: &Key, plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::generate();

    let cipher_text = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| anyhow!("Encryption failed"))?;

    let mut out = nonce.to_vec();
    out.extend_from_slice(&cipher_text);
    Ok(out)
}

pub fn decrypt(key: &Key, data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < NONCE_LEN {
        bail!("Encrypted data is too short");
    }
    let (nonce, cipher_text) = data.split_at(NONCE_LEN);
    let nonce = Nonce::try_from(nonce).map_err(|_| anyhow!("Invalid nonce"))?;
    ChaCha20Poly1305::new(key)
        .decrypt(&nonce, cipher_text)
        .map_err(|_| anyhow!("Decryption failed: wrong key or corrupted file"))
}
