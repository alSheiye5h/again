use actix_web::{web, HttpResponse, Responder};
use api::models::communitieStruct::{AddMemberPayload, CommunityMember, MemberRole};
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as staff to a club's community.
pub async fn add_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This is club_id
    payload: web::Json<AddClubCommunityStaffPayload>,
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

    // Using ON CONFLICT DO NOTHING to prevent errors if the user is already staff.
    let result = sqlx::query(
        "INSERT INTO club_community_staff (user_id, club_community_id, promoted_by) VALUES ($1, $2, $3) ON CONFLICT (user_id, club_community_id) DO NOTHING",
    )
    .bind(payload.user_id)
    .bind(community_id)
    .bind(payload.promoted_by)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                HttpResponse::Ok().json(json!({"status": "success", "message": "User is already a staff member in this community."}))
            } else {
                HttpResponse::Created().json(json!({"status": "success", "message": "User promoted to staff successfully."}))
            }
        }
        Err(e) => {
            eprintln!("Failed to add club community staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add staff."}))
        }
    }
}