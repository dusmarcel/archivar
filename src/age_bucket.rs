use std::time::{Duration, SystemTime};
use chrono::{Datelike, Local, TimeZone};

pub fn age_bucket(latest: SystemTime, now: SystemTime) -> u8 {
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
        8
    } else if january_age > YEAR * 6 {
        6
    } else if age > YEAR * 4 {
        4
    } else if age > YEAR * 2 {
        2
    } else {
        0
    }
}