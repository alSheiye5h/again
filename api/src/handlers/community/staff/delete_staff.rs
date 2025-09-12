use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove a staff member from a community.
pub async fn delete_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (community_id, user_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM community_members WHERE community_id = $1 AND user_id = $2 AND role IN ('staff', 'admin')",
    )
    .bind(community_id)
    .bind(user_id)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found."})),
        Err(e) => {
            eprintln!("Failed to delete community staff member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete staff member."}))
        }
    }
}