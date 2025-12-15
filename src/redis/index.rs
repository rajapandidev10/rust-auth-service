use crate::config::index::CONFIG;
use fred::prelude::*;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub static REDIS_CLIENT: OnceCell<Arc<RedisClient>> = OnceCell::const_new();

pub async fn init_redis() -> Result<(), RedisError> {
    let redis_url: String = CONFIG.get().unwrap().s_redis_url.clone();

    let config: RedisConfig = RedisConfig::from_url(&redis_url)?;

    let client: Arc<RedisClient> = Arc::new(RedisClient::new(config, None, None, None));

    client.connect();
    client.wait_for_connect().await?;

    println!("Redis connected successfully!");

    REDIS_CLIENT
        .set(client)
        .map_err(|_| RedisError::new(RedisErrorKind::Config, "Redis already initialized"))
}
