pub struct Endpoint {
    pub host: String,
}

impl Endpoint {
    pub fn url(&self, path: &str) -> String {
        format!("http://{}/{}", self.host, path)
    }
}
