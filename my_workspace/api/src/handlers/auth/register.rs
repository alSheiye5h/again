use actix_web::{HttpResponse, Responder};

use crate::models::UserStruct;

pub async fn get_user() -> impl Responder {
    let body_response = UserStruct {
        name: String::from("anas"),
        age: 21,
    };

    HttpResponse::Ok().json(body_response) 
}