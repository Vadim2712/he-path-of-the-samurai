use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use serde_json::Value;

/// Picks the first non-empty string value from a JSON object using a list of possible keys.
pub fn s_pick(v: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(s) = x.as_str() {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            } else if x.is_number() {
                return Some(x.to_string());
            }
        }
    }
    None
}

/// Picks and parses the first valid timestamp from a JSON object using a list of possible keys.
/// Handles RFC3339, "Y-m-d H:M:S", and Unix timestamps.
pub fn t_pick(v: &Value, keys: &[&str]) -> Option<DateTime<Utc>> {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(s) = x.as_str() {
                if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                    return Some(dt);
                }
                if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                    return Some(Utc.from_utc_datetime(&ndt));
                }
            } else if let Some(n) = x.as_i64() {
                if let Some(dt) = Utc.timestamp_opt(n, 0).single() {
                    return Some(dt);
                }
            }
        }
    }
    None
}

/// Returns a tuple of (start_date, end_date) as strings for the last N days.
pub fn last_days(n: i64) -> (String, String) {
    let to = Utc::now().date_naive();
    let from = to - chrono::Duration::days(n);
    (from.to_string(), to.to_string())
}
