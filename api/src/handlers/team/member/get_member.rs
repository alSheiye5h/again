use crate::models::team_struct::TeamMemberInfo;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a specific member of a team.
pub async fn get_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (team_id, user_id)
) -> impl Responder {
    let (team_id, user_id) = path.into_inner();

    let query = r#"
        SELECT u.id as user_id, u.username
        FROM team_members tm
        JOIN users u ON tm.user_id = u.id
        WHERE tm.team_id = $1 AND tm.user_id = $2
    "#;

    match sqlx::query_as::<_, TeamMemberInfo>(query)
        .bind(team_id)
        .bind(user_id)
        .fetch_optional(&**db_pool)
        .await
    {
        Ok(Some(member)) => HttpResponse::Ok().json(member),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Member not found in this team."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to fetch team member: {}", e)})),
    }
}