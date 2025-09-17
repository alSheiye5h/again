use crate::models::Discussion_struct::{AddMemberPayload, MemberRole};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as a staff member to a discussion.
/// This uses an "upsert" logic: if the user is already a member, their role is updated to staff.
/// If they are not a member, they are added as staff.
pub async fn add_staff(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<AddMemberPayload>,
) -> impl Responder {
    let result = sqlx::query(
        r#"
        INSERT INTO discussion_members (user_id, discussion_id, role)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, discussion_id) DO UPDATE SET role = $3
        "#,
    )
    .bind(payload.user_id)
    .bind(discussion_id.into_inner())
    .bind(MemberRole::Staff)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Staff member added or role updated successfully."})),
        Err(e) => {
            eprintln!("Failed to add discussion staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add staff member. Ensure discussion and user exist."}))
        }
    }
}