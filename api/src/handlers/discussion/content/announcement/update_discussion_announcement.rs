use crate::models::announcement_struct::DiscussionAnnouncement;
use crate::models::Discussion_struct::UpdateDiscussionAnnouncementPayload;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a specific announcement for a discussion.
pub async fn update_discussion_announcement(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    payload: web::Json<UpdateDiscussionAnnouncementPayload>,
) -> impl Responder {
    let (discussion_id, announcement_id) = path.into_inner();

    let result = sqlx::query_as::<_, DiscussionAnnouncement>(
        r#"
        UPDATE announcements
        SET title = COALESCE($1, title), content = COALESCE($2, content)
        WHERE id = $3 AND discussion_id = $4
        RETURNING id, title, content, created_by, discussion_id
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(announcement_id)
    .bind(discussion_id)
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(announcement)) => HttpResponse::Ok().json(announcement),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Announcement not found for this discussion."})),
        Err(e) => {
            eprintln!("Failed to update discussion announcement: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update announcement."}))
        }
    }
}
