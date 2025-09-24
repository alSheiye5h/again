use crate::models::discussion_struct::UpdateMemberRolePayload;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a staff member's role within a discussion.
pub async fn update_staff_role(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    payload: web::Json<UpdateMemberRolePayload>,
) -> impl Responder {
    let (discussion_id, user_id) = path.into_inner();

    let result = sqlx::query(
        "UPDATE discussion_staff SET role = $1 WHERE discussion_id = $2 AND user_id = $3",
    )
    .bind(payload.role)
    .bind(discussion_id)
    .bind(user_id)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(json!({"status": "success", "message": "Staff role updated successfully."})),
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found in this discussion."})),
        Err(e) => {
            eprintln!("Failed to update discussion staff role: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update staff role."}))
        }
    }
}