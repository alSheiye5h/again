use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use api::models::userStruct::User;
use serde_json::json;

pub async fn register_user(
    db_pool: web::Data<PgPool>, 
    user: web::Json<User>
) -> impl Responder {

    let user_data = user.into_inner();

    // Insert into database (without last_seen)
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, name, email, password, profil_pic, bio)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        user_data.username,
        user_data.name,
        user_data.email,
        user_data.password,
        user_data.profil_pic,
        user_data.bio,
    )
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(json!({
            "status": "success",
            "user_id": record.id
        })),
        Err(err) => {
            eprintln!("Database insert error: {:?}", err);
            HttpResponse::InternalServerError().body("Error inserting user")
        }
    }
}
