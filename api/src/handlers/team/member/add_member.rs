use crate::models::Team_struct::AddTeamMemberPayload;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as a member to a team.
pub async fn add_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This will be the team_id
    payload: web::Json<AddTeamMemberPayload>,
) -> impl Responder {
    let team_id = path.into_inner();
    let user_id = payload.user_id;

    let result = sqlx::query(
        "INSERT INTO team_members (team_id, user_id) VALUES ($1, $2) ON CONFLICT (team_id, user_id) DO NOTHING",
    )
    .bind(team_id)
    .bind(user_id)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Created().json(json!({"status": "success", "message": "User added to team successfully."}))
        }
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "User is already a member of this team."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to add member to team: {}", e)})),
    }
}