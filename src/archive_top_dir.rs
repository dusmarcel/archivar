use std::fs;

use crate::archive_year_dir::archive_year_dir;

fn is_year_dir(dir_name: &str) -> bool {
    dir_name.len() == 2 && dir_name.chars().all(|c| c.is_ascii_digit())
}

pub fn archive_top_dir(name: &str, dry_run: bool, remove: bool) -> Result<(), Box<dyn std::error::Error>> {
    if dry_run {
        println!("dry run!");
    }

    for entry in fs::read_dir(name)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if !file_type.is_dir() {
            continue;
        }

        let Some(dir_name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };

        if is_year_dir(&dir_name) {
            archive_year_dir(entry.path().to_string_lossy().as_ref(), dry_run, remove)?;
        }
    }

    Ok(())
}
