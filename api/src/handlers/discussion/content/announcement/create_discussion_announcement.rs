use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;
use sqlx::{FromRow, PgPool};

use crate::models::Announcement_struct::{AnnouncementStruct, CreateAnnouncementPayload};

/// Handler to create a new announcement for a discussion.
pub async fn create_discussion_announcement(
    pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<CreateAnnouncementPayload>,
) -> impl Responder {
    let discussion_id_val = discussion_id.into_inner();

    // Define a local struct for the response to ensure it matches the RETURNING clause.
    // This avoids potential mismatches with a global AnnouncementStruct.
    #[derive(FromRow, Serialize)]
    struct NewAnnouncement {
        id: i32,
        title: String,
        content: String,
        created_by: i32,
        discussion_id: Option<i32>,
    }

    match sqlx::query_as::<_, NewAnnouncement>(
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
        Err(e) => {
            eprintln!("Failed to create announcement: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to create announcement."
            }))
        }
    }
}