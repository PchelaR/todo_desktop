use crate::crypto;
use crate::todo::TodoItem;
use anyhow::{Context, Result, bail};
use chacha20poly1305::Key;
use directories::ProjectDirs;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub const TASKS_FILE: &str = "tasks.dat";
const FORMAT_VERSION: u8 = 1;

pub fn tasks_path() -> Result<PathBuf> {
    let dirs =
        ProjectDirs::from("com", "PR", "TodoDesktop").context("Couldn't find home directory")?;

    let dir = dirs.data_dir();
    std::fs::create_dir_all(dir).context("Failed to create data directory")?;

    Ok(dir.join(TASKS_FILE))
}

pub fn backup_broken(path: &Path) -> Result<PathBuf> {
    let backup = path.with_extension("dat.broken");
    std::fs::rename(path, &backup).context("Failed to back up broken tasks file")?;
    Ok(backup)
}

pub fn load_tasks(key: &Key, path: &Path) -> Result<Vec<TodoItem>> {
    let data = match std::fs::read(path) {
        Ok(data) => data,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e).context(format!("Failed to open {:?}", path)),
    };
    let plain_text = crypto::decrypt(key, &data)?;

    let (&version, body) = plain_text.split_first().context("Tasks file is empty")?;

    let tasks = match version {
        1 => postcard::from_bytes(body).context("Failed to read tasks")?,
        v => bail!("Unsupported tasks file version: {v}"),
    };
    Ok(tasks)
}

pub fn save_tasks(key: &Key, path: &Path, tasks: &[TodoItem]) -> Result<()> {
    let mut bytes = vec![FORMAT_VERSION];
    bytes.extend(postcard::to_allocvec(tasks).context("Failed to serialize tasks")?);
    let encrypted = crypto::encrypt(key, &bytes)?;

    let tmp_path = path.with_extension("dat.tmp");
    let tmp_file = File::create(&tmp_path).context("Failed to create tasks file")?;
    let mut writer = BufWriter::new(tmp_file);

    writer
        .write_all(&encrypted)
        .context("Failed to write tasks")?;
    writer.flush().context("Failed to flush tasks")?;
    writer
        .get_ref()
        .sync_all()
        .context("Failed to sync tasks")?;
    drop(writer);
    std::fs::rename(tmp_path, path).context("Failed to rename tasks file")?;
    Ok(())
}
