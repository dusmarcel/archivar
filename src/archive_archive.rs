use std::{fs, fs::File, io, time::{Duration, UNIX_EPOCH}};

use anyhow::Result;
use rusqlite::{Connection, params};
use digest_io::IoWrapper;
use sha2::{Digest, Sha256};

use crate::age_bucket::age_bucket;

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
        let duration = Duration::from_secs_f32(timestamp);
        if let Some(age) = UNIX_EPOCH.checked_add(duration) {
            if let Some(file_name) = archive_path.file_name() {
                let bucket = age_bucket(age);
                if bucket >= 2 {
                    let p = adir.join(bucket.to_string());
                    fs::create_dir_all(&p)?;
                    fs::rename(&archive_path, p.join(file_name))?;
                }
            }
        }
    } else {
        let Some(prefix) = archive_path.file_prefix() else {
            return Ok(());
        };
        let name = prefix.to_string_lossy();
            // ...
        let timestamp: Option<f32> = conn.query_one(
            "SELECT timestamp FROM archive WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
    }

    Ok(())
}