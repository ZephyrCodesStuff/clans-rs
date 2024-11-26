//! Routes pertaining to a clan's blacklist, such as:
//!
//! - Getting a clan's blacklist
//! - Adding a player to a clan's blacklist
//! - Removing a player from a clan's blacklist
//! - ...

use actix_web::post;

use crate::structs::{
    entities::player::{Jid, Player, Role, Status},
    responses::{
        base::{Content, List, Response},
        entities::BlacklistEntry,
    },
};

/// Get a clan's blacklist.
#[post("/clan_manager_view/sec/get_blacklist")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_blacklist() -> Response<BlacklistEntry> {
    log::warn!("TODO: Implement get_blacklist");

    let data = vec![Player {
        jid: Jid::from("debug"),
        role: Role::Leader,
        status: Status::Member,
        allow_msg: false,
        description: String::from("Description"),
    }];

    let items: Vec<BlacklistEntry> = data.into_iter().map(BlacklistEntry::from).collect();

    let list = List {
        results: items.len() as u32,
        total: items.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Add a player to a clan's blacklist.
#[post("/clan_manager_update/sec/record_blacklist_entry")]
pub async fn record_blacklist_entry() -> Response<()> {
    log::warn!("TODO: Implement record_blacklist_entry");

    Response::success(Content::Empty)
}

/// Remove a player from a clan's blacklist.
#[post("/clan_manager_update/sec/delete_blacklist_entry")]
pub async fn delete_blacklist_entry() -> Response<()> {
    log::warn!("TODO: Implement delete_blacklist_entry");

    Response::success(Content::Empty)
}
