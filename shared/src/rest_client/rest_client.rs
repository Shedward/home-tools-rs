use super::request;
use super::response;
use super::response::Response;
use super::ApiRequest;
use crate::tools::Loading;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub enum Error {
    NetworkFailed(String),
    WrongResponse(String),
    SerializationFailed(String),
}

pub type ResponseHandler = Box<dyn 'static + Send + FnOnce(Result<response::Response, Error>)>;
pub type LoadingResponse = Arc<Mutex<Loading<Response, Error>>>;
pub type LoadingValue<T> = Arc<Mutex<Loading<T, Error>>>;

pub trait RestClient: Send + Sync {
    fn fetch(&self, request: request::Request, on_done: ResponseHandler);

    #[inline]
    fn api_request(&self, request: &dyn ApiRequest, on_done: ResponseHandler) {
        self.fetch(request.request(), on_done);
    }
}

#[cfg(feature = "ehttp")]
pub mod ehttp_client {
    use crate::rest_client::{endpoint::Endpoint, response::ToResponse};

    use super::*;
    pub struct Client {
        pub endpoint: Endpoint,
    }

    impl RestClient for Client {
        fn fetch(&self, request: request::Request, on_done: ResponseHandler) {
            let r = request.into_request(&self.endpoint);
            ehttp::fetch(r, move |result| {
                on_done(
                    result
                        .map(|r| r.to_response())
                        .map_err(|e| Error::NetworkFailed(e)),
                )
            });
        }
    }
}

pub trait DefaultResponseHandlers {
    fn loading_response(loading_response: LoadingResponse) -> ResponseHandler;

    fn loading_map<T, U>(
        loading_value: LoadingValue<U>,
        map_fn: impl 'static + Send + FnOnce(T) -> U,
    ) -> ResponseHandler
    where
        T: serde::de::DeserializeOwned + Send + Sync + 'static,
        U: Send + 'static;

    fn loading<T>(loading_value: LoadingValue<T>) -> ResponseHandler
    where
        T: serde::de::DeserializeOwned + Send + Sync + 'static,
    {
        Self::loading_map(loading_value, |v| v)
    }
}

impl DefaultResponseHandlers for ResponseHandler {
    fn loading_response(loading_response: LoadingResponse) -> ResponseHandler {
        let mut locked_response = loading_response.lock().unwrap();
        *locked_response = Loading::Loading;

        let cloned_response = loading_response.clone();
        Box::new(move |r| {
            let mut locked_response = cloned_response.lock().unwrap();
            *locked_response = match r {
                Ok(response) => Loading::Loaded(response),
                Err(error) => Loading::Failed(error),
            };
        })
    }

    fn loading_map<T, U>(
        loading_value: LoadingValue<U>,
        map_fn: impl 'static + Send + FnOnce(T) -> U,
    ) -> ResponseHandler
    where
        T: serde::de::DeserializeOwned + Send + Sync + 'static,
        U: Send + 'static,
    {
        let mut locked_value = loading_value.lock().unwrap();
        *locked_value = Loading::Loading;

        let cloned_value = loading_value.clone();
        Box::new(move |r| {
            let new_value = match r {
                Ok(response) => Loading::from_result(response.try_body().map(map_fn)),
                Err(error) => Loading::Failed(error),
            };

            let mut locked_value = cloned_value.lock().unwrap();
            *locked_value = new_value;
        })
    }
}
