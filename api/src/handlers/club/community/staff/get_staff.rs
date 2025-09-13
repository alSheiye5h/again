use actix_web::{web, HttpResponse, Responder};
use api::models::communitieStruct::{CommunityMember, MemberRole};
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a single staff member from a community.
pub async fn get_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (community_id, user_id) = path.into_inner();

    let result = sqlx::query_as::<_, CommunityMember>(
        "SELECT * FROM community_members WHERE community_id = $1 AND user_id = $2 AND role IN ($3, $4)",
    )
    .bind(community_id)
    .bind(user_id)
    .bind(MemberRole::Staff)
    .bind(MemberRole::Admin)
    .fetch_optional(&**db_pool)
    .await;

    match result {
        Ok(Some(staff_member)) => HttpResponse::Ok().json(staff_member),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found."})),
        Err(e) => {
            eprintln!("Failed to fetch community staff member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch staff member."}))
        }
    }
}