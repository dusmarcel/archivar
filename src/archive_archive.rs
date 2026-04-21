use std::{env, fs, fs::File, io, time::{Duration, UNIX_EPOCH}};

use anyhow::Result;
use rusqlite::{Connection, params};
use digest_io::IoWrapper;
use sha2::{Digest, Sha256};

use crate::{
    age_bucket::age_bucket,
    create_archive::create_archive,
    latest_content_modification_time::latest_content_modification_time
};

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
    let Some(file_name) = archive_path.file_name() else {
        return Ok(());
    };

    let timestamp: Option<f32> = conn.query_one(
        "SELECT timestamp FROM archive WHERE hash = ?1",
        params![hsh],
        |row| row.get(3),
    )?;

    if let Some(timestamp) = timestamp {
        let duration = Duration::from_secs_f32(timestamp);
        if let Some(age) = UNIX_EPOCH.checked_add(duration) {
            let bucket = age_bucket(age);
            if bucket >= 2 {
                let p = adir.join(bucket.to_string());
                fs::create_dir_all(&p)?;
                fs::rename(&archive_path, p.join(file_name))?;
            }
        }
    } else {
        let Some(prefix) = archive_path.file_prefix() else {
            return Ok(());
        };
        let temp_dir = env::temp_dir();
        //let name = prefix.to_string_lossy();
        let decoder = xz2::read::XzDecoder::new(File::open(archive_path)?);
        let mut archive = tar::Archive::new(decoder);
        let file_dir = temp_dir.join(prefix);
        archive.unpack(temp_dir.join(&file_dir))?;
        // let timestamp: Option<f32> = conn.query_one(
        //     "SELECT timestamp FROM archive WHERE name = ?1",
        //     params![name],
        //     |row| row.get(0),
        // )?;

        // CHECK ME!!!
        let Some(latest) = latest_content_modification_time(&file_dir)? else {
            return Ok(());
        };
        let bucket = age_bucket(latest);
        if bucket >= 2 {
            let year = file_name.to_string_lossy().split('_').next().unwrap_or_default().parse::<u16>().unwrap_or_default();
            let (mut archive, archive_name) = create_archive(&file_dir.to_string_lossy(), year)?;
            let Some(file_name) = archive_name.file_name() else {
                return Ok(());
            };

            let mut writer = IoWrapper(Sha256::new());
            io::copy(&mut archive, &mut writer)?;
            let hsh: [u8; 32] = writer.0.finalize().into();

            let p = adir.join(bucket.to_string());
            fs::create_dir_all(&p)?;
            fs::copy(&archive_name, p.join(file_name))?;
            fs::remove_file(&archive_name)?;
            fs::remove_dir_all(file_dir)?;

            conn.execute(
            "INSERT INTO archive (name, change_time, hash) VALUES (?1, ?2, ?3) ON CONFLICT(name) DO UPDATE SET change_time = excluded.change_time, hash = excluded.hash",
            (archive_name.file_prefix().unwrap_or_default().to_string_lossy(), latest.duration_since(UNIX_EPOCH)?.as_secs_f64(), hsh)
            )?;
        }
    }

    Ok(())
}