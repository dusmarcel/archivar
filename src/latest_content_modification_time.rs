use std::time::SystemTime;

use anyhow::Result;
use walkdir::WalkDir;

pub fn latest_content_modification_time(
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