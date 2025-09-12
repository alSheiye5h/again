use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove a member from a community.
pub async fn delete_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (community_id, user_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM community_members WHERE community_id = $1 AND user_id = $2 AND role = 'member'",
    )
    .bind(community_id)
    .bind(user_id)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(json!({"status": "success", "message": "Member removed successfully."})),
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Member not found."})),
        Err(e) => {
            eprintln!("Failed to delete community member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete member."}))
        }
    }
}