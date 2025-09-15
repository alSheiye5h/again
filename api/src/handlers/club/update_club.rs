use actix_web::{web, HttpResponse, Responder};
use api::models::Club_struct::{Club, UpdateClubPayload};
use sqlx::PgPool;

/// Handler to update a club's details.
pub async fn update_club(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<UpdateClubPayload>,
) -> impl Responder {
    let club_id = path.into_inner();

    // Fetch the current club to apply updates
    let club = match sqlx::query_as::<_, Club>("SELECT * FROM club WHERE id = $1")
        .bind(club_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(club) => club,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(format!("Club with id {} not found", club_id));
        }
        Err(e) => {
            eprintln!("Failed to fetch club for update: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to fetch club for update");
        }
    };

    // Use new values from payload or keep existing values
    let name = payload.name.as_deref().unwrap_or(&club.name);
    let profil_pic = payload.profil_pic.as_deref().unwrap_or(&club.profil_pic);
    let cover_pic = payload.cover_pic.as_deref().unwrap_or(&club.cover_pic);

    match sqlx::query(
        "UPDATE club SET name = $1, profil_pic = $2, cover_pic = $3 WHERE id = $4",
    )
    .bind(name)
    .bind(profil_pic)
    .bind(cover_pic)
    .bind(club_id)
    .execute(&**db_pool)
    .await
    {
        Ok(_) => HttpResponse::Ok().json("Club updated successfully"),
        Err(e) => {
            eprintln!("Failed to update club: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to update club")
        }
    }
}

