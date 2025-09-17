use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to make a user unfollow another.
/// This is initiated by the `follower_id` to stop following the `followed_id`.
/// The route would look like /users/{follower_id}/following/{followed_id}
pub async fn delete_following(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (follower_id, followed_id)
) -> impl Responder {
    let (follower_id, followed_id) = path.into_inner();

    let result = sqlx::query("DELETE FROM user_relationship WHERE follower = $1 AND followed = $2")
        .bind(follower_id)
        .bind(followed_id)
        .execute(&**db_pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "User unfollowed successfully."}))
        }
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Follow relationship not found."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to unfollow user: {}", e)})),
    }
}