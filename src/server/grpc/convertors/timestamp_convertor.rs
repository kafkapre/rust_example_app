use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

const UTC_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";

/// E.g. "2017-12-17 18:01:00" or "2017-12-17 18:01:00.000"
pub fn utc_from_str(str: impl AsRef<str>) -> Result<DateTime<Utc>> {
  NaiveDateTime::parse_from_str(str.as_ref(), UTC_FORMAT)
    .map(|date_time| DateTime::<Utc>::from_utc(date_time, Utc))
    .map_err(|_| anyhow!("parse utc failed - invalid input '{}'", str.as_ref()))
}