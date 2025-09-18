use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::event_struct::Event;

/// Handler to list all events hosted by a specific club.
pub async fn list_club_events(
    db_pool: web::Data<PgPool>,
    club_id: web::Path<i32>,
) -> impl Responder {
    let club_id_val = club_id.into_inner();

    let result = sqlx::query_as::<_, Event>(
        r#"
        SELECT e.id, e.club_host, e.community_host, e.organizer, e.has_discussion, charity_event_discussion.discussion_id
        FROM charity_event e
        LEFT JOIN charity_event_discussion ON e.id = charity_event_discussion.event_id
        WHERE e.club_host = $1
        UNION ALL
        SELECT e.id, e.club_host, e.community_host, e.organizer, e.has_discussion, regular_event_discussion.discussion_id
        FROM regular_event e
        LEFT JOIN regular_event_discussion ON e.id = regular_event_discussion.event_id
        WHERE e.club_host = $1
        UNION ALL
        SELECT e.id, e.club_host, e.community_host, e.organizer, e.has_discussion, tournament_event_discussion.discussion_id
        FROM tournament_event e
        LEFT JOIN tournament_event_discussion ON e.id = tournament_event_discussion.event_id
        WHERE e.club_host = $1
        "#,
    )
    .bind(club_id_val)
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => {
            eprintln!("Failed to fetch club events: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to fetch club events: {}", e)
            }))
        }
    }
}
