use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize};
use serde_json::json;
use sqlx::PgPool;

use super::list_users::UserInfo;

/// Payload for updating a user.
/// All fields are optional.
#[derive(Deserialize)]
pub struct UserUpdatePayload {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Handler to update a user by ID.
pub async fn update_user(
    db_pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    payload: web::Json<UserUpdatePayload>,
) -> impl Responder {
    // For a partial update, we should fetch the current user first,
    // but for simplicity, this example only updates non-null fields.
    // A more robust solution would use COALESCE in SQL or fetch-then-update logic.
    // Note: This query will fail if you try to update a username to one that already exists.
    match sqlx::query_as::<_, UserInfo>(
        "UPDATE users SET username = COALESCE($1, username), email = COALESCE($2, email) WHERE id = $3 RETURNING id, username, email"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(*user_id)
    .fetch_optional(&**db_pool)
    .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": format!("User with id {} not found", *user_id)})),
        Err(e) => {
            eprintln!("Failed to update user: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update user."}))
        }
    }
}