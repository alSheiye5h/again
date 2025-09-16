use crate::models::Team_struct::TeamMemberInfo;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all members of a specific team.
pub async fn list_members(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // team_id
) -> impl Responder {
    let team_id = path.into_inner();

    let query = r#"
        SELECT tm.user_id, u.username
        FROM team_members tm
        JOIN users u ON tm.user_id = u.id
        WHERE tm.team_id = $1
    "#;

    match sqlx::query_as::<_, TeamMemberInfo>(query).bind(team_id).fetch_all(&**db_pool).await {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to fetch team members: {}", e)})),
    }
}