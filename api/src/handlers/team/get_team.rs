use crate::models::team_struct::Team;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Get a single team by its ID.
pub async fn get_team(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    match sqlx::query_as!(
        Team,
        r#"
        SELECT id, created_by
        FROM team WHERE id = $1
        "#,
        *id
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(team)) => HttpResponse::Ok().json(team),
        Ok(None) => HttpResponse::NotFound()
            .json(json!({"status": "error", "message": format!("Team with id {} not found", id)})),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to fetch team: {}", e))
        }
    }
}