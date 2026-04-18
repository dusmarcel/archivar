use std::path::{Path, PathBuf};
use std::{env, fs::File};

use anyhow::{Result, anyhow};
use tar::Builder;
use xz2::write::XzEncoder;

pub fn create_archive(name: &str, year: u16) -> Result<(File, PathBuf)> {
    let path = Path::new(name);
    let parent = path.parent().unwrap_or(Path::new("."));
    let dir_name = path
        .file_name()
        .ok_or(anyhow!("directory path must have a final path component"))?;
    let archive_path = parent.join(format!(
        "{}/{}_{}__{:x}{:x}.tar.xz",
        env::temp_dir().to_string_lossy(),
        year,
        dir_name.to_string_lossy(),
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos()
    ));
    println!("Creating archive at: {}", archive_path.display());
    let archive_file = File::create(&archive_path).unwrap();
    let encoder = XzEncoder::new(&archive_file, 6);
    let mut builder = Builder::new(encoder);
    builder.append_dir_all(dir_name, path)?;
    let encoder = builder.into_inner()?;
    encoder.finish()?;

    let archive_file = File::open(&archive_path)?;
    Ok((archive_file, archive_path))
}
