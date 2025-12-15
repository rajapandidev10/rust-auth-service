use crate::db::index::get_db;
use mongodb::{Collection, bson::oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
}

impl User {
    pub fn collection() -> Collection<User> {
        let db: mongodb::Database = get_db();
        db.collection::<User>("users")
    }
}
