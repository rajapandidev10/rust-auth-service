use crate::utils::{general::*, is_empty::*, json_convert::*};
use axum::{
    Json,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use serde_json::json;

pub async fn signup_validate(req: Request<Body>, next: Next) -> impl IntoResponse {
    let (req, json_value) = match req_to_json(req).await {
        Ok(v) => v,
        Err(status) => return status.into_response(),
    };

    if is_empty(&json_value["name"]) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Name field is required",
            })),
        )
            .into_response();
    }

    if is_empty(&json_value["email"]) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Email field is required",
            })),
        )
            .into_response();
    } else if !is_valid_email(&json_value["email"].to_string()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Enter a valid email",
            })),
        )
            .into_response();
    }

    if is_empty(&json_value["password"]) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Passowrd field is required",
            })),
        )
            .into_response();
    } else if !is_valid_password(&json_value["password"].to_string()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Password should contain atleast one uppercase, atleast one lowercase, atleast one number, atleast one special character and minimum 6 and maximum 18 characters",
            })),
        )
            .into_response();
    } else if json_value["password"].to_string() != json_value["confirm_password"].to_string() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Confirm Password does not match",
            })),
        )
            .into_response();
    }

    next.run(req).await
}

pub async fn signin_validate(req: Request<Body>, next: Next) -> impl IntoResponse {
    let (req, json_value) = match req_to_json(req).await {
        Ok(v) => v,
        Err(status) => return status.into_response(),
    };

    if is_empty(&json_value["email"]) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Email field is required",
            })),
        )
            .into_response();
    } else if !is_valid_email(&json_value["email"].to_string()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Enter a valid email",
            })),
        )
            .into_response();
    }

    if is_empty(&json_value["password"]) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Passowrd field is required",
            })),
        )
            .into_response();
    } else if !is_valid_password(&json_value["password"].to_string()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status":400,
                "success": false,
                "message": "Password should contain atleast one uppercase, atleast one lowercase, atleast one number, atleast one special character and minimum 6 and maximum 18 characters",
            })),
        )
            .into_response();
    }
    next.run(req).await
}
