use crate::rest_client::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

pub struct OnlineCountersRequest {
    pub from: Option<NaiveDateTime>,
    pub mac: Option<String>,
}

impl OnlineCountersRequest {
    pub fn new() -> Self {
        OnlineCountersRequest {
            from: None,
            mac: None,
        }
    }

    pub fn from(mut self, from: NaiveDateTime) -> Self {
        self.from = Some(from);
        self
    }

    pub fn mac(mut self, mac: String) -> Self {
        self.mac = Some(mac);
        self
    }

    pub fn request(&self) -> request::Request {
        request::Request::get("api/online/counters".into())
            .query_opt("mac", self.mac.clone())
            .query_opt(
                "from",
                self.from.map(|t| t.and_utc().timestamp().to_string()),
            )
    }
}

#[derive(Deserialize, Debug)]
pub struct OnlineCounter {
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub start: DateTime<Utc>,
    pub count: u32,
}
