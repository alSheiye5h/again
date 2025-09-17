use crate::handlers::discussion::{
    create_discussion::create_discussion, delete_discussion::delete_discussion,
    get_discussion::get_discussion, list_discussions::list_discussions,
    update_discussion::update_discussion,
};
use crate::handlers::discussion::member::{
    add_discussion_member::add_discussion_member,
    delete_discussion_member::remove_discussion_member,
    get_discussion_member::get_discussion_member,
    list_discussion_members::list_discussion_members,
    update_discussion_member::update_discussion_member_role,
};
use crate::handlers::discussion::content::message::{
    create_discussion_message::create_discussion_message,
    delete_discussion_message::delete_discussion_message,
    get_discussion_message::get_discussion_message,
    list_discussion_message::list_discussion_messages,
    update_discussion_message::update_discussion_message,
};
use crate::handlers::discussion::content::announcement::{
    create_discussion_announcement::link_discussion_announcement,
    delete_discussion_announcement::unlink_discussion_announcement,
    get_discussion_announcements::get_linked_discussion_announcement,
    list_discussion_announcement::list_linked_discussion_announcements,
};
use actix_web::web;

pub fn discussion_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/discussions")
            .route("", web::get().to(list_discussions))
            .route("", web::post().to(create_discussion))
            .service(
                web::scope("/{discussion_id}")
                    .route("", web::get().to(get_discussion))
                    .route("", web::put().to(update_discussion))
                    .route("", web::delete().to(delete_discussion))
                    // Discussion Members
                    .route("/members", web::get().to(list_discussion_members))
                    .route("/members", web::post().to(add_discussion_member))
                    .route("/members/{user_id}", web::get().to(get_discussion_member))
                    .route("/members/{user_id}", web::put().to(update_discussion_member_role))
                    .route("/members/{user_id}", web::delete().to(remove_discussion_member))
                    // Discussion Messages
                    .route("/messages", web::get().to(list_discussion_messages))
                    .route("/messages", web::post().to(create_discussion_message))
                    .route("/messages/{message_id}", web::get().to(get_discussion_message))
                    .route("/messages/{message_id}", web::put().to(update_discussion_message))
                    .route("/messages/{message_id}", web::delete().to(delete_discussion_message))
                    // Discussion Announcements (linking existing announcements)
                    .route("/announcements", web::get().to(list_linked_discussion_announcements))
                    .route("/announcements/{announcement_id}", web::post().to(link_discussion_announcement))
                    .route("/announcements/{announcement_id}", web::get().to(get_linked_discussion_announcement))
                    .route("/announcements/{announcement_id}", web::delete().to(unlink_discussion_announcement)),
            ),
    );
}