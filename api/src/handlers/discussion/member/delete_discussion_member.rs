use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove a member from a discussion.
pub async fn remove_discussion_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (discussion_id, user_id) = path.into_inner();

    let result = sqlx::query("DELETE FROM discussion_members WHERE discussion_id = $1 AND user_id = $2")
        .bind(discussion_id)
        .bind(user_id)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Member removed successfully."}))
        }
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Member not found in this discussion."})),
        Err(e) => {
            eprintln!("Failed to remove discussion member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to remove member."}))
        }
    }
}
