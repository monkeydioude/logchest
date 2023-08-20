use std::time::{SystemTime, UNIX_EPOCH};

use rocket::serde::json::serde_json::{to_string, Result};
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Debug, Deserialize, Serialize)]
pub struct Log {
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub date: u128,
    #[serde(skip_deserializing)]
    pub human_date: String,
}

impl Log {
    pub fn push_log(&mut self) -> Result<String> {
        self.date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        to_string(&self)
    }

    pub fn read_nomalizing(&self) -> Log {
        Log {
            msg: self.msg.clone(),
            date: self.date,
            human_date: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(
                    (self.date / 1000) as i64, ((self.date % 1000) * 1000) as u32
                ).unwrap(),
                Utc,
            ).format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
