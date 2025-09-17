use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::handlers::user::handle::list_users::UserInfo;

/// Handler to list all followers of a user.
pub async fn list_followers(db_pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as::<_, UserInfo>(
        r#"
        SELECT u.id, u.username, u.email FROM users u
        INNER JOIN user_relationship ur ON u.id = ur.follower
        WHERE ur.followed = $1
        "#,
    )
    .bind(*user_id)
    .fetch_all(&**db_pool)
    .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to fetch followers: {}", e)})),
    }
}
