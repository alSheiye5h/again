use actix_web::{web, HttpResponse, Responder};
use crate::models::Club_struct::ClubCommunityStaffInfo;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all staff members of a specific club's community.
pub async fn list_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This is club_id
) -> impl Responder {
    let club_id = path.into_inner();

    // First, get the club_community_id from the club_id
    let community_id_result: Result<Option<(i32,)>, sqlx::Error> =
        sqlx::query_as("SELECT id FROM club_community WHERE club_id = $1")
            .bind(club_id)
            .fetch_optional(&**db_pool)
            .await;

    let community_id = match community_id_result {
        Ok(Some((id,))) => id,
        Ok(None) => return HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found for this club."})),
        Err(_) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to check community existence."})),
    };

    let query = r#"
        SELECT ccs.user_id, u.username, ccs.promoted_by
        FROM club_community_staff ccs
        JOIN users u ON ccs.user_id = u.id
        WHERE ccs.club_community_id = $1
    "#;

    match sqlx::query_as::<_, ClubCommunityStaffInfo>(query)
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