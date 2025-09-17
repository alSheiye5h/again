use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to unlink an announcement from a discussion.
pub async fn unlink_discussion_announcement(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (discussion_id, announcement_id) = path.into_inner();

    let result = sqlx::query("DELETE FROM discussion_announcements WHERE discussion_id = $1 AND announcement_id = $2")
        .bind(discussion_id)
        .bind(announcement_id)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Announcement unlinked successfully."}))
        }
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Announcement link not found."})),
        Err(e) => {
            eprintln!("Failed to unlink discussion announcement: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to unlink announcement."}))
        }
    }
}
