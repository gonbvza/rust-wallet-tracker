extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, UNIX_EPOCH};

pub fn epoch_converter(date: String) -> String {
    let epoch_date: u64 = date.parse::<u64>().unwrap();
    let d = UNIX_EPOCH + Duration::from_secs(epoch_date);
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    timestamp_str
}
