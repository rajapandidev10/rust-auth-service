use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::controller::user::*;
use crate::validator::auth::*;
use crate::validator::jwt::*;

pub fn app_routes() -> Router {
    Router::new()
        .route(
            "/signup",
            post(register).layer(middleware::from_fn(signup_validate)),
        )
        .route(
            "/sigin",
            post(login).layer(middleware::from_fn(signin_validate)),
        )
        .route("/list", get(get_users).layer(middleware::from_fn(passport)))
}
