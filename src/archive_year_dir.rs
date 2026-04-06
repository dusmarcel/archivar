use std::fs;
use std::time::{Duration, SystemTime};

use chrono::{Datelike, Local, TimeZone};
use walkdir::WalkDir;
use rusqlite::Connection;

use crate::archive_dir::archive_dir;

#[derive(Debug, PartialEq, PartialOrd)]
enum Bucket {
    Last2Years,
    MoreThan2Years,
    MoreThan4Years,
    MoreThan6Years,
    MoreThan8Years
}

fn starts_with_three_digits(dir_name: &str) -> bool {
    dir_name.chars().take(3).count() == 3
        && dir_name.chars().take(3).all(|c| c.is_ascii_digit())
}

fn age_bucket(latest: SystemTime, now: SystemTime) -> Bucket {
    const YEAR: Duration = Duration::from_secs(365 * 24 * 60 * 60);

    let age = now.duration_since(latest).unwrap_or(Duration::ZERO);
    let latest_local = chrono::DateTime::<Local>::from(latest);
    let next_january_first = Local
        .with_ymd_and_hms(latest_local.year() + 1, 1, 1, 0, 0, 0)
        .single()
        .expect("valid next January 1 date");
    let january_age = now
        .duration_since(SystemTime::from(next_january_first))
        .unwrap_or(Duration::ZERO);

    if january_age > YEAR * 8 {
        Bucket::MoreThan8Years
    } else if january_age > YEAR * 6 {
        Bucket::MoreThan6Years
    } else if age > YEAR * 4 {
        Bucket::MoreThan4Years
    } else if age > YEAR * 2 {
        Bucket::MoreThan2Years
    } else {
        Bucket::Last2Years
    }
}

fn latest_content_modification_time(
    path: &std::path::Path,
) -> Result<Option<SystemTime>, Box<dyn std::error::Error>> {
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

pub fn archive_year_dir(name: &str, dry_run: bool, remove: bool, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let now = SystemTime::now();

    for entry in fs::read_dir(name)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if !file_type.is_dir() {
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

        if let Some(latest) = latest_content_modification_time(&entry.path())? {
            let bucket = age_bucket(latest, now);

            println!("{}: {:#?}", entry.path().display(), bucket);
            if bucket > Bucket::Last2Years {
                archive_dir(entry.path().to_string_lossy().as_ref(), dry_run)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use chrono::{Local, TimeZone};

    use crate::archive_year_dir::Bucket;

    use super::{age_bucket, starts_with_three_digits};

    #[test]
    fn matches_names_starting_with_three_digits() {
        assert!(starts_with_three_digits("123"));
        assert!(starts_with_three_digits("123abc"));
        assert!(!starts_with_three_digits("12"));
        assert!(!starts_with_three_digits("12a"));
        assert!(!starts_with_three_digits("ab3"));
    }

    #[test]
    fn classifies_age_into_expected_buckets() {
        let year = Duration::from_secs(365 * 24 * 60 * 60);
        let now = SystemTime::now();

        assert_eq!(
            age_bucket(now - year / 2, now),
            Bucket::Last2Years
        );
        assert_eq!(
            age_bucket(now - year * 2, now),
            Bucket::Last2Years
        );
        assert_eq!(
            age_bucket(now - year * 3, now),
            Bucket::MoreThan2Years
        );
        assert_eq!(
            age_bucket(now - year * 4, now),
            Bucket::MoreThan2Years
        );
        assert_eq!(
            age_bucket(now - year * 5, now),
            Bucket::MoreThan4Years
        );
    }

    #[test]
    fn uses_next_january_for_six_and_eight_year_buckets() {
        let now = SystemTime::from(
            Local
                .with_ymd_and_hms(2026, 4, 2, 12, 0, 0)
                .single()
                .expect("valid test date"),
        );

        let seven_calendar_years_ago_but_after_january = SystemTime::from(
            Local
                .with_ymd_and_hms(2019, 7, 1, 12, 0, 0)
                .single()
                .expect("valid test date"),
        );
        assert_eq!(
            age_bucket(seven_calendar_years_ago_but_after_january, now),
            Bucket::MoreThan6Years
        );

        let eight_calendar_years_ago_but_after_january = SystemTime::from(
            Local
                .with_ymd_and_hms(2017, 7, 1, 12, 0, 0)
                .single()
                .expect("valid test date"),
        );
        assert_eq!(
            age_bucket(eight_calendar_years_ago_but_after_january, now),
            Bucket::MoreThan8Years
        );
    }
}
