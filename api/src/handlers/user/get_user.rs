use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use super::list_users::UserInfo;

/// Handler to get a single user by ID.
pub async fn get_user(db_pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    match sqlx::query_as::<_, UserInfo>("SELECT id, username, email FROM users WHERE id = $1")
        .bind(*user_id)
        .fetch_optional(&**db_pool)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(
            json!({"status": "error", "message": format!("User with id {} not found", *user_id)}),
        ),
        Err(e) => {
            eprintln!("Failed to fetch user: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to fetch user."}))
        }
    }
}