use crate::models::Announcement_struct::AnnouncementStruct;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    page: Option<i64>,
    per_page: Option<i64>,
}

/// List all announcements with pagination.
pub async fn list_announcements(
    pool: web::Data<PgPool>,
    query: web::Query<Pagination>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;

    match sqlx::query_as!(
        
AnnouncementStruct,
        r#"
        SELECT id, title, content, club_id, community_id, created_by, created_at, updated_at
        FROM announcements
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        per_page,
        offset
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch announcements: {}", e)),
    }
}