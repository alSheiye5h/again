use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to link an existing announcement to a discussion.
pub async fn link_discussion_announcement(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (discussion_id, announcement_id) = path.into_inner();

    // Use ON CONFLICT to avoid duplicate entries.
    let result = sqlx::query(
        "INSERT INTO discussion_announcements (discussion_id, announcement_id) VALUES ($1, $2) ON CONFLICT (discussion_id, announcement_id) DO NOTHING",
    )
    .bind(discussion_id)
    .bind(announcement_id)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Created().json(json!({"status": "success", "message": "Announcement linked successfully."})),
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Announcement was already linked."})),
        Err(e) => {
            eprintln!("Failed to link discussion announcement: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to link announcement. Ensure discussion and announcement exist."}))
        }
    }
}
