use crate::redis::index::REDIS_CLIENT;
use fred::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

pub async fn hset<T>(key: &str, field: &str, value: &T) -> Result<(), RedisError>
where
    T: Serialize,
{
    let client = REDIS_CLIENT.get().unwrap().clone();
    let json = serde_json::to_string(value)
        .map_err(|e| RedisError::new(RedisErrorKind::InvalidArgument, e.to_string()))?;

    client.hset::<i64, _, _>(key, vec![(field, json)]).await?;
    Ok(())
}

pub async fn hget<T>(key: &str, field: &str) -> Result<Option<T>, RedisError>
where
    T: DeserializeOwned,
{
    let client = REDIS_CLIENT.get().unwrap().clone();
    let val: Option<String> = client.hget(key, field).await?;

    match val {
        Some(json) => {
            let obj: T = serde_json::from_str(&json)
                .map_err(|e| RedisError::new(RedisErrorKind::InvalidArgument, e.to_string()))?;
            Ok(Some(obj))
        }
        None => Ok(None),
    }
}

pub async fn hgetall<T>(key: &str) -> Result<Vec<T>, RedisError>
where
    T: DeserializeOwned,
{
    let client = REDIS_CLIENT.get().unwrap().clone();
    let map: HashMap<String, String> = client.hgetall(key).await?;

    let mut result = Vec::new();

    for (_, json_str) in map {
        let obj: T = serde_json::from_str(&json_str)
            .map_err(|e| RedisError::new(RedisErrorKind::InvalidArgument, e.to_string()))?;

        result.push(obj);
    }

    Ok(result)
}
