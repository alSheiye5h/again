use actix_web::{web, HttpResponse, Responder};
use crate::models::Club_struct::ClubMemberInfo;
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a specific member from a club.
pub async fn get_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    let query = "
        SELECT u.id as user_id, u.username, u.email
        FROM club_members cm
        JOIN users u ON cm.user_id = u.id
        WHERE cm.club_id = $1 AND cm.user_id = $2
    ";

    match sqlx::query_as::<_, ClubMemberInfo>(query)
        .bind(club_id)
        .bind(user_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(member) => HttpResponse::Ok().json(member),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .json(json!({"status": "error", "message": "Member not found in this club."})),
        Err(e) => {
            eprintln!("Failed to fetch club member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch club member."}))
        }
    }
}
