use crate::models::Team_struct::{CreateTeamPayload, Team};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

/// Create a new team.
pub async fn create_team(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateTeamPayload>,
) -> impl Responder {
    match sqlx::query_as!(
        Team,
        r#"
        INSERT INTO team (created_by)
        VALUES ($1)
        RETURNING id, created_by
        "#,
        payload.created_by
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(team) => HttpResponse::Created().json(team),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create team: {}", e)),
    }
}