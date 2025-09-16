use actix_web::web;
use crate::handlers::team::{
    create_team::create_team, delete_team::delete_team, get_team::get_team,
    list_teams::list_teams, update_team::update_team,
};
use crate::handlers::team::member::{
    add_member::add_member, delete_member::delete_member, list_members::list_members,
    update_member::update_member,
};


pub fn team_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/team", web::post().to(create_team))
       .route("/team", web::get().to(list_teams))
       .route("/team/{id}", web::get().to(get_team))
       .route("/team/{id}", web::put().to(update_team))
       .route("/team/{id}", web::delete().to(delete_team));
}