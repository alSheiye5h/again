use crate::models::Discussion_struct::{DiscussionMemberInfo, MemberRole};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all staff members of a specific discussion.
pub async fn list_staff(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as::<_, DiscussionMemberInfo>(
        r#"
        SELECT u.id as user_id, u.username, dm.role
        FROM users u
        JOIN discussion_members dm ON u.id = dm.user_id
        WHERE dm.discussion_id = $1 AND dm.role = $2
        "#,
    )
    .bind(discussion_id.into_inner())
    .bind(MemberRole::Staff)
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(staff) => HttpResponse::Ok().json(staff),
        Err(e) => {
            eprintln!("Failed to list discussion staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list staff."}))
        }
    }
}