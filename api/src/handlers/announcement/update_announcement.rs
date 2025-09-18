use crate::models::announcement_struct::{AnnouncementStruct, 
AnnouncementUpdatePayload};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

/// Update an existing announcement.
pub async fn update_announcement(
    pool: web::Data<PgPool>,
    id: web::Path<u32>,
    payload: web::Json<
AnnouncementUpdatePayload>,
) -> impl Responder {
    // Fetch current announcement to get existing values
    let current = match sqlx::query_as!(
        
AnnouncementStruct,
        r#"SELECT id, title, content, club_id, community_id, created_by, created_at, updated_at FROM announcements WHERE id = $1"#,
        *id as i32
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(ann) => ann,
        Err(_) => return HttpResponse::NotFound().body("Announcement not found"),
    };

    match sqlx::query_as!(
        
AnnouncementStruct,
        r#"
        UPDATE announcements
        SET title = $1, content = $2, club_id = $3, community_id = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING id, title, content, club_id, community_id, created_by,
                  created_at, updated_at
        "#,
        payload.title.clone().unwrap_or(current.title),
        payload.content.clone().unwrap_or(current.content),
        payload.club_id.or(current.club_id),
        payload.community_id.or(current.community_id),
        *id as i32
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to update announcement: {}", e))
        }
    }
}