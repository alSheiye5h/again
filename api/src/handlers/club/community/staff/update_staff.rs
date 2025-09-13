use actix_web::{web, HttpResponse, Responder};
use api::models::communitieStruct::{CommunityMember, MemberRole, UpdateMemberPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a club community staff member's `promoted_by` field.
pub async fn update_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (club_id, user_id)
    payload: web::Json<UpdateClubCommunityStaffPayload>,
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    let query = r#"
        UPDATE club_community_staff ccs
        SET promoted_by = $1
        FROM club_community cc
        WHERE ccs.club_community_id = cc.id
          AND cc.club_id = $2
          AND ccs.user_id = $3
        RETURNING (SELECT username FROM users WHERE id = $3), ccs.user_id, ccs.promoted_by
    "#;

    match sqlx::query_as::<_, ClubCommunityStaffInfo>(query)
        .bind(payload.promoted_by)
        .bind(club_id)
        .bind(user_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(updated_member) => HttpResponse::Ok().json(updated_member),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found."}))
        }
        Err(e) => {
            eprintln!("Failed to update community staff member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update staff member."}))
        }
    }
}