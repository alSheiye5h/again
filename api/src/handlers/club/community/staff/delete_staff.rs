use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove a staff member from a community.
pub async fn delete_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (club_id, user_id)
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    let result = sqlx::query(r#"
        DELETE FROM club_community_staff ccs
        USING club_community cc
        WHERE ccs.club_community_id = cc.id
          AND cc.club_id = $1
          AND ccs.user_id = $2
    "#)
        .bind(club_id)
        .bind(user_id)
        .execute(&**db_pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Staff member removed successfully."}))
        }
        Ok(_) => HttpResponse::NotFound()
            .json(json!({"status": "error", "message": "Staff member not found in this community."})),
        Err(e) => {
            eprintln!("Failed to delete community staff member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete staff member."}))
        }
    }
}