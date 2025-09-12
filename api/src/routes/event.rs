use actix_web::web;

// Event management
use crate::handlers::event::create_event::create_event;
use crate::handlers::event::delete_event::delete_event;
use crate::handlers::event::get_event::get_event_by_id;
use crate::handlers::event::list_events::list_events;
use crate::handlers::event::update_event::update_event;

pub fn event_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/event", web::post().to(create_event))
       .route("/event", web::get().to(list_events))
       .route("/event/{id}", web::get().to(get_event_by_id))
       .route("/event/{id}", web::put().to(update_event))
       .route("/event/{id}", web::delete().to(delete_event));
}