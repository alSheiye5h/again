use crate::models::
Announcement_struct::{
Announcement_create_payload, 
Announcement_struct};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

/// Create a new announcement.
pub async fn create_announcement(
    pool: web::Data<PgPool>,
    payload: web::Json<
Announcement_create_payload>,
) -> impl Responder {
    match sqlx::query_as!(
        
Announcement_struct,
        r#"
        INSERT INTO announcements (title, content, club_id, community_id, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, content, club_id, community_id, created_by,
                  created_at, updated_at
        "#,
        payload.title,
        payload.content,
        payload.club_id,
        payload.community_id,
        payload.created_by
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(announcement) => HttpResponse::Created().json(announcement),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create announcement: {}", e)),
    }
}