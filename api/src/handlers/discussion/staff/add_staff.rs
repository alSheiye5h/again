use crate::models::discussion_struct::{AddStaffPayload};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as a staff member to a discussion.
/// Returns an error if the user is already a staff member.
pub async fn add_staff(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<AddStaffPayload>,
) -> impl Responder {
    let result = sqlx::query(
        r#"
        INSERT INTO discussion_staff (user_id, discussion_id, promoted_by, role)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(payload.user_id)
    .bind(discussion_id.into_inner())
    .bind(payload.promoted_by)
    .bind(payload.role)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({"status": "success", "message": "Staff member added successfully."})),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                return HttpResponse::Conflict().json(json!({
                    "status": "error",
                    "message": "User is already a staff member in this discussion."
                }));
            } else if db_err.is_foreign_key_violation() {
                return HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Failed to add staff. The specified discussion, user, or promoter does not exist."
                }));
            }
            eprintln!("Database error while adding discussion staff: {:?}", db_err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "A database error occurred."}))
        }
        Err(e) => {
            eprintln!("Failed to add discussion staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "An unexpected error occurred."}))
        }
    }
}