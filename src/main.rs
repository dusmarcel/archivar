use std::fs;
use rusqlite::Connection;

use archivar::archive_top_dir::archive_top_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::Command::new("Archivar")
        .version("0.1.0")
        .author("Marcel Keienborg <marcel@keienb.org>")
        .about("A simple archiving tool for my directories")
        .arg(clap::Arg::new("dry-run")
            .short('d')
            .long("dry-run")
            .action(clap::ArgAction::SetTrue)
            .help("Just writing what I would do, but not actually doing it")
        )
        .arg(clap::Arg::new("remove-empty-dirs")
            .short('r')
            .long("remove")
            .action(clap::ArgAction::SetTrue)
            .help("Remove empty directories")
        )
        .get_matches();
    let d = matches.get_flag("dry-run");
    let r = matches.get_flag("remove-empty-dirs");

    let conn = Connection::open("archivar.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS archive (
            year INTEGER NOT NULL,
            no INTEGER NOT NULL,
            change_time REAL NOT NULL,
            hash INTEGER UNIQUE,
            PRIMARY KEY (year, no)
        )",
        (),
    )?;

    for dir in ["kanzlei", "ablage2", "ablage4", "ablage6", "ablage8"] {
        let mut found = false;
        for entry in fs::read_dir(".")? {
            if let Ok(entry) = entry {
                if entry.file_type()?.is_dir() {
                    if entry.file_name().to_string_lossy() == dir {
                        found = true;
                        archive_top_dir(entry.path(), d, r, &conn)?;
                        break;
                    }
                }
            }
        }

        if !found {
            println!("Directory '{}' not found, skipping.", dir);
        }
    }

    Ok(())
}
