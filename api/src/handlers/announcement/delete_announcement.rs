use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Delete an announcement by its ID.
pub async fn delete_announcement(
    pool: web::Data<PgPool>,
    id: web::Path<u32>,
) -> impl Responder {
    let result = sqlx::query!("DELETE FROM announcements WHERE id = $1", *id as i32)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Announcement deleted successfully."}))
        }
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": format!("Announcement with id {} not found.", *id)})),
        Err(e) => {
            eprintln!("Failed to delete announcement: {}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete announcement."}))
        }
    }
}