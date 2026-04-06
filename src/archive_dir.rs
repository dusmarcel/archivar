use std::fs;
use std::fs::File;
use std::path::Path;

use tar::Builder;
use xz2::write::XzEncoder;

pub fn archive_dir(name: &str, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    if dry_run {
        println!("Would archive directory: {}", name);
    } else {
        let path = Path::new(name);
        let parent = path.parent().unwrap_or(Path::new("."));
        let dir_name = path
            .file_name()
            .ok_or("directory path must have a final path component")?;
        let archive_path = parent.join(format!("{}.tar.xz", dir_name.to_string_lossy()));
        let archive_file = File::create(&archive_path)?;
        let encoder = XzEncoder::new(archive_file, 6);
        let mut builder = Builder::new(encoder);
        builder.append_dir_all(dir_name, path)?;
        let encoder = builder.into_inner()?;
        encoder.finish()?;

        fs::remove_dir_all(path)?;
        println!("Archived and removed directory: {}", name);
    }

    Ok(())
}
