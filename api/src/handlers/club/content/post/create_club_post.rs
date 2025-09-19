use actix_web::{web, HttpResponse, Responder};
use crate::models::post_struct::{CreatePostPayload, Post};
use serde_json::json;
use sqlx::PgPool;

/// Handler to create a new post within a specific club.
pub async fn create_club_post(
    db_pool: web::Data<PgPool>,
    club_id: web::Path<i32>,
    payload: web::Json<CreatePostPayload>,
) -> impl Responder {
    let club_id_val = club_id.into_inner();
    let has_discussion = payload.has_discussion.unwrap_or(false);

    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Step 1: Create the post in the `post` table.
    let post_result = sqlx::query_as::<_, (i32, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
        "INSERT INTO post (user_id, content) VALUES ($1, $2) RETURNING id, created_at, updated_at",
    )
    .bind(payload.user_id)
    .bind(&payload.content)
    .fetch_one(&mut *tx)
    .await;

    let post = match post_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to insert post: {:?}", e);
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create post"}));
        }
    };

    let (post_id, created_at, updated_at) = post;

    let mut discussion_id_final = None;

    // Step 2: If requested, create a discussion and link it to the post.
    if has_discussion {
        let discussion_id_result = sqlx::query_scalar::<_, i32>(
            "INSERT INTO low_discussion DEFAULT VALUES RETURNING id",
        ).fetch_one(&mut *tx).await;

        match discussion_id_result {
            Ok(discussion_id) => {
                discussion_id_final = Some(discussion_id);
                if let Err(e) = sqlx::query("INSERT INTO post_discussion (post_id, discussion_id) VALUES ($1, $2)")
                    .bind(post_id)
                    .bind(discussion_id)
                    .execute(&mut *tx)
                    .await {
                        eprintln!("Failed to link post to discussion: {:?}", e);
                        let _ = tx.rollback().await;
                        return HttpResponse::InternalServerError().json("Failed to link discussion");
                    }
            },
            Err(e) => {
                eprintln!("Failed to create discussion: {:?}", e);
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().json("Failed to create discussion");
            }
        }
    }

    // Step 3: Link the post to the club in the `club_post` table.
    if let Err(e) = sqlx::query("INSERT INTO club_post (post_id, club_id, has_discussion) VALUES ($1, $2, $3)")
        .bind(post_id)
        .bind(club_id_val)
        .bind(has_discussion)
        .execute(&mut *tx)
        .await {
        eprintln!("Failed to link post to club: {:?}", e);
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to link post to club."}));
    }

    // Step 4: Commit the transaction.
    match tx.commit().await {
        Ok(_) => {
            let created_post = Post {
                id: post_id,
                content: payload.content.clone(),
                user_id: payload.user_id,
                has_discussion,
                discussion_id: discussion_id_final,
                created_at,
                updated_at,
            };
            HttpResponse::Created().json(created_post)
        }
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to save post")
        }
    }
}
