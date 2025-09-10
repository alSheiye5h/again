// use actix_web::{web, HttpResponse, Responder};
// use api::models::userStruct::User; // Corrected the import path to reference the nested module

// pub async fn register_user(user: web::Json<User>) -> impl Responder {
//     let body_response = User {
//         id: user.id,
//         username: user.username.clone(),
//         name: user.name.clone(),
//         email: user.email.clone(),
//         password: user.password.clone(),
//         profil_pic: user.profil_pic.clone(),
//         bio: user.bio.clone(),
//         last_seen: user.last_seen.clone(),
//     };

//     HttpResponse::Ok().json(body_response)
// }
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use api::models::userStruct::User;

pub async fn register_user(
    db_pool: web::Data<PgPool>, 
    user: web::Json<User>
) -> impl Responder {

    let user_data = user.into_inner();

    // Insert into database
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, name, email, password, profil_pic, bio, last_seen)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
        user_data.username,
        user_data.name,
        user_data.email,
        user_data.password,
        user_data.profil_pic,
        user_data.bio,
        user_data.last_seen,
    )
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "user_id": record.id
        })),
        Err(err) => {
            eprintln!("Database insert error: {:?}", err);
            HttpResponse::InternalServerError().body("Error inserting user")
        }
    }
}
