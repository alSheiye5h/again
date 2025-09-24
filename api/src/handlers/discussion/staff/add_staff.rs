use crate::models::discussion_struct::{AddStaffPayload};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as a staff member to a discussion.
/// This uses an "upsert" logic: if the user is already a staff member, their record is updated.
/// If they are not, they are added as staff.
pub async fn add_staff(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<AddStaffPayload>,
) -> impl Responder {
    let result = sqlx::query(
        r#"
        INSERT INTO discussion_staff (user_id, discussion_id, promoted_by, role)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, discussion_id) DO UPDATE 
        SET role = EXCLUDED.role, promoted_by = EXCLUDED.promoted_by
        "#,
    )
    .bind(payload.user_id)
    .bind(discussion_id.into_inner())
    .bind(payload.promoted_by)
    .bind(payload.role)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Staff member added or updated successfully."})),
        Err(e) => {
            eprintln!("Failed to add discussion staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add staff member. Ensure discussion and user exist."}))
        }
    }
}