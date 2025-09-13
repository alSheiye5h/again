use actix_web::{web, HttpResponse, Responder};
use api::models::communitieStruct::{CommunityMember, MemberRole};
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all staff members (Staff, Admin) of a specific community.
pub async fn list_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let community_id = path.into_inner();

    let result = sqlx::query_as::<_, CommunityMember>(
        "SELECT * FROM community_members WHERE community_id = $1 AND role IN ($2, $3)",
    )
    .bind(community_id)
    .bind(MemberRole::Staff)
    .bind(MemberRole::Admin)
    .fetch_all(&**db_pool)
    .await;

    match result {
        Ok(staff) => HttpResponse::Ok().json(staff),
        Err(e) => {
            eprintln!("Failed to fetch community staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch staff."}))
        }
    }
}