use crate::state::index::*;
use chrono::{Duration, Utc};
use jsonwebtoken::*;

pub fn generate_jwt(user_id: &str, secret: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, String> {
    let validation = Validation::default();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(data) => Ok(data.claims),
        Err(err) => Err(format!("Invalid token: {}", err)),
    }
}
