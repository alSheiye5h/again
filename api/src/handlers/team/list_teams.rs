use crate::models::Team_struct::{Pagination, Team};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

/// List all teams with pagination.
pub async fn list_teams(
    pool: web::Data<PgPool>,
    query: web::Query<Pagination>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;

    match sqlx::query_as!(
        Team,
        r#"
        SELECT id, created_by
        FROM team
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#,
        per_page,
        offset
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(teams) => HttpResponse::Ok().json(teams),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch teams: {}", e)),
    }
}