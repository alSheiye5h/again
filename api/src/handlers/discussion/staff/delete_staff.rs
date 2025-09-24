use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove a staff member from a discussion.
/// This specifically targets users with the 'staff' role.
pub async fn delete_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (discussion_id, user_id) = path.into_inner();

    let result = sqlx::query("DELETE FROM discussion_staff WHERE discussion_id = $1 AND user_id = $2")
        .bind(discussion_id)
        .bind(user_id)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(json!({"status": "success", "message": "Staff member removed successfully."})),
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found in this discussion."})),
        Err(e) => {
            eprintln!("Failed to remove discussion staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to remove staff member."}))
        }
    }
}