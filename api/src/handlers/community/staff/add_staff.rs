use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::models::communitie_struct::AddCommunityStaffPayload;

/// Handler to add a user as staff to a community.
pub async fn add_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<AddCommunityStaffPayload>,
) -> impl Responder {
    let community_id = path.into_inner();

        // Step 1: Check if the user is already a staff member.
        let existing_staff: Result<Option<(i32,)>, sqlx::Error> =
        sqlx::query_as("SELECT user_id FROM community_staff WHERE user_id = $1 AND community_id = $2")
            .bind(payload.user_id)
            .bind(community_id)
            .fetch_optional(&**db_pool)
            .await;

    match existing_staff {
        Ok(Some(_)) => {
            return HttpResponse::Ok().json(json!({"status": "success", "message": "User is already a staff member."}));
        }
        Ok(None) => { /* Continue to insert */ }
        Err(e) => {
            eprintln!("Failed to check for existing community staff: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to check for existing staff."}));
        }
    }

    // Step 2: Insert the new staff member.
    let result = sqlx::query(
        "INSERT INTO community_staff (user_id, community_id, promoted_by) VALUES ($1, $2, $3)",
    )
    .bind(payload.user_id)
    .bind(community_id)
    .bind(payload.promoted_by)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({"status": "success", "message": "User promoted to staff successfully."})),
        Err(e) => {
            eprintln!("Failed to add community staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add staff."}))
        }
    }
}