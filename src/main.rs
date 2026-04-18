use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::fs;

use archivar::archive_top_dir::archive_top_dir;

fn find_directory(p :&PathBuf, name: &str) -> Result<Option<std::path::PathBuf>> {
    for entry in fs::read_dir(p)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && entry.file_name().to_string_lossy() == name {
            return Ok(Some(entry.path()));
        }
    }

    Ok(None)
}

fn main() -> Result<()> {
    let matches = clap::Command::new("Archivar")
        .version("0.1.0")
        .author("Marcel Keienborg <marcel@keienb.org>")
        .about("A simple archiving tool for my directories")
        .arg(
            clap::Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .action(clap::ArgAction::SetTrue)
                .help("Just writing what I would do, but not actually doing it"),
        )
        .arg(
            clap::Arg::new("remove-empty-dirs")
                .short('r')
                .long("remove")
                .action(clap::ArgAction::SetTrue)
                .help("Remove empty directories"),
        )
        .arg(
            clap::Arg::new("path")
                .short('p')
                .long("path")
                .action(clap::ArgAction::Append)
                .value_name("PATH")
                .help("Path to the directory to archive (defaults to current directory)")
        )
        .get_matches();
    let d = matches.get_flag("dry-run");
    let r = matches.get_flag("remove-empty-dirs");
    let p = match matches.get_one::<String>("path") {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir().context("failed to determine current directory")?,
    };

    if !p.is_dir() {
        anyhow::bail!("Provided path '{}' has to be a directory, but it does'nt seem to be one", p.display());
    }

    let conn = Connection::open(p.join("archivar.db"))?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS archive (
            year INTEGER NOT NULL,
            no INTEGER NOT NULL,
            name TEXT NOT NULL,
            change_time REAL NOT NULL,
            hash BLOB UNIQUE CHECK (length(hash) = 32),
            PRIMARY KEY (year, no)
        )",
        (),
    )?;

    if let Some(kanzlei_dir) = find_directory(&p, "kanzlei")? {
        archive_top_dir(kanzlei_dir, d, r, &conn)?;
    } else {
        println!("Directory 'kanzlei' not found, skipping.");
    }

    if let Some(ablage_dir) = find_directory(&p, "ablage")? {
        for bucket in ["2", "4", "6", "8"] {
            let bucket_dir = ablage_dir.join(bucket);
            if bucket_dir.is_dir() {
                archive_top_dir(bucket_dir, d, r, &conn)?;
            } else {
                println!("Subdirectory '{}' not found in 'ablage', skipping.", bucket);
            }
        }
    } else {
        println!("Directory 'ablage' not found, skipping.");
    }

    Ok(())
}
