use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use api::models::Ama_pool_struct::Pool;
use api::models::Club_struct::CreateCommunityPoolPayload;

/// Handler to create a new Pool for a club's community.
pub async fn create_club_community_pool(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This is club_id from the URL
    payload: web::Json<CreateCommunityPoolPayload>,
) -> impl Responder {
    let club_id = path.into_inner();

    // First, get the community_id from the club_id
    let community_id_result: Result<Option<(i32,)>, sqlx::Error> = sqlx::query_as("SELECT id FROM club_community WHERE club_id = $1")
        .bind(club_id)
        .fetch_optional(&**db_pool)
        .await;

    let community_id = match community_id_result {
        Ok(Some((id,))) => id,
        Ok(None) => return HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found for this club."})),
        Err(_) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to check community existence."})),
    };

    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to start transaction"}));
        }
    };

    // Step 1: Create the Pool in the central `pool` table.
    let pool_result = sqlx::query_as::<_, Pool>(
        "INSERT INTO pool (created_by) VALUES ($1) RETURNING id, created_by",
    )
    .bind(payload.created_by)
    .fetch_one(&mut *tx)
    .await;

    let pool = match pool_result {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to insert into pool table: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create Pool."}));
        }
    };

    // Step 2: Link the new Pool to the community in the `club_community_pool` table.
    if let Err(e) = sqlx::query("INSERT INTO club_community_pool (community_id, pool_id) VALUES ($1, $2)")
        .bind(community_id)
        .bind(pool.id)
        .execute(&mut *tx)
        .await {
        eprintln!("Failed to link Pool to community: {:?}", e);
        return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to link Pool to community."}));
    }

    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(pool),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to save club community Pool."}))
        }
    }
}