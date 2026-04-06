use std::fs;
use std::time::{Duration, SystemTime};

use chrono::{Datelike, Local, TimeZone};
use walkdir::WalkDir;

fn starts_with_three_digits(dir_name: &str) -> bool {
    dir_name.chars().take(3).count() == 3
        && dir_name.chars().take(3).all(|c| c.is_ascii_digit())
}

fn age_bucket(age: Duration) -> &'static str {
    const YEAR: Duration = Duration::from_secs(365 * 24 * 60 * 60);

    if age > YEAR * 4 {
        "letzte Aenderung vor mehr als 4 Jahren"
    } else if age > YEAR * 2 {
        "letzte Aenderung vor mehr als 2 Jahren"
    } else {
        "letzte Aenderung innerhalb der letzten 2 Jahre"
    }
}

fn age_bucket_from_modification_time(latest: SystemTime, now: SystemTime) -> &'static str {
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
        "letzte Aenderung vor mehr als 8 Jahren"
    } else if january_age > YEAR * 6 {
        "letzte Aenderung vor mehr als 6 Jahren"
    } else {
        age_bucket(age)
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

pub fn archive_year_dir(name: &str, dry_run: bool, remove: bool) -> Result<(), Box<dyn std::error::Error>> {
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
            println!(
                "{}: {}",
                entry.path().display(),
                age_bucket_from_modification_time(latest, now)
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use chrono::{Local, TimeZone};

    use super::{age_bucket, age_bucket_from_modification_time, starts_with_three_digits};

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

        assert_eq!(
            age_bucket(year / 2),
            "letzte Aenderung innerhalb der letzten 2 Jahre"
        );
        assert_eq!(
            age_bucket(year * 2),
            "letzte Aenderung innerhalb der letzten 2 Jahre"
        );
        assert_eq!(
            age_bucket(year * 3),
            "letzte Aenderung vor mehr als 2 Jahren"
        );
        assert_eq!(
            age_bucket(year * 4),
            "letzte Aenderung vor mehr als 2 Jahren"
        );
        assert_eq!(
            age_bucket(year * 5),
            "letzte Aenderung vor mehr als 4 Jahren"
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
            age_bucket_from_modification_time(seven_calendar_years_ago_but_after_january, now),
            "letzte Aenderung vor mehr als 6 Jahren"
        );

        let eight_calendar_years_ago_but_after_january = SystemTime::from(
            Local
                .with_ymd_and_hms(2017, 7, 1, 12, 0, 0)
                .single()
                .expect("valid test date"),
        );
        assert_eq!(
            age_bucket_from_modification_time(eight_calendar_years_ago_but_after_january, now),
            "letzte Aenderung vor mehr als 8 Jahren"
        );
    }
}
