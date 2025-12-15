use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AppState {
    pub s_redis_url: String,
    pub db_url: String,
    pub port: String,
    pub jwt_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
