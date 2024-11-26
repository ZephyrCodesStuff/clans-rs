//! TODO: document this

use actix_web::{post, web::Bytes};

use crate::structs::responses::base::{Content, List, Response};

/// Retrieve a clan's announcements.
#[post("/clan_manager_view/sec/retrieve_announcements")]
pub async fn retrieve_announcements(bytes: Bytes) -> Response<()> {
    log::warn!("TODO: Implement retrieve_announcements");
    log::debug!("{}", String::from_utf8_lossy(&bytes));

    let list = List {
        results: 0,
        total: 0,

        items: vec![],
    };

    Response::success(Content::List(list))
}

/// Publish a new announcement for a clan.
#[post("/clan_manager_update/sec/post_announcement")]
pub async fn post_announcement(bytes: Bytes) -> Response<()> {
    log::warn!("TODO: Implement post_announcement");
    log::debug!("{}", String::from_utf8_lossy(&bytes));

    Response::success(Content::Empty)
}