use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AddMemberPayload {
    pub user_id: i32,
}

/// Handler to add a user as a member to a discussion.
pub async fn add_discussion_member(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<AddMemberPayload>,
) -> impl Responder {
    let discussion_id_val = discussion_id.into_inner();

    // First, check if the discussion exists to provide a clear error message.
    let discussion_exists: Result<Option<i32>, _> = sqlx::query_scalar("SELECT id FROM discussion WHERE id = $1")
        .bind(discussion_id_val)
        .fetch_optional(db_pool.get_ref())
        .await;

    if let Ok(None) | Err(_) = discussion_exists {
        return HttpResponse::NotFound().json(json!({"status": "error", "message": "Discussion not found."}));
    }

    // Use ON CONFLICT to gracefully handle cases where the user is already a member.
    let result = sqlx::query(
        "INSERT INTO discussion_members (user_id, discussion_id) VALUES ($1, $2) ON CONFLICT (user_id, discussion_id) DO NOTHING",
    )
    .bind(payload.user_id)
    .bind(discussion_id_val)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Created().json(json!({"status": "success", "message": "Member added successfully."})),
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "User is already a member."})),
        Err(e) => {
            eprintln!("Failed to add discussion member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add member."}))
        }
    }
}
