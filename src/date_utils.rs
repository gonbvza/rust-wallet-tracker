extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, UNIX_EPOCH};

/// Converts a Unix timestamp string to a formatted UTC datetime.
pub fn epoch_converter(date: String) -> String {
    let epoch_date: u64 = date.parse::<u64>().unwrap();
    let d = UNIX_EPOCH + Duration::from_secs(epoch_date);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
