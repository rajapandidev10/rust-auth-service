use crate::config::index::CONFIG;
use mongodb::{Client, Database, options::ClientOptions};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::OnceCell;
pub static DB_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

pub async fn init_db() -> Result<(), Box<dyn Error>> {
    let uri = CONFIG.get().unwrap().db_url.clone();

    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("rust_auth".to_string());

    let client = Client::with_options(client_options)?;

    DB_CLIENT
        .set(Arc::new(client))
        .map_err(|_| "db already initialized")?;

    let db_client = DB_CLIENT.get().unwrap();

    db_client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 })
        .await?;

    println!("✅ Connected to MongoDB!");
    Ok(())
}

pub fn get_db() -> Database {
    let db_client = DB_CLIENT.get().unwrap();
    db_client.database("rust_auth")
}
