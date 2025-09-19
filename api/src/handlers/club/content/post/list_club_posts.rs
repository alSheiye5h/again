use actix_web::{web, HttpResponse, Responder};
use crate::models::post_struct::Post;
use serde_json::json;
use sqlx::PgPool;

pub async fn list_club_posts(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let club_id = path.into_inner();

    // First, check if the club exists to provide a clear error message.
    let club_exists: (bool,) = match sqlx::query_as("SELECT EXISTS(SELECT 1 FROM club WHERE id = $1)")
        .bind(club_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(exists) => exists,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to check club existence."})),
    };

    if !club_exists.0 {
        return HttpResponse::NotFound().json(json!({"status": "error", "message": "Club not found."}));
    }

    match sqlx::query_as::<_, Post>(
        r#"
        SELECT p.id, p.user_id, p.content, p.created_at, p.updated_at, cp.has_discussion, pd.discussion_id
        FROM post p
        JOIN club_post cp ON p.id = cp.post_id
        LEFT JOIN post_discussion pd ON p.id = pd.post_id
        WHERE cp.club_id = $1
        ORDER BY p.created_at DESC
        "#,
    )
    .bind(club_id)
    .fetch_all(&**db_pool)
    .await
    {
        Ok(posts) => HttpResponse::Ok().json(posts), // fetch_all returns an empty Vec if no rows are found, which serializes to []
        Err(e) => {
            eprintln!("Failed to list club posts: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to list posts for the club."}))
        }
    }
}