use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to make one user follow another.
/// The follower_id is the one performing the action (e.g., from auth token),
/// and the followed_id is the user they want to follow.
pub async fn follow_user(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (follower_id, followed_id)
) -> impl Responder {
    let (follower_id, followed_id) = path.into_inner();

    if follower_id == followed_id {
        return HttpResponse::BadRequest().json(json!({"status": "error", "message": "User cannot follow themselves."}));
    }

    // Using ON CONFLICT DO NOTHING to avoid errors if the relationship already exists.
    let result = sqlx::query("INSERT INTO user_relationship (follower, followed) VALUES ($1, $2) ON CONFLICT (follower, followed) DO NOTHING")
        .bind(follower_id)
        .bind(followed_id)
        .execute(&**db_pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Created().json(json!({"status": "success", "message": "User followed successfully."})),
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "User is already being followed."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to follow user: {}", e)})),
    }
}

