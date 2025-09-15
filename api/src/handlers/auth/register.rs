use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::userStruct::User;
use serde_json::json;
use bcrypt::{hash, DEFAULT_COST};

pub async fn register_user(
    db_pool: web::Data<PgPool>, 
    user: web::Json<User>
) -> impl Responder {

    println!("touch");
    let user_data = user.into_inner();

    // Hash the user's password before storing it
    let hashed_password = match hash(&user_data.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Password hashing error: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to process registration."}));
        }
    };

    // Insert into database
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, name, email, password, profil_pic, bio)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        user_data.username,
        user_data.name,
        user_data.email,
        hashed_password,
        user_data.profil_pic,
        user_data.bio,
    )
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            // Print the record to the console
            println!("Inserted user record: {:?}", record);

            // Return success response
            HttpResponse::Ok().json(json!({
                "status": "success",
                "user_id": record.id
            }))
        }
        Err(err) => {
            // Print the error in the console
            eprintln!("Database insert error: {:?}", err);

            // Return the error in the JSON response
            HttpResponse::Ok().json(json!({
                "status": "error",
                "message": format!("{}", err)
            }))
        }
    }
}