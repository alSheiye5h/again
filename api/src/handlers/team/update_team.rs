use actix_web::{web, HttpResponse, Responder};
use crate::models::team_struct::{Team, UpdateTeamPayload};
use serde_json::json;
use sqlx::PgPool;

pub async fn update_team(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<UpdateTeamPayload>,
) -> impl Responder {
    let team_id = path.into_inner();

    // Using COALESCE to only update fields that are provided in the payload.
    // If a field is None, its value in the database remains unchanged.
    let query = r#"
        UPDATE team
        SET description = COALESCE($1, description)
        WHERE id = $2
        RETURNING id, created_by, description
    "#;

    match sqlx::query_as::<_, Team>(query)
        .bind(&payload.description)
        .bind(team_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .json(json!({"status": "error", "message": format!("Team with id {} not found", team_id)})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to update team: {}", e)})),
    }
}