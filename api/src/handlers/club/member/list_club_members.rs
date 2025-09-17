use actix_web::{web, HttpResponse, Responder};
use crate::models::Club_struct::ClubMemberInfo;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all members of a club.
/// Joins with the `users` table to provide more detailed member info.
pub async fn list_members(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let club_id = path.into_inner();

    let query = "
        SELECT u.id as user_id, u.username, u.email
        FROM club_members cm
        JOIN users u ON cm.user_id = u.id
        WHERE cm.club_id = $1
    ";

    match sqlx::query_as::<_, ClubMemberInfo>(query)
        .bind(club_id)
        .fetch_all(&**db_pool)
        .await
    {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => {
            eprintln!("Failed to fetch club members: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to fetch club members."}))
        }
    }
}
