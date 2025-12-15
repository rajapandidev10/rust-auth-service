use crate::config::index::CONFIG;
use crate::redis::ops::*;
use crate::schema::user::User;
use crate::utils::is_empty::*;
use crate::utils::jwt::*;
use crate::utils::password::*;
use axum::{Json, response::IntoResponse};
use mongodb::bson::doc;
use serde_json::json;

pub async fn get_users() -> impl IntoResponse {
    let collection = User::collection();

    match collection
        .find(doc! {})
        .projection(doc! {
            "email": 1,
            "name": 1,
            "_id": 0
        })
        .await
    {
        Ok(mut cursor) => {
            let mut users = Vec::new();

            while cursor.advance().await.unwrap_or(false) {
                let user: User = cursor.deserialize_current().unwrap();
                users.push(user);
            }

            Json(json!({
                "success": true,
                "count": users.len(),
                "users": users
            }))
        }
        Err(err) => {
            eprintln!("❌ Error fetching users: {:?}", err);
            Json(json!({
                "success": false,
                "error": "Failed to fetch users"
            }))
        }
    }
}

pub async fn register(Json(mut body): Json<User>) -> impl IntoResponse {
    let user = User::collection();

    let exist = user.find_one(doc! { "email": &body.email }).await.unwrap();
    println!("Existing user check: {:?}", json!(exist));

    if !is_empty(&json!(exist)) {
        return Json(json!({ "success": false, "message": "User already exists" }));
    }

    body = User {
        password: hash_password(&body.password),
        ..body
    };

    let insert_id = match user.insert_one(&body).await {
        Ok(result) => result.inserted_id,
        Err(e) => {
            eprintln!("❌ Error inserting user: {:?}", e);
            return Json(json!({
                "success": false,
                "message": "Failed to register user"
            }));
        }
    };
    let _ = hset("user", &insert_id.to_string(), &body).await;

    if is_empty(&json!(insert_id)) {
        return Json(json!({ "success": false, "message": "Failed to register user" }));
    }

    return Json(json!({ "success": true, "message": "User registered successfully" }));
}

pub async fn login(Json(body): Json<User>) -> impl IntoResponse {
    let user = User::collection();

    let exist = user.find_one(doc! { "email": &body.email }).await.unwrap();

    let existing_user = match exist {
        Some(doc) => doc,
        None => {
            println!("User not found");
            return Json(json!({
                "success": false,
                "message": "User not found"
            }));
        }
    };

    let check_password: bool = verify_password(&body.password, &existing_user.password);

    if !check_password {
        return Json(json!({ "success": false, "message": "Invalid email or password" }));
    }

    let auth_token = generate_jwt(&existing_user.email, &CONFIG.get().unwrap().jwt_key.clone());

    if is_empty(&json!(auth_token)) {
        return Json(json!({ "success": false, "message": "Error on server" }));
    }

    return Json(json!({ "success": true,
    "message": "User login successfully",
    "data":{"token":format!("Bearer {}",auth_token) ,"email":existing_user.email}
    }));
}
