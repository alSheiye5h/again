use actix_web::{web, HttpResponse, Responder};
use api::models::Club_struct::ClubCommunityMember;
use api::models::Communitie_struct::UpdateMemberPayload;
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a community member's role.
pub async fn update_community_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (club_id, user_id)
    payload: web::Json<UpdateMemberPayload>,
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    let query = r#"
        UPDATE club_community_members ccm
        SET role = $1
        FROM club_community cc
        WHERE ccm.club_community_id = cc.id
          AND cc.club_id = $2
          AND ccm.user_id = $3
        RETURNING ccm.id, ccm.user_id, ccm.club_community_id, ccm.role, ccm.joined_at
    "#;

    let result = sqlx::query_as::<_, ClubCommunityMember>(query)
        .bind(payload.role) // Directly bind the enum; sqlx handles the type mapping
        .bind(club_id)
        .bind(user_id)
        .fetch_optional(&**db_pool)
        .await;

    match result {
        Ok(Some(updated_member)) => HttpResponse::Ok().json(updated_member),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Member not found in this club's community."})),
        Err(e) => {
            eprintln!("Failed to update community member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update member."}))
        }
    }
}