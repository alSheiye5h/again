use actix_web::web;

// Event management
use crate::handlers::event::charity::{
    create_event::create_event, 
    list_events::list_events, 
    get_event::get_event_by_id, 
    update_event::update_event, 
    delete_event::delete_event
};
use crate::handlers::event::regular::{
    create_event::create_event as create_regular_event, 
    list_events::list_events as list_regular_events, 
    get_event::get_event_by_id as get_regular_event_by_id, 
    update_event::update_event as update_regular_event,
    delete_event::delete_event as delete_regular_event
};
use crate::handlers::event::tournament::{
    create_event::create_event as create_tournament_event, 
    list_events::list_events as list_tournament_events, 
    get_event::get_event_by_id as get_tournament_event_by_id, 
    update_event::update_event as update_tournament_event,
    delete_event::delete_event as delete_tournament_event
};
 
pub fn event_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/events")
            .service(web::scope("/charity")
                .route("", web::post().to(create_event))
                .route("", web::get().to(list_events))
                .route("/{id}", web::get().to(get_event_by_id))
                .route("/{id}", web::put().to(update_event))
                .route("/{id}", web::delete().to(delete_event))
        )        
            .service(web::scope("/regular")
                .route("", web::post().to(create_regular_event))
                .route("", web::get().to(list_regular_events))
                .route("/{id}", web::get().to(get_regular_event_by_id))
                .route("/{id}", web::put().to(update_regular_event))
                .route("/{id}", web::delete().to(delete_regular_event))
        )   
            .service(web::scope("/tournament")
                .route("", web::post().to(create_tournament_event))
                .route("", web::get().to(list_tournament_events))
                .route("/{id}", web::get().to(get_tournament_event_by_id))
                .route("/{id}", web::put().to(update_tournament_event))
                .route("/{id}", web::delete().to(delete_tournament_event))
        ));
}