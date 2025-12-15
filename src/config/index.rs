use crate::state::index::AppState;
use dotenv::{dotenv, from_filename};
use once_cell::sync::OnceCell;
use std::env;

pub static CONFIG: OnceCell<AppState> = OnceCell::new();

impl AppState {
    pub fn from_env() -> Self {
        dotenv().ok();
        from_filename("local.env").ok();
        AppState {
            s_redis_url: env::var("S_REDIS_URL").expect("S_REDIS_URL Missing in .env"),
            db_url: env::var("DB_URL").expect("DB_URL Missing in .env"),
            port: env::var("PORT").expect("PORT Missing in .env"),
            jwt_key: env::var("JWT_KEY").expect("PORT Missing in .env"),
        }
    }
}

pub async fn init_config() {
    CONFIG
        .set(AppState::from_env())
        .expect("Config already set");
}
