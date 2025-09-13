use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::{Club, CreateClubPayload};
use sqlx::PgPool;

/// Handler to create a new club.
/// This will also add the creator as the first staff member.
pub async fn create_club(
    db_pool: web::Data<PgPool>,
    payload: web::Json<CreateClubPayload>,
) -> impl Responder {
    // Start a transaction to ensure all-or-nothing creation.
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Step 1: Insert the new club into the `club` table.
    let club_result = sqlx::query_as::<_, Club>(
        "INSERT INTO club (name, profil_pic, cover_pic, created_by) VALUES ($1, $2, $3, $4) RETURNING id, name, profil_pic, cover_pic, created_by",
    )
    .bind(&payload.name)
    .bind(&payload.profil_pic)
    .bind(&payload.cover_pic)
    .bind(payload.created_by)
    .fetch_one(&mut *tx)
    .await;

    let club = match club_result {
        Ok(club) => club,
        Err(e) => {
            eprintln!("Failed to create club: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to create club.");
        }
    };

    // Step 2: Create a default community for the new club.
    let community_name = format!("{} Community", &club.name);
    let community_description = format!("The official community for the {} club.", &club.name);
    if let Err(e) = sqlx::query(
        "INSERT INTO club_community (club_id, name, description, created_by) VALUES ($1, $2, $3, $4)",
    )
    .bind(club.id)
    .bind(&community_name)
    .bind(&community_description)
    .bind(payload.created_by)
    .execute(&mut *tx)
    .await
    {
        eprintln!("Failed to create default community for club: {:?}", e);
        return HttpResponse::InternalServerError().json("Failed to create default community for club.");
    }

    // Step 3: Add the creator as the first staff member.
    if let Err(e) = sqlx::query(
        "INSERT INTO club_staff (user_id, club_id, promoted_by) VALUES ($1, $2, $3)",
    )
    .bind(payload.created_by)
    .bind(club.id)
    .bind(payload.created_by) // The creator promotes themselves.
    .execute(&mut *tx)
    .await
    {
        eprintln!("Failed to add creator to club staff: {:?}", e);
        return HttpResponse::InternalServerError().json("Failed to set club creator as staff.");
    }

    // Step 4: Commit the transaction and return the new club.
    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(club),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to save club.")
        }
    }
}
