use actix_web::{web, HttpResponse, Responder};
use crate::models::Team_struct::UpdateTeamPayload;

pub async fn update_team(path: web::Path<i32>, _payload: web::Json<UpdateTeamPayload>) -> impl Responder {
    let team_id = path.into_inner();
    HttpResponse::Ok().body(format!("Updating team {} (not implemented)", team_id))
}