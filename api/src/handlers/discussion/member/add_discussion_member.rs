use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::models::Discussion_struct::AddMemberPayload;
use crate::models::Discussion_struct::MemberRole;

/// Handler to add a user as a member to a discussion.
pub async fn add_discussion_member(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<AddMemberPayload>,
) -> impl Responder {
    // Use ON CONFLICT to gracefully handle cases where the user is already a member.
    // This relies on a UNIQUE constraint on (user_id, discussion_id) in the discussion_members table.
    let result = sqlx::query("INSERT INTO discussion_members (user_id, discussion_id, role) VALUES ($1, $2, $3) ON CONFLICT (user_id, discussion_id) DO NOTHING")
    .bind(payload.user_id)
    .bind(discussion_id.into_inner())
    .bind(MemberRole::Member)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Created().json(json!({"status": "success", "message": "Member added successfully."})),
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "User is already a member."})),
        Err(e) => {
            eprintln!("Failed to add discussion member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add member. Ensure discussion and user exist."}))
        }
    }
}
