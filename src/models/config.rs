use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    router_host: String,
    db_path: String,
    credentials: Credentials,
}
