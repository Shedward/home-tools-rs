use crate::rest_client::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

pub struct GetOnlineCounters {
    pub from: Option<NaiveDateTime>,
    pub mac: Option<String>,
}

impl GetOnlineCounters {
    pub fn new() -> Self {
        Self {
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
}

impl ApiRequest for GetOnlineCounters {
    fn request(&self) -> request::Request {
        request::Request::get("api/online/counters".into())
            .query_opt("mac", self.mac.clone())
            .query_opt(
                "from",
                self.from.map(|t| t.and_utc().timestamp().to_string()),
            )
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct OnlineCounter {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub start: DateTime<Utc>,
    pub count: u32,
}
