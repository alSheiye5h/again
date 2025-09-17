use crate::models::Announcement_struct::DiscussionAnnouncement;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all announcements for a specific discussion.
pub async fn list_discussion_announcements(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as::<_, DiscussionAnnouncement>(
        r#"
        SELECT id, title, content, created_by, discussion_id
        FROM announcements
        WHERE discussion_id = $1
        "#,
    )
    .bind(discussion_id.into_inner())
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(e) => {
            eprintln!("Failed to list discussion announcements: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list discussion announcements."}))
        }
    }
}
