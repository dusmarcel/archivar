use std::{env, fs::File};
use std::path::Path;

use tar::Builder;
use xz2::write::XzEncoder;

pub fn create_archive(name: &str) -> Result<File, Box<dyn std::error::Error>> {
    let path = Path::new(name);
    let parent = path.parent().unwrap_or(Path::new("."));
    let dir_name = path
        .file_name()
        .ok_or("directory path must have a final path component")?;
    let archive_path = parent.join(format!("{}/{}.tar.xz", env::temp_dir().to_string_lossy(), dir_name.to_string_lossy()));
    println!("Creating archive at: {}", archive_path.display());
    let archive_file = File::create(&archive_path).unwrap();
    let encoder = XzEncoder::new(&archive_file, 6);
    let mut builder = Builder::new(encoder);
    builder.append_dir_all(dir_name, path)?;
    let encoder = builder.into_inner()?;
    encoder.finish()?;

    Ok(archive_file)
}
