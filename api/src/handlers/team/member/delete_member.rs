use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove a member from a team.
pub async fn delete_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (team_id, user_id)
) -> impl Responder {
    let (team_id, user_id) = path.into_inner();

    match sqlx::query("DELETE FROM team_members WHERE team_id = $1 AND user_id = $2")
        .bind(team_id)
        .bind(user_id)
        .execute(&**db_pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Member removed from team successfully."}))
        }
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Member not found in this team."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to remove member from team: {}", e)})),
    }
}