use crate::models::Announcement_struct::AnnouncementStruct;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all announcements linked to a specific discussion.
pub async fn list_linked_discussion_announcements(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as::<_, AnnouncementStruct>(
        r#"
        SELECT a.*
        FROM announcements a
        JOIN discussion_announcements da ON a.id = da.announcement_id
        WHERE da.discussion_id = $1
        ORDER BY a.created_at DESC
        "#,
    )
    .bind(discussion_id.into_inner())
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(e) => {
            eprintln!("Failed to list linked discussion announcements: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list announcements."}))
        }
    }
}
