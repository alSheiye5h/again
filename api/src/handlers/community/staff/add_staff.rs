use actix_web::{web, HttpResponse, Responder};
use api::models::communitieStruct::{AddMemberPayload, CommunityMember, MemberRole};
use serde_json::json;
use sqlx::PgPool;

/// Handler to add a user as staff or promote an existing member to staff.
/// This uses an "upsert" operation.
pub async fn add_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<AddMemberPayload>,
) -> impl Responder {
    let community_id = path.into_inner();
    let user_id = payload.user_id;

    let result = sqlx::query_as::<_, CommunityMember>(
        r#"
        INSERT INTO community_members (community_id, user_id, role)
        VALUES ($1, $2, $3)
        ON CONFLICT (community_id, user_id)
        DO UPDATE SET role = EXCLUDED.role
        RETURNING *
        "#,
    )
    .bind(community_id)
    .bind(user_id)
    .bind(MemberRole::Staff) // Add or update to 'staff'
    .fetch_one(&**db_pool)
    .await;

    match result {
        Ok(staff_member) => HttpResponse::Ok().json(staff_member),
        Err(e) => {
            eprintln!("Failed to add or promote community staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to add or promote staff."}))
        }
    }
}