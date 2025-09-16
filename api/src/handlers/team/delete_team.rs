use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Delete a team by its ID.
pub async fn delete_team(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    // The ON DELETE CASCADE on team_members table will handle member removal
    let result = sqlx::query!("DELETE FROM team WHERE id = $1", *id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Team deleted successfully."}))
        }
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": format!("Team with id {} not found.", *id)})),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete team: {}", e))
        }
    }
}