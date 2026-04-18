use std::{fs, path::PathBuf};

use anyhow::Result;
use rusqlite::Connection;

use crate::archive_year_dir::archive_year_dir;

fn is_year_dir(dir_name: &str) -> bool {
    dir_name.len() == 2 && dir_name.chars().all(|c| c.is_ascii_digit())
}

pub fn archive_top_dir(
    dir: PathBuf,
    adir: &PathBuf,
    dry_run: bool,
    remove: bool,
    conn: &Connection,
) -> Result<()> {
    if dry_run {
        println!("dry run!");
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.path().file_name() {
                if let Some(name) = name.to_str() {
                    if is_year_dir(name) {
                        archive_year_dir(entry.path(), adir, dry_run, remove, conn)?;
                    }
                }
            }
        }
    }

    Ok(())
}
