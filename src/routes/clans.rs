//! Routes pertaining to clans, such as:
//! 
//! - Creating a clan
//! - Searching for a clan
//! - Viewing a clan
//! - Editing a clan
//! - ...

use actix_web::post;

use crate::structs::{entities::clan::{BasicInfo, Clan, Id}, responses::{Content, Entity, List, Response}};

/// View basic information about a clan.
#[post("/clan_manager_view/func/get_clan_info")]
pub async fn get_clan_info() -> Response<Clan> {
    log::warn!("TODO: Implement get_clan_info");

    let clan = Clan {
        id: 0,
        name: String::from("Clan Name"),
        tag: String::from("TAG"),
        description: String::from("Description"),
        members: 0,
        date_created: chrono::Utc::now(),
        int_attr1: 0,
        int_attr2: 0,
        int_attr3: 0,
        auto_accept: false,
    };

    Response::success(Content::Item(Entity::Info(clan)))
}

/// Get a list of clans.
#[post("/clan_manager_view/sec/get_clan_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_clan_list() -> Response<Clan> {
    log::warn!("TODO: Implement get_clan_list");

    let items = vec![
        Entity::Info(Clan {
            id: 0,
            name: String::from("Clan Name"),
            tag: String::from("TAG"),
            description: String::from("Description"),
            members: 0,
            date_created: chrono::Utc::now(),
            int_attr1: 0,
            int_attr2: 0,
            int_attr3: 0,
            auto_accept: false,
        })
    ];

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
pub async fn clan_search() -> Response<BasicInfo> {
    log::warn!("TODO: Implement clan_search");

    let items = vec![
        Entity::Info(BasicInfo {
            id: 0,
            name: String::from("Clan Name"),
            tag: String::from("TAG"),
            members: 0,
        })
    ];

    let list = List {
        results: items.len() as u32,
        total: items.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Create a clan.
#[post("/clan_manager_view/sec/create_clan")]
pub async fn create_clan() -> Response<Id> {
    log::warn!("TODO: Implement create_clan");

    let id = 0;

    Response::success(Content::Item(Entity::Id(id)))
}