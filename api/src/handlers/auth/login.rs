use actix_web::{web, HttpResponse, Responder};
use api::models::userStruct::UserLogin; // Corrected the import path to reference the nested module

// pub async fn login_user(user: web::Json<UserLogin>) -> impl Responder {
//     let body_response = UserLogin {
//         username: user.username.clone(),
//         email: user.email.clone(),
//         password: user.password.clone(),
//     };

//     HttpResponse::Ok().json(body_response)
// }

pub async fn login_user(user: web::Json<UserLogin>) -> impl Responder {
    if user.password.trim().is_empty() {
        return HttpResponse::BadRequest().body("Password is required");
    }

    let has_username = !user.username.trim().is_empty();
    let has_email = !user.email.trim().is_empty();

    match (has_username, has_email) {
        (true, false) => HttpResponse::Ok().json(format!("Logging in with username: {}", user.username)),
        (false, true) => HttpResponse::Ok().json(format!("Logging in with email: {}", user.email)),
        (true, true) => HttpResponse::BadRequest().body("Provide either username or email, not both"),
        (false, false) => HttpResponse::BadRequest().body("Either username or email is required"),
    }
}