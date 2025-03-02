use super::endpoint::Endpoint;

#[derive(Clone, Debug)]
pub struct Method(pub &'static str);

impl Method {
    pub const GET: Method = Method("GET");
    pub const POST: Method = Method("POST");
    pub const PUT: Method = Method("PUT");
    pub const DELETE: Method = Method("DELETE");
}

#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn request(method: Method, path: String) -> Self {
        Self {
            method,
            path,
            query: Vec::new(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn get(path: String) -> Self {
        Self::request(Method::GET, path)
    }

    pub fn post(path: String) -> Self {
        Self::request(Method::POST, path)
    }

    pub fn put(path: String) -> Self {
        Self::request(Method::PUT, path)
    }

    pub fn delete(path: String) -> Self {
        Self::request(Method::DELETE, path)
    }

    pub fn query(mut self, key: &str, value: &str) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    pub fn query_opt(mut self, key: &str, value: Option<String>) -> Self {
        if let Some(value) = value {
            self.query.push((key.into(), value));
        }
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn header_opt(mut self, key: &str, value: Option<String>) -> Self {
        if let Some(value) = value {
            self.headers.push((key.into(), value.into()));
        }
        self
    }

    pub fn body_data(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn try_body<Body>(mut self, body: Body) -> Result<Self, serde_json::Error>
    where
        Body: serde::ser::Serialize,
    {
        let data = serde_json::to_vec(&body)?;
        self.body = Some(data);
        Ok(self)
    }

    pub fn body<Body>(mut self, body: Body) -> Self
    where
        Body: serde::ser::Serialize,
    {
        let data = serde_json::to_vec(&body).unwrap();
        self.body = Some(data);
        self
    }

    pub fn into_request<RequestType>(self, endpoint: &Endpoint) -> RequestType
    where
        RequestType: FromRequest,
    {
        RequestType::from_request(self.clone(), endpoint)
    }
}

pub trait FromRequest {
    fn from_request(request: Request, endpoint: &Endpoint) -> Self;
}

#[cfg(feature = "ehttp")]
impl FromRequest for ehttp::Request {
    fn from_request(request: Request, endpoint: &Endpoint) -> Self {
        Self {
            method: request.method.0.to_string(),
            url: endpoint.url(&request.path),
            body: request.body.unwrap_or(vec![]),
            headers: ehttp::Headers {
                headers: request.headers,
            },
        }
    }
}
