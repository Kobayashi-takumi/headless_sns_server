use chrono::{DateTime, Utc};

pub fn now() -> u64 {
    let now: DateTime<Utc> = Utc::now();
    now.timestamp() as u64
}
