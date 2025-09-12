use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::ClubMemberPayload;
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as a member to a club.
pub async fn add_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This will be the club_id from the path
    payload: web::Json<ClubMemberPayload>,
) -> impl Responder {
    let club_id = path.into_inner(); // e.g., /club/123/members -> club_id is 123
    let user_id = payload.user_id;

    // Using ON CONFLICT DO NOTHING is a robust way to handle cases where the user is already a member.
    // It prevents the query from failing with a unique constraint violation.
    let result = sqlx::query(
        "INSERT INTO club_members (club_id, user_id) VALUES ($1, $2) ON CONFLICT (club_id, user_id) DO NOTHING",
    )
    .bind(club_id)
    .bind(user_id)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                // This means the user was already a member.
                HttpResponse::Ok().json(json!({"status": "success", "message": "User is already a member of this club."}))
            } else {
                // This means a new row was inserted.
                HttpResponse::Created().json(json!({"status": "success", "message": "User added to club successfully."}))
            }
        }
        Err(e) => {
            // This could fail if the club_id or user_id don't exist (foreign key constraint).
            eprintln!("Failed to add club member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add member to club. The club or user may not exist."}))
        }
    }
}
