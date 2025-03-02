use super::rest_client;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: u16, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        Response {
            status,
            headers,
            body,
        }
    }

    pub fn try_body<Body>(self) -> Result<Body, rest_client::Error>
    where
        Body: serde::de::DeserializeOwned,
    {
        serde_json::from_slice(&self.body)
            .map_err(|e| rest_client::Error::SerializationFailed(e.to_string()))
    }
}

pub trait ToResponse {
    fn to_response(self) -> Response;
}

#[cfg(feature = "ehttp")]
impl ToResponse for ehttp::Response {
    fn to_response(self) -> Response {
        Response {
            status: self.status,
            headers: self.headers.headers,
            body: self.bytes,
        }
    }
}
