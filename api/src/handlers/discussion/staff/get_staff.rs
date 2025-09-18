use crate::models::discussion_struct::{DiscussionMemberInfo, MemberRole};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a specific staff member of a discussion.
pub async fn get_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (discussion_id, user_id) = path.into_inner();

    let result = sqlx::query_as::<_, DiscussionMemberInfo>(
        r#"
        SELECT u.id as user_id, u.username, dm.role
        FROM users u
        JOIN discussion_members dm ON u.id = dm.user_id
        WHERE dm.discussion_id = $1 AND dm.user_id = $2 AND dm.role = $3
        "#,
    )
    .bind(discussion_id)
    .bind(user_id)
    .bind(MemberRole::Staff)
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(staff)) => HttpResponse::Ok().json(staff),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found in this discussion."})),
        Err(e) => {
            eprintln!("Failed to get discussion staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to retrieve staff member."}))
        }
    }
}