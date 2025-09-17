use actix_web::{web, HttpResponse, Responder};
use crate::models::Communitie_struct::CommunityStaffInfo;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all staff members of a specific community.
pub async fn list_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let community_id = path.into_inner();

    let query = r#"
        SELECT cs.user_id, u.username, cs.promoted_by
        FROM community_staff cs
        JOIN users u ON cs.user_id = u.id
        WHERE cs.community_id = $1
    "#;

    match sqlx::query_as::<_, CommunityStaffInfo>(query)
        .bind(community_id)
        .fetch_all(&**db_pool)
        .await
    {
        Ok(staff) => HttpResponse::Ok().json(staff),
        Err(e) => {
            eprintln!("Failed to fetch community staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch staff."}))
        }
    }
}