//use std::fs::create_dir;
use walkdir::WalkDir;

use archivar::archive_top_dir::archive_top_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kanzlei = false;
    // let mut ablage1 = false;
    // let mut ablage3 = false;
    // let mut ablage6 = false;
    // let mut ablage8 = false;

    let walker = WalkDir::new(".").max_depth(1).into_iter();
    for entry in walker {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    match entry.path().file_name() {
                        Some(name) if name == "kanzlei" => kanzlei = true,
                        // Some(name) if name == "ablage1" => ablage1 = true,
                        // Some(name) if name == "ablage3" => ablage3 = true,
                        // Some(name) if name == "ablage6" => ablage6 = true,
                        // Some(name) if name == "ablage8" => ablage8 = true,
                        _ => (),
                        
                    }
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    if kanzlei {
        archive_top_dir("kanzlei")?;

        // if !ablage1 {
        //     create_dir("ablage1")?
        // }

        // if !ablage3 {
        //     create_dir("ablage3")?
        // }

        // if !ablage6 {
        //     create_dir("ablage6")?
        // }

        // if !ablage8 {
        //     create_dir("ablage8")?
        // }

        Ok(())
    } else {
        Err("Aborting: Directory kanzlei not found. Nothing to archive!".into())
    }
}
