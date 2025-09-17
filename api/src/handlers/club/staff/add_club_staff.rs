use actix_web::{web, HttpResponse, Responder};
use crate::models::Club_struct::AddClubStaffPayload;
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as a staff member to a club.
pub async fn add_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<AddClubStaffPayload>,
) -> impl Responder {
    let club_id = path.into_inner();

    // Using ON CONFLICT DO NOTHING to prevent errors if the user is already staff.
    let result = sqlx::query(
        "INSERT INTO club_staff (user_id, club_id, promoted_by) VALUES ($1, $2, $3) ON CONFLICT (user_id, club_id) DO NOTHING",
    )
    .bind(payload.user_id)
    .bind(club_id)
    .bind(payload.promoted_by)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                HttpResponse::Ok().json(json!({"status": "success", "message": "User is already a staff member in this club."}))
            } else {
                HttpResponse::Created().json(json!({"status": "success", "message": "User promoted to staff successfully."}))
            }
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code() == Some("23503".into()) => {
            // This is a foreign key violation error.
            eprintln!("Failed to add club staff due to foreign key violation: {:?}", db_err);
            HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Failed to add staff. The specified club, user, or promoter does not exist."
            }))
        }
        Err(other_err) => {
            eprintln!("Failed to add club staff: {:?}", other_err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "An unexpected error occurred while trying to add staff."}))
        }
    }
}
