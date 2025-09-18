use actix_web::web;

// Event management
use crate::handlers::event::charity::{
    create_event::create_event, 
    list_events::list_events, 
    get_event::get_event_by_id, 
    update_event::update_event, 
    delete_event::delete_event
};
use crate::handlers::event::charity::rsvp::{
    configure_rsvp::configure_rsvp as configure_charity_rsvp,
    create_rsvp::create_or_update_rsvp as create_or_update_charity_rsvp,
    delete_rsvp::delete_rsvp as delete_charity_rsvp,
    get_rsvps::list_rsvps_for_event as list_charity_rsvps_for_event,
    get_rsvp_choices::get_rsvp_choices as get_charity_rsvp_choices,
};
use crate::handlers::event::regular::{
    create_event::create_event as create_regular_event, 
    list_events::list_events as list_regular_events, 
    get_event::get_event_by_id as get_regular_event_by_id, 
    update_event::update_event as update_regular_event,
    delete_event::delete_event as delete_regular_event
};
use crate::handlers::event::regular::rsvp::{
    configure_rsvp::configure_rsvp as configure_regular_rsvp,
    create_rsvp::create_or_update_rsvp as create_or_update_regular_rsvp,
    delete_rsvp::delete_rsvp as delete_regular_rsvp,
    get_rsvps::list_rsvps_for_event as list_regular_rsvps_for_event,
    get_rsvp_choices::get_rsvp_choices as get_regular_rsvp_choices,
};
use crate::handlers::event::tournament::{
    create_event::create_event as create_tournament_event, 
    list_events::list_events as list_tournament_events, 
    get_event::get_event_by_id as get_tournament_event_by_id, 
    update_event::update_event as update_tournament_event,
    delete_event::delete_event as delete_tournament_event
};
use crate::handlers::event::tournament::rsvp::{
    configure_rsvp::configure_rsvp,
    create_rsvp::create_or_update_rsvp,
    delete_rsvp::delete_rsvp,
    get_rsvps::list_rsvps_for_event,
    get_rsvp_choices::get_rsvp_choices,
};
 
pub fn event_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/events")
            .service(web::scope("/charity")
                .route("", web::post().to(create_event))
                .route("", web::get().to(list_events))
                .route("/{id}", web::get().to(get_event_by_id))
                .route("/{id}", web::put().to(update_event))
                .route("/{id}", web::delete().to(delete_event))
                // Charity Event RSVP routes
                .service(web::scope("/{id}/rsvp")
                    .route("", web::get().to(list_charity_rsvps_for_event))
                    .route("", web::post().to(create_or_update_charity_rsvp)) // for user
                    .route("/configure", web::post().to(configure_charity_rsvp)) // for organizer
                    .route("/configure", web::get().to(get_charity_rsvp_choices))
                    .route("/{user_id}", web::delete().to(delete_charity_rsvp))
                )
        )        
            .service(web::scope("/regular")
                .route("", web::post().to(create_regular_event))
                .route("", web::get().to(list_regular_events))
                .route("/{id}", web::get().to(get_regular_event_by_id))
                .route("/{id}", web::put().to(update_regular_event))
                .route("/{id}", web::delete().to(delete_regular_event))
                // Regular Event RSVP routes
                .service(web::scope("/{id}/rsvp")
                    .route("", web::get().to(list_regular_rsvps_for_event))
                    .route("", web::post().to(create_or_update_regular_rsvp))
                    .route("/configure", web::post().to(configure_regular_rsvp))
                    .route("/configure", web::get().to(get_regular_rsvp_choices))
                    .route("/{user_id}", web::delete().to(delete_regular_rsvp))
                )
        )   
            .service(web::scope("/tournament")
                .route("", web::post().to(create_tournament_event))
                .route("", web::get().to(list_tournament_events))
                .route("/{id}", web::get().to(get_tournament_event_by_id))
                .route("/{id}", web::put().to(update_tournament_event))
                .route("/{id}", web::delete().to(delete_tournament_event))
                // Tournament RSVP routes
                .service(web::scope("/{id}/rsvp")
                    .route("", web::get().to(list_rsvps_for_event))
                    .route("", web::post().to(create_or_update_rsvp))
                    .route("/configure", web::post().to(configure_rsvp))
                    .route("/configure", web::get().to(get_rsvp_choices))
                    .route("/{user_id}", web::delete().to(delete_rsvp))
                )
        ));
}