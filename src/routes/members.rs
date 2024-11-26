//! Routes pertaining to a clan's members, such as:
//! 
//! - Getting a clan's members
//! - Inviting a player to a clan
//! - Removing a player from a clan
//! - ...

use actix_web::post;

use crate::structs::{entities::player::{Jid, Player, Role, Status}, responses::{Content, Entity, List, Response}};

/// Get a clan's members.
#[post("/clan_manager_view/sec/get_member_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_member_list() -> Response<Player> {
    log::warn!("TODO: Implement get_clan_members");

    let items = vec![
        Entity::Entry(Player {
            jid: Jid::from("debug"),
            role: Role::Leader,
            status: Status::Member,
            description: String::from("Description"),
        })
    ];

    let list = List {
        results: items.len() as u32,
        total: items.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}