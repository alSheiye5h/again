use actix_web::{web, HttpResponse, Responder};
use crate::models::club_struct::Club;
use sqlx::PgPool;

/// Handler to get a club by its ID.
pub async fn get_club_by_id(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let club_id = path.into_inner();

    match sqlx::query_as::<_, Club>("SELECT * FROM club WHERE id = $1")
        .bind(club_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(club) => HttpResponse::Ok().json(club),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(format!("Club with id {} not found", club_id))
        }
        Err(e) => {
            eprintln!("Failed to fetch club: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to fetch club")
        }
    }
}
