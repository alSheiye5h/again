use actix_web::{web, HttpResponse, Responder};

use api::models::ama_poolStruct::Ama;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all AMAs for a club.
pub async fn list_club_amas(db_pool: web::Data<PgPool>, club_id: web::Path<i32>) -> impl Responder {
    let club_id_val = club_id.into_inner();

    let query = r#"
        SELECT a.id, a.created_by
        FROM ama a
        INNER JOIN club_ama ca ON a.id = ca.ama_id
        WHERE ca.club_id = $1
        ORDER BY a.id DESC
    "#;

    match sqlx::query_as::<_, Ama>(query)
        .bind(club_id_val)
        .fetch_all(&**db_pool)
        .await
    {
        Ok(amas) => HttpResponse::Ok().json(amas),
        Err(e) => {
            eprintln!("Failed to list club AMAs: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list club AMAs."}))
        }
    }
}
