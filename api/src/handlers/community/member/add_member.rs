use actix_web::{web, HttpResponse, Responder};
use api::models::communitieStruct::{AddMemberPayload, CommunityMember, MemberRole};
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as a member to a community.
pub async fn add_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<AddMemberPayload>,
) -> impl Responder {
    let community_id = path.into_inner();
    let user_id = payload.user_id;

    // Default role for a new member is 'Member'.
    let default_role = MemberRole::Member;

    let result = sqlx::query_as::<_, CommunityMember>(
        "INSERT INTO community_members (community_id, user_id, role) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(community_id)
    .bind(user_id)
    .bind(default_role)
    .fetch_one(&**db_pool)
    .await;

    match result {
        Ok(new_member) => HttpResponse::Created().json(new_member),
        Err(e) => {
            eprintln!("Failed to add community member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add member."}))
        }
    }
}
