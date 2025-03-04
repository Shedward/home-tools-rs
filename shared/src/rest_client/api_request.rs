use crate::rest_client::*;

pub trait ApiRequest {
    fn request(&self) -> request::Request;
}
