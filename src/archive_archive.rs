use std::{io, fs::File};

use anyhow::Result;
use rusqlite::{Connection, params};
use digest_io::IoWrapper;
use sha2::{Digest, Sha256};

pub fn archive_archive(
    archive_path: &std::path::Path,
    adir: &std::path::Path,
    dry_run: bool,
    conn: &Connection,
) -> Result<()> {
    if dry_run {
        println!("Would archive archive file: {}", archive_path.display());
        return Ok(());
    }

    let mut archive = File::open(archive_path)?;
    let mut writer = IoWrapper(Sha256::new());
    io::copy(&mut archive, &mut writer)?;
    let hsh: [u8; 32] = writer.0.finalize().into();
    println!(
        "Archiving archive file: {} with hash {:?}",
        archive_path.display(),
        hsh
    );
    let timestamp: Option<f32> = conn.query_one(
        "SELECT timestamp FROM archive WHERE hash = ?1",
        params![hsh],
        |row| row.get(3),
    )?;

    if let Some(timestamp) = timestamp {
        // ...do something
    } else {
        // ...do something else
    }

    Ok(())
}