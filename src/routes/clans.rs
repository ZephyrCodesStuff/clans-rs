//! Routes pertaining to clans, such as:
//!
//! - Creating a clan
//! - Searching for a clan
//! - Viewing a clan
//! - Editing a clan
//! - ...

use actix_web::{post, web::Data};

use crate::{
    database::Database,
    structs::{
        entities::{
            clan::Clan,
            player::{Jid, Player, Role, Status},
        },
        responses::{
            base::{Content, List, Response},
            entities::{ClanId, ClanInfo, ClanPlayerInfo, ClanSearchInfo},
        },
    },
};

/// View basic information about a clan.
#[post("/clan_manager_view/func/get_clan_info")]
pub async fn get_clan_info() -> Response<ClanInfo> {
    log::warn!("TODO: Implement get_clan_info");

    let clan = Clan {
        id: 0,
        name: String::from("Clan Name"),
        tag: String::from("TAG"),
        description: String::from("Description"),
        members: vec![],
        blacklist: vec![],
        date_created: chrono::Utc::now(),
        last_updated: chrono::Utc::now(),
        int_attr1: 0,
        int_attr2: 0,
        int_attr3: 0,
        auto_accept: false,
    };

    let info = ClanInfo::from(clan);
    Response::success(Content::Item(info))
}

/// Get a list of clans.
#[post("/clan_manager_view/sec/get_clan_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_clan_list(database: Data<Database>) -> Response<ClanPlayerInfo> {
    log::warn!("TODO: Implement get_clan_list");

    let data = vec![Clan {
        id: 0,
        name: String::from("Clan Name"),
        tag: String::from("TAG"),
        description: String::from("Description"),
        members: vec![],
        blacklist: vec![],
        date_created: chrono::Utc::now(),
        last_updated: chrono::Utc::now(),
        int_attr1: 0,
        int_attr2: 0,
        int_attr3: 0,
        auto_accept: false,
    }];

    let player = Player {
        jid: Jid::from("debug"),
        role: Role::Leader,
        status: Status::Member,
        allow_msg: false,
        description: String::from("Description"),
    };

    let items: Vec<ClanPlayerInfo> = data
        .into_iter()
        .map(|clan| ClanPlayerInfo::from((clan, player.clone())))
        .collect();

    let list = List {
        results: items.len() as u32,
        total: items.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Search for a clan.
#[post("/clan_manager_view/sec/clan_search")]
#[allow(clippy::cast_possible_truncation)]
pub async fn clan_search() -> Response<ClanSearchInfo> {
    log::warn!("TODO: Implement clan_search");

    let data = vec![Clan {
        id: 0,
        name: String::from("Clan Name"),
        tag: String::from("TAG"),
        description: String::from("Description"),
        members: vec![],
        blacklist: vec![],
        date_created: chrono::Utc::now(),
        last_updated: chrono::Utc::now(),
        int_attr1: 0,
        int_attr2: 0,
        int_attr3: 0,
        auto_accept: false,
    }];

    let items: Vec<ClanSearchInfo> = data.into_iter().map(ClanSearchInfo::from).collect();

    let list = List {
        results: items.len() as u32,
        total: items.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Create a clan.
#[post("/clan_manager_view/sec/create_clan")]
pub async fn create_clan() -> Response<ClanId> {
    log::warn!("TODO: Implement create_clan");

    let id = 0;

    let clan_id = ClanId::from(id);
    Response::success(Content::Item(clan_id))
}
