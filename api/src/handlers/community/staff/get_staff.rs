use actix_web::{web, HttpResponse, Responder};
use crate::models::communitie_struct::CommunityStaffInfo;
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a single staff member from a community.
pub async fn get_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (community_id, user_id)
) -> impl Responder {
    let (community_id, user_id) = path.into_inner();

    let query = r#"
        SELECT cs.user_id, u.username, cs.promoted_by
        FROM community_staff cs
        JOIN users u ON cs.user_id = u.id
        WHERE cs.community_id = $1 AND cs.user_id = $2
    "#;

    match sqlx::query_as::<_, CommunityStaffInfo>(query)
        .bind(community_id)
        .bind(user_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(staff) => HttpResponse::Ok().json(staff),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .json(json!({"status": "error", "message": "Staff member not found in this community."})),
        Err(e) => {
            eprintln!("Failed to fetch community staff member: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to fetch staff member."}))
        }
    }
}