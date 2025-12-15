use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;

pub async fn req_to_json(req: Request<Body>) -> Result<(Request<Body>, Value), StatusCode> {
    let (parts, body) = req.into_parts();

    let bytes = axum::body::to_bytes(body, 1024 * 1024)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let body_string =
        String::from_utf8(bytes.clone().into()).map_err(|_| StatusCode::BAD_REQUEST)?;

    let json_value: Value =
        serde_json::from_str(&body_string).map_err(|_| StatusCode::BAD_REQUEST)?;

    let req = Request::from_parts(parts, Body::from(bytes));

    Ok((req, json_value))
}
