//! TODO: document this

use actix_web::{post, web::Bytes};

use crate::structs::responses::base::{Content, Response};

/// Invite a player to a clan.
#[post("/clan_manager_update/sec/send_invitation")]
pub async fn send_invitation(bytes: Bytes) -> Response<()> {
    log::warn!("TODO: Implement retrieve_announcements");
    log::debug!("{}", String::from_utf8_lossy(&bytes));

    Response::success(Content::Empty)
}

/// Request to join a clan.
#[post("/clan_manager_update/sec/request_membership")]
pub async fn request_membership(bytes: Bytes) -> Response<()> {
    log::warn!("TODO: Implement request_membership");
    log::debug!("{}", String::from_utf8_lossy(&bytes));

    Response::success(Content::Empty)
}