use actix_web::{web, HttpResponse, Responder};
use crate::models::club_struct::Club;
use sqlx::PgPool;

/// Handler to get a list of all clubs.
pub async fn list_clubs(db_pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Club>("SELECT * FROM club")
        .fetch_all(&**db_pool)
        .await
    {
        Ok(clubs) => HttpResponse::Ok().json(clubs),
        Err(e) => {
            eprintln!("Failed to fetch clubs: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to fetch clubs")
        }
    }
}
