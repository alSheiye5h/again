use crate::models::Announcement_struct::AnnouncementStruct;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a specific announcement linked to a discussion.
pub async fn get_linked_discussion_announcement(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (discussion_id, announcement_id) = path.into_inner();

    let result = sqlx::query_as::<_, AnnouncementStruct>(
        r#"
        SELECT a.*
        FROM announcements a
        JOIN discussion_announcements da ON a.id = da.announcement_id
        WHERE da.discussion_id = $1 AND da.announcement_id = $2
        "#,
    )
    .bind(discussion_id)
    .bind(announcement_id)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(announcement) => HttpResponse::Ok().json(announcement),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Linked announcement not found."})),
        Err(e) => {
            eprintln!("Failed to get linked discussion announcement: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to retrieve announcement."}))
        }
    }
}
