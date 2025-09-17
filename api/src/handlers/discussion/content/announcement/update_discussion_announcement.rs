use crate::models::Discussion_struct::{UpdateDiscussionAnnouncementPayload, DiscussionAnnouncement};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a discussion announcement.
pub async fn update_discussion_announcement(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    payload: web::Json<UpdateDiscussionAnnouncementPayload>,
) -> impl Responder {
    let (discussion_id, announcement_id) = path.into_inner();

    // Fetch current announcement to fill in non-updated fields
    let current = match sqlx::query_as::<_, DiscussionAnnouncement>(
        "SELECT * FROM discussion_announcement WHERE id = $1 AND discussion_id = $2",
    )
    .bind(announcement_id)
    .bind(discussion_id)
    .fetch_one(db_pool.get_ref())
    .await
    {
        Ok(ann) => ann,
        Err(_) => return HttpResponse::NotFound().json(json!({"status": "error", "message": "Announcement not found."})),
    };

    let result = sqlx::query_as::<_, DiscussionAnnouncement>(
        "UPDATE discussion_announcement SET title = $1, content = $2, updated_at = NOW() WHERE id = $3 AND discussion_id = $4 RETURNING *",
    )
    .bind(payload.title.clone().unwrap_or(current.title))
    .bind(payload.content.clone().unwrap_or(current.content))
    .bind(announcement_id)
    .bind(discussion_id)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => {
            eprintln!("Failed to update discussion announcement: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update announcement."}))
        }
    }
}
