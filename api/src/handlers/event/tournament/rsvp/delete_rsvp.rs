use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to delete a user's RSVP for a tournament event.
pub async fn delete_rsvp(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (event_id, user_id)
) -> impl Responder {
    let (event_id, user_id) = path.into_inner();

    let result = sqlx::query("DELETE FROM tournament_event_rsvp WHERE event_id = $1 AND user_id = $2")
        .bind(event_id)
        .bind(user_id)
        .execute(&**db_pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(json!({"status": "success", "message": "RSVP deleted successfully."})),
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "RSVP not found."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to delete RSVP: {}", e)})),
    }
}

