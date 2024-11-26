//! Routes pertaining to a clan's members, such as:
//!
//! - Getting a clan's members
//! - Inviting a player to a clan
//! - Removing a player from a clan
//! - ...

use actix_web::post;
use mongodb::bson::doc;

use crate::structs::{
    entities::clan::Clan, requests::{base::Request, members::GetMemberList}, responses::{
        base::{Content, List, Response}, entities::PlayerInfo
    }
};

/// Get a clan's members.
#[post("/clan_manager_view/sec/get_member_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_member_list(req: Request<GetMemberList>, clan: Clan) -> Response<PlayerInfo> {
    // Collect all valid entries
    let items = clan.members
        .iter()
        .skip((req.request.start - 1) as usize)
        .take(req.request.max as usize)
        .map(|m| PlayerInfo::from(m.to_owned()))
        .collect::<Vec<PlayerInfo>>();

    let list = List {
        results: items.len() as u32,
        total: clan.members.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}