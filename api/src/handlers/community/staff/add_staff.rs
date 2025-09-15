use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use api::models::Communitie_struct::AddCommunityStaffPayload;

/// Handler to add a user as staff to a community.
pub async fn add_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<AddCommunityStaffPayload>,
) -> impl Responder {
    let community_id = path.into_inner();

    // Using ON CONFLICT DO NOTHING to prevent errors if the user is already staff.
    let result = sqlx::query(
        "INSERT INTO community_staff (user_id, community_id, promoted_by) VALUES ($1, $2, $3) ON CONFLICT (user_id, community_id) DO NOTHING",
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
            eprintln!("Failed to add community staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add staff."}))
        }
    }
}