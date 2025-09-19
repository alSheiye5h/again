use crate::models::post_struct::{CreatePostPayload, Post};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

pub async fn create_community_post(
    db_pool: web::Data<PgPool>,
    community_id: web::Path<i32>,
    payload: web::Json<CreatePostPayload>,
) -> impl Responder {
    let has_discussion = payload.has_discussion.unwrap_or(false);
    let community_id = community_id.into_inner();

    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Step 1: Create the post in the `post` table.
    let post_result = sqlx::query_as::<_, Post>(
        "INSERT INTO post (user_id, content, has_discussion) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(payload.user_id)
    .bind(&payload.content)
    .bind(has_discussion)
    .fetch_one(&mut *tx)
    .await;

    let post = match post_result {
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
            },
            Err(e) => {
                eprintln!("Failed to create discussion: {:?}", e);
                return HttpResponse::InternalServerError().json("Failed to create discussion");
            }
        }
    }

    // Step 3: Link the post to the community in the `community_post` table.
    if let Err(e) = sqlx::query("INSERT INTO community_post (post_id, community_id) VALUES ($1, $2)")
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