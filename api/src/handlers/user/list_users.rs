use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;

/// A struct to represent a user's public information.
#[derive(Serialize, sqlx::FromRow)]
pub struct UserInfo {
    id: i32,
    username: String,
    email: String,
}

/// Handler to list all users.
pub async fn list_users(db_pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, UserInfo>("SELECT id, username, email FROM users ORDER BY id")
        .fetch_all(&**db_pool)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Failed to fetch users: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to fetch users."}))
        }
    }
}
