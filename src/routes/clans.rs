//! Routes pertaining to clans, such as:
//!
//! - Creating a clan
//! - Searching for a clan
//! - Viewing a clan
//! - Editing a clan
//! - ...

use actix_web::{post, web::{Bytes, Data}};
use futures_util::StreamExt;
use mongodb::bson::doc;

use crate::{
    database::Database,
    structs::{
        entities::{
            clan::Clan,
            player::Jid,
        }, requests::{base::Request, clans::{CreateClan, GetClanInfo, GetClanList}}, responses::{
            base::{Content, ErrorCode, List, Response},
            entities::{ClanId, ClanInfo, ClanPlayerInfo},
        }
    },
};

/// View basic information about a clan.
#[post("/clan_manager_view/func/get_clan_info")]
pub async fn get_clan_info(database: Data<Database>, req: Request<GetClanInfo>) -> Response<ClanInfo> {
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await
    else { return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR) };

    if clan.is_none() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_BAD_REQUEST);
    }

    let info = ClanInfo::from(clan.unwrap());
    Response::success(Content::Item(info))
}

/// Get a list of clans.
#[post("/clan_manager_view/sec/get_clan_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_clan_list(database: Data<Database>, req: Request<GetClanList>) -> Response<ClanPlayerInfo> {
    let jid = Jid::from(req.request.ticket);

    // Find all the clans
    let Ok(mut clans) = database.clans.find(doc! {}).await
    else { return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR) };

    // Collect all valid entries
    let mut data: Vec<Clan> = vec![];
    while let Some (clan) = clans.next().await {
        if let Ok(clan) = clan {
            data.push(clan);
        }
    }

    let data_len = data.len();

    // Format them from the perspective of the player
    let items: Vec<ClanPlayerInfo> = data
        .into_iter()
        .skip(req.request.start as usize)
        .take(req.request.max as usize)
        .map(|clan| ClanPlayerInfo::from((clan, jid.clone())))
        .collect();

    let list = List {
        results: items.len() as u32,
        total: data_len as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Search for a clan.
#[post("/clan_manager_view/func/clan_search")]
#[allow(clippy::cast_possible_truncation)]
pub async fn clan_search(database: Data<Database>, req: Request<GetClanList>, bytes: Bytes) -> Response<ClanPlayerInfo> {
    log::warn!("TODO: Implement clan_search");
    log::debug!("{}", String::from_utf8_lossy(&bytes));
    
    // TODO: Implement the actual search logic
    
    // Return all clans for now
    let jid = Jid::from(req.request.ticket);

    // Find all the clans
    let Ok(mut clans) = database.clans.find(doc! {}).await
    else { return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR) };

    // Collect all valid entries
    let mut data: Vec<Clan> = vec![];
    while let Some (clan) = clans.next().await {
        if let Ok(clan) = clan {
            data.push(clan);
        }
    }

    let data_len = data.len();

    // Format them from the perspective of the player
    let items: Vec<ClanPlayerInfo> = data
        .into_iter()
        .skip(req.request.start as usize)
        .take(req.request.max as usize)
        .map(|clan| ClanPlayerInfo::from((clan, jid.clone())))
        .collect();

    let list = List {
        results: items.len() as u32,
        total: data_len as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Create a clan.
#[post("/clan_manager_update/sec/create_clan")]
pub async fn create_clan(database: Data<Database>, req: Request<CreateClan>) -> Response<ClanId> {
    let clan = Clan::from(req.request);

    // Save the clan to the database.
    if database.clans.insert_one(&clan).await.is_err() {
        // Could not save the clan to the database.
        // Perhaps a duplicate clan with the same ``id`` already exists?
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR);
    }

    Response::success(Content::Item(clan.into()))
}

/// Disband a clan.
#[post("/clan_manager_update/sec/disband_clan")]
pub async fn disband_clan(bytes: Bytes) -> Response<()> {
    log::warn!("TODO: Implement disband_clan");
    log::debug!("{}", String::from_utf8_lossy(&bytes));

    Response::success(Content::Empty)
}

/// Update a clan's info.
#[post("/clan_manager_update/sec/update_clan_info")]
pub async fn update_clan_info(bytes: Bytes) -> Response<()> {
    log::warn!("TODO: Implement update_clan_info");
    log::debug!("{}", String::from_utf8_lossy(&bytes));

    Response::success(Content::Empty)
}