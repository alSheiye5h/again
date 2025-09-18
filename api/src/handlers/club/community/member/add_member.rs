use actix_web::{web, HttpResponse, Responder};
use crate::models::Communitie_struct::AddMemberPayload;
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as a member to a club's community.
pub async fn add_community_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This is club_id
    payload: web::Json<AddMemberPayload>,
) -> impl Responder {
    let club_id = path.into_inner();

    // First, get the club_community_id from the club_id
    let community_id_result: Result<Option<(i32,)>, sqlx::Error> =
        sqlx::query_as("SELECT id FROM club_community WHERE club_id = $1")
            .bind(club_id)
            .fetch_optional(&**db_pool)
            .await;

    let community_id = match community_id_result {
        Ok(Some((id,))) => id,
        Ok(None) => return HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found for this club."})),
        Err(_) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to check community existence."})),
    };

    // Using ON CONFLICT DO NOTHING to handle cases where the user is already a member.
    let result = sqlx::query(
        "INSERT INTO club_community_members (user_id, club_community_id, role) VALUES ($1, $2, 'member') ON CONFLICT (user_id, club_community_id) DO NOTHING",
    )
    .bind(payload.user_id)
    .bind(community_id)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                HttpResponse::Ok().json(json!({"status": "success", "message": "User is already a member of this community."}))
            } else {
                HttpResponse::Created().json(json!({"status": "success", "message": "User added to community successfully."}))
            }
        }
        Err(e) => {
            eprintln!("Failed to add community member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add member."}))
        }
    }
}
