use crate::models::postStruct::{CreatePostPayload, Post};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

pub async fn create_club_community_post(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This is club_id from the URL
    payload: web::Json<CreatePostPayload>,
) -> impl Responder {
    let has_discussion = payload.has_discussion.unwrap_or(false);
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
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Step 1: Create the post in the `post` table.
    let post_result = sqlx::query_as::<_, Post>(
        "INSERT INTO post (created_by, content, has_discussion) VALUES ($1, $2, $3) RETURNING id, content, created_by, has_discussion, NULL as discussion_id",
    )
    .bind(payload.created_by)
    .bind(&payload.content)
    .bind(has_discussion)
    .fetch_one(&mut *tx)
    .await;

    let mut post = match post_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to insert post: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to create post");
        }
    };

    // Step 2: If requested, create a discussion and link it to the post.
    if has_discussion {
        let discussion_id_result = sqlx::query_scalar::<_, i32>(
            "INSERT INTO low_discussion DEFAULT VALUES RETURNING id",
        )
        .fetch_one(&mut *tx)
        .await;

        match discussion_id_result {
            Ok(discussion_id) => {
                if let Err(e) = sqlx::query("INSERT INTO post_discussion (post_id, discussion_id) VALUES ($1, $2)")
                    .bind(post.id)
                    .bind(discussion_id)
                    .execute(&mut *tx)
                    .await {
                        eprintln!("Failed to link post to discussion: {:?}", e);
                        return HttpResponse::InternalServerError().json("Failed to link discussion");
                    }
                post.discussion_id = Some(discussion_id);
            },
            Err(e) => {
                eprintln!("Failed to create discussion: {:?}", e);
                return HttpResponse::InternalServerError().json("Failed to create discussion");
            }
        }
    }

    // Step 3: Link the post to the community in the `club_community_post` table.
    if let Err(e) = sqlx::query("INSERT INTO club_community_post (post_id, club_community_id) VALUES ($1, $2)")
        .bind(post.id)
        .bind(community_id)
        .execute(&mut *tx)
        .await {
        eprintln!("Failed to link post to community: {:?}", e);
        return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to link post to community."}));
    }

    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(post),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to save post")
        }
    }
}