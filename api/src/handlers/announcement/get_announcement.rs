use crate::models::
Announcement_struct::
Announcement_struct;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Get a single announcement by its ID.
pub async fn get_announcement(
    pool: web::Data<PgPool>,
    id: web::Path<u32>,
) -> impl Responder {
    match sqlx::query_as!(
        
Announcement_struct,
        r#"
        SELECT id, title, content, club_id, community_id, created_by,
               created_at, updated_at
        FROM announcements WHERE id = $1
        "#,
        *id as i32
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(announcement)) => HttpResponse::Ok().json(announcement),
        Ok(None) => HttpResponse::NotFound().json(
            json!({"status": "error", "message": format!("Announcement with id {} not found", id)}),
        ),
        Err(e) => {
            eprintln!("Failed to fetch announcement: {}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch announcement."}))
        }
    }
}