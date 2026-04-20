use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io, io::Read, path::PathBuf};

use anyhow::Result;
use digest_io::IoWrapper;
use rusqlite::{Connection, types::Null};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::{
    create_archive::create_archive,
    archive_archive::archive_archive,
    age_bucket::age_bucket,
};

fn is_xz_archive(p: &std::path::Path) -> bool {
    let Ok(mut file) = fs::File::open(p) else {
        return false;
    };
    let mut magic = [0u8; 6];
    file.read_exact(&mut magic).unwrap_or(());
    magic == [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]
}

fn starts_with_three_digits(dir_name: &str) -> bool {
    dir_name.chars().take(3).count() == 3 && dir_name.chars().take(3).all(|c| c.is_ascii_digit())
}

fn latest_content_modification_time(
    path: &std::path::Path,
) -> Result<Option<SystemTime>> {
    let mut latest = None;

    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let modified = entry.metadata()?.modified()?;

        match latest {
            Some(current_latest) if modified <= current_latest => {}
            _ => latest = Some(modified),
        }
    }

    Ok(latest)
}

fn is_empty_dir(path: &std::path::Path) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(fs::read_dir(path)?.next().is_none())
}

pub fn archive_year_dir(
    dir: PathBuf,
    adir: &PathBuf,
    dry_run: bool,
    remove: bool,
    conn: &Connection,
) -> Result<()> {
    println!("Got year directory: {}", dir.display());
    let year = dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .parse::<u16>()?;
    println!("Got year: {}", year);

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        println!("Got entry: {}", entry.path().display());
        let file_type = entry.file_type()?;

        if !file_type.is_dir() {
            if entry.file_type()?.is_file() && is_xz_archive(&entry.path()) {
                archive_archive(&entry.path(), adir, dry_run, conn)?;
            }
            continue;
        }

        let dir_name = entry.file_name();
        let dir_name = dir_name.to_string_lossy();

        if !starts_with_three_digits(&dir_name) {
            continue;
        }

        if let Ok(empty_dir) = is_empty_dir(&entry.path()) {
            if empty_dir {
                if remove {
                    if dry_run {
                        println!("Would remove empty directory: {}", entry.path().display());
                    } else {
                        fs::remove_dir(&entry.path())?;
                        println!("Removed empty directory: {}", entry.path().display());
                    }
                } else {
                    println!("Empty directory (not removed): {}", entry.path().display());
                }
                continue;
            }
        }

        let Some(latest) = latest_content_modification_time(&entry.path())? else {
            continue;
        };

        let bucket = age_bucket(latest);
        // let no = dir_name
        //     .chars()
        //     .take(3)
        //     .collect::<String>()
        //     .parse::<u16>()?;
        // let name = entry
        //     .path()
        //     .file_name()
        //     .unwrap_or_default()
        //     .to_string_lossy()
        //     .to_string();
        if bucket >= 2 {
            if dry_run {
                println!("Would archive directory: {}", entry.path().display());
            } else {
                let (mut archive, archive_name) = create_archive(entry.path().to_string_lossy().as_ref(), year)?;
                let Some(file_name) = archive_name.file_name() else {
                    eprintln!("Failed to determine file name for archive '{}', skipping", archive_name.display());
                    continue;
                };

                let mut writer = IoWrapper(Sha256::new());
                io::copy(&mut archive, &mut writer)?;
                let hsh: [u8; 32] = writer.0.finalize().into();

                let p = adir.join(bucket.to_string());
                fs::create_dir_all(&p)?;
                fs::copy(&archive_name, p.join(file_name))?;
                fs::remove_file(&archive_name)?;
                fs::remove_dir_all(entry.path())?;

                // conn.execute(
                // "INSERT INTO archive (year, no, name, change_time, hash) VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(year, no) DO UPDATE SET change_time = excluded.change_time, hash = excluded.hash",
                // (year, no, name, latest.duration_since(UNIX_EPOCH)?.as_secs_f64(), hsh)
                // )?;
                conn.execute(
                "INSERT INTO archive (name, change_time, hash) VALUES (?1, ?2, ?3) ON CONFLICT(name) DO UPDATE SET change_time = excluded.change_time, hash = excluded.hash",
                (archive_name.file_prefix().unwrap_or_default().to_string_lossy(), latest.duration_since(UNIX_EPOCH)?.as_secs_f64(), hsh)
                )?;
            }
        } else {
            if dry_run {
                println!(
                    "Would execute SQL: INSERT INTO archive (name, change_time, hash) VALUES ({}, {}, NULL) ON CONFLICT(name) DO UPDATE SET change_time = excluded.change_time, hash = excluded.hash",
                    format!("{}_{}", year, dir_name),
                    latest.duration_since(UNIX_EPOCH)?.as_secs_f64()
                );
            } else {
                // conn.execute(
                // "INSERT INTO archive (year, no, name, change_time, hash) VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(year, no) DO UPDATE SET change_time = excluded.change_time, hash = excluded.hash",
                // (year, no, name, latest.duration_since(UNIX_EPOCH)?.as_secs_f64(), Null)
                // )?;
                conn.execute(
                "INSERT INTO archive (name, change_time, hash) VALUES (?1, ?2, ?3) ON CONFLICT(name) DO UPDATE SET change_time = excluded.change_time, hash = excluded.hash",
                (format!("{}_{}", year, dir_name), latest.duration_since(UNIX_EPOCH)?.as_secs_f64(), Null)
                )?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::starts_with_three_digits;

    #[test]
    fn matches_names_starting_with_three_digits() {
        assert!(starts_with_three_digits("123"));
        assert!(starts_with_three_digits("123abc"));
        assert!(!starts_with_three_digits("12"));
        assert!(!starts_with_three_digits("12a"));
        assert!(!starts_with_three_digits("ab3"));
    }
}
