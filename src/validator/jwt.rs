use crate::config::index::CONFIG;
use crate::schema::user::User;
use crate::utils::jwt::*;
use axum::{
    Json,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use mongodb::bson::doc;
use serde_json::json;

pub async fn passport(req: Request<Body>, next: Next) -> impl IntoResponse {
    // Get Authorization header
    let auth_header = match req.headers().get("authorization") {
        Some(v) => match v.to_str() {
            Ok(s) => s,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "success": false,
                        "message": "Invalid authorization header format"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": "Authorization header missing"
                })),
            )
                .into_response();
        }
    };

    if !auth_header.starts_with("Bearer ") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "message": "Invalid token format"
            })),
        )
            .into_response();
    }

    let token = auth_header.trim_start_matches("Bearer ").trim();

    let claims = match verify_jwt(token, &CONFIG.get().unwrap().jwt_key.clone()) {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "success": false,
                    "message": "Forbidden — Invalid or expired token"
                })),
            )
                .into_response();
        }
    };
    let user = User::collection();

    let exist = user.find_one(doc! { "email": &claims.sub }).await.unwrap();

    match exist {
        Some(doc) => doc,
        None => {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "success": false,
                    "message": "Authorization failed"
                })),
            )
                .into_response();
        }
    };
    next.run(req).await
}
