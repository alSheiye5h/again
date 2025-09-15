use actix_web::{web, HttpResponse, Responder};
use api::models::Communitie_struct::{CommunityMember, MemberRole};
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all members of a specific community.
pub async fn list_community_members(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let community_id = path.into_inner();

    let result = sqlx::query_as::<_, CommunityMember>(
        "SELECT * FROM community_members WHERE community_id = $1 AND role = $2",
    )
    .bind(community_id)
    .bind(MemberRole::Member)
    .fetch_all(&**db_pool)
    .await;

    match result {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => {
            eprintln!("Failed to fetch community members: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch members."}))
        }
    }
}
