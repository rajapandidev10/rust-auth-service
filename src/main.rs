mod config;
mod controller;
mod db;
mod redis;
mod routes;
mod schema;
mod socket;
mod state;
mod utils;
mod validator;

use config::index::CONFIG;
use config::index::init_config;
use db::index::init_db;
use redis::index::init_redis;
use routes::app_routes;
use socket::index::init_socket;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_config().await;
    let _ = init_db().await;
    let _ = init_redis().await;
    let layer = init_socket().await?;

    println!("Server listening at {}", CONFIG.get().unwrap().port.clone());
    let app = app_routes().layer(layer);
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", CONFIG.get().unwrap().port.clone()))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
    tokio::signal::ctrl_c().await.unwrap();
    println!("Shutting down...");
    Ok(())
}
