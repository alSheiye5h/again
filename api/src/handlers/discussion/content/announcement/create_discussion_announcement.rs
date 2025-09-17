use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::models::Announcement_struct::{DiscussionAnnouncement, CreateAnnouncementPayload};

/// Handler to create a new announcement for a discussion.
pub async fn create_discussion_announcement(
    pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<CreateAnnouncementPayload>,
) -> impl Responder {
    let discussion_id_val = discussion_id.into_inner();

    match sqlx::query_as::<_, DiscussionAnnouncement>(
        r#"
        INSERT INTO announcements (title, content, created_by, discussion_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, content, created_by, discussion_id
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(payload.created_by)
    .bind(discussion_id_val)
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(announcement) => HttpResponse::Created().json(announcement),
        Err(sqlx::Error::Database(db_err)) if db_err.is_foreign_key_violation() => {
            HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": format!("Discussion with ID {} not found.", discussion_id_val)
            }))
        }
        Err(e) => {
            eprintln!("Failed to create announcement: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to create announcement."
            }))
        }
    }
}