use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::ClubCommunityStaffInfo;
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a single staff member from a community.
pub async fn get_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (club_id, user_id)
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    let query = r#"
        SELECT ccs.user_id, u.username, ccs.promoted_by
        FROM club_community_staff ccs
        JOIN users u ON ccs.user_id = u.id
        JOIN club_community cc ON ccs.club_community_id = cc.id
        WHERE cc.club_id = $1 AND ccs.user_id = $2
    "#;

    match sqlx::query_as::<_, ClubCommunityStaffInfo>(query)
        .bind(club_id)
        .bind(user_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(staff) => HttpResponse::Ok().json(staff),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .json(json!({"status": "error", "message": "Staff member not found in this club's community."})),
        Err(e) => {
            eprintln!("Failed to fetch community staff member: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to fetch staff member."}))
        }
    }
}