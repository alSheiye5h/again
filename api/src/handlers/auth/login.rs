use actix_web::{web, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use sqlx::PgPool;
use api::models::userStruct::UserLogin;
use api::models::userStruct::DbUser;


pub async fn login_user(
    db_pool: web::Data<PgPool>,
    user_data: web::Json<UserLogin>,
) -> impl Responder {
    if user_data.password.trim().is_empty() {
        return HttpResponse::BadRequest().body("Password is required");
    }

    let has_username = user_data.username
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);

    let has_email = user_data.email
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);

    if has_username && has_email {
        return HttpResponse::BadRequest().body("Provide either username or email, not both");
    }

    let query_result = if has_username {
        sqlx::query_as::<_, DbUser>("SELECT id, username, email, password FROM users WHERE username = $1")
            .bind(&user_data.username)
            .fetch_optional(db_pool.get_ref())
            .await
    } else if has_email {
        sqlx::query_as::<_, DbUser>("SELECT id, username, email, password FROM users WHERE email = $1")
            .bind(&user_data.email)
            .fetch_optional(db_pool.get_ref())
            .await
    } else {
        return HttpResponse::BadRequest().body("Either username or email is required");
    };

    let user = match query_result {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().body("User not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database query failed"),
    };

    // SECURITY WARNING: Storing and comparing plaintext passwords is a major vulnerability.
    // 1. When a user registers, hash their password using a library like `bcrypt` or `argon2`
    //    and store the resulting hash in the database, not the plain password.
    // 2. During login, use the library's verification function to compare the provided
    //    password against the stored hash in a secure, constant-time manner.
    //    Example: `bcrypt::verify(user_data.password, &user.password_hash_from_db)`
    if user.password == user_data.password {
        let cookie = Cookie::build("user_id", user.id.to_string())
            .path("/")
            .http_only(true)
            .finish();

        HttpResponse::Ok()
            .cookie(cookie)
            .json(user)
    } else {
        // Use a generic message to avoid confirming whether a user exists.
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}
