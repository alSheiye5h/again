use crate::models::team_struct::{CreateTeamPayload, Team};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Create a new team.
pub async fn create_team(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateTeamPayload>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Team>(
        r#"
        INSERT INTO team (created_by, description)
        VALUES ($1, $2)
        RETURNING id, created_by, description
        "#,
    )
    .bind(payload.created_by)
    .bind(&payload.description)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(team) => HttpResponse::Created().json(team),
        Err(sqlx::Error::Database(db_err)) if db_err.code() == Some("23503".into()) => {
            HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Failed to create team. The specified creator (user) does not exist."
            }))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": format!("Failed to create team: {}", e)})),
    }
}