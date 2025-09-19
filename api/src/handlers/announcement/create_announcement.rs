use crate::models::announcement_struct::{
AnnouncementCreatePayload, 
AnnouncementStruct};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Create a new announcement.
pub async fn create_announcement(
    pool: web::Data<PgPool>,
    payload: web::Json<AnnouncementCreatePayload>,
) -> impl Responder {
    // Application-level check for immediate feedback
    if payload.club_id.is_some() == payload.community_id.is_some() {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "An announcement must be linked to exactly one of either a club_id or a community_id."
        }));
    }

    let result = sqlx::query_as::<_, AnnouncementStruct>(
        r#"
        INSERT INTO announcements (title, content, club_id, community_id, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, content, club_id, community_id, created_by,
                  created_at, updated_at
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(payload.club_id)
    .bind(payload.community_id)
    .bind(payload.created_by)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(announcement) => HttpResponse::Created().json(announcement),
        Err(sqlx::Error::Database(db_err)) if db_err.code() == Some("23514".into()) => {
            HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "An announcement must be linked to exactly one of either a club_id or a community_id."
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to create announcement: {}", e)})),
    }
}