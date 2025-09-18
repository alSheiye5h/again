use crate::models::announcement_struct::DiscussionAnnouncement;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a specific announcement for a discussion.
pub async fn get_discussion_announcement(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (discussion_id, announcement_id) = path.into_inner();

    let result = sqlx::query_as::<_, DiscussionAnnouncement>(
        r#"
        SELECT id, title, content, created_by, discussion_id
        FROM announcements
        WHERE discussion_id = $1 AND id = $2
        "#,
    )
    .bind(discussion_id)
    .bind(announcement_id)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(announcement) => HttpResponse::Ok().json(announcement),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Announcement not found for this discussion."})),
        Err(e) => {
            eprintln!("Failed to get discussion announcement: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to retrieve discussion announcement."}))
        }
    }
}
