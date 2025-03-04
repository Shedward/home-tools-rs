use crate::{rest_client::*, tools::DisplayingName};
use serde::Deserialize;

#[derive(Debug)]
pub struct GetOnlineDevices;

impl GetOnlineDevices {
    pub fn new() -> Self {
        GetOnlineDevices
    }
}

impl ApiRequest for GetOnlineDevices {
    fn request(&self) -> request::Request {
        request::Request::get("api/online/devices".into())
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct OnlineDevice {
    pub address: String,
    pub mac: String,
    pub host: Option<String>,
}

impl DisplayingName for OnlineDevice {
    fn display_name(&self) -> String {
        match &self.host {
            Some(host) => host.clone(),
            None => self.address.clone(),
        }
    }
}
