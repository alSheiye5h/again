use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DiscussionMemberInfo {
    pub user_id: i32,
    pub username: String,
}

/// Handler to list all members of a specific discussion.
pub async fn list_discussion_members(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as::<_, DiscussionMemberInfo>(
        r#"
        SELECT u.id as user_id, u.username
        FROM users u
        JOIN discussion_members dm ON u.id = dm.user_id
        WHERE dm.discussion_id = $1
        "#,
    )
    .bind(discussion_id.into_inner())
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => {
            eprintln!("Failed to list discussion members: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list members."}))
        }
    }
}
