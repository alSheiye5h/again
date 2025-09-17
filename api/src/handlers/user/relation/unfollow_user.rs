use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

// TODO: Import a struct that can be extracted from the JWT token for authorization checks.

/// Handler to make one user unfollow another.
/// The `follower_id` is the one performing the action (and should be checked against auth).
/// The `followed_id` is the user they want to unfollow.
pub async fn unfollow_user(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (followed_id, follower_id)
) -> impl Responder {
    let (followed_id, follower_id) = path.into_inner();

    let result = sqlx::query("DELETE FROM user_relationship WHERE follower = $1 AND followed = $2")
        .bind(follower_id)
        .bind(followed_id)
        .execute(&**db_pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(json!({"status": "success", "message": "User unfollowed successfully."})),
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Relationship not found."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to unfollow user: {}", e)})),
    }
}
