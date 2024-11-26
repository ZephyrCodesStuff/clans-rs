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
        }, requests::{base::Request, clans::{ClanSearch, CreateClan, GetClanList, UpdateClanInfo}}, responses::{
            base::{Content, List, Response},
            error::ErrorCode,
            entities::{ClanId, ClanInfo, ClanPlayerInfo, ClanSearchInfo},
        }
    },
};

/// View basic information about a clan.
#[post("/clan_manager_view/func/get_clan_info")]
pub async fn get_clan_info(clan: Clan) -> Response<ClanInfo> {
    Response::success(Content::Item(ClanInfo::from(clan)))
}

/// Get a list of clans.
#[post("/clan_manager_view/sec/get_clan_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_clan_list(database: Data<Database>, req: Request<GetClanList>, user: Jid) -> Response<ClanPlayerInfo> {
    // Find all the clans
    let Ok(mut clans) = database.clans.find(doc! {}).await
    else { return Response::error(ErrorCode::InternalServerError) };

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
        .skip((req.request.start - 1) as usize)
        .take(req.request.max as usize)
        .map(|clan| ClanPlayerInfo::from((clan, user.clone())))
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
pub async fn clan_search(database: Data<Database>, req: Request<ClanSearch>, bytes: Bytes) -> Response<ClanSearchInfo> {
    log::warn!("TODO: Implement clan_search");
    log::debug!("{}", String::from_utf8_lossy(&bytes));
    
    // TODO: Implement the actual search logic

    // Find all the clans
    let Ok(mut clans) = database.clans.find(doc! {}).await
    else { return Response::error(ErrorCode::InternalServerError) };

    // Collect all valid entries
    let mut data: Vec<Clan> = vec![];
    while let Some (clan) = clans.next().await {
        if let Ok(clan) = clan {
            data.push(clan);
        }
    }

    let data_len = data.len();

    // Format them from the perspective of the player
    let items: Vec<ClanSearchInfo> = data
        .into_iter()
        .skip((req.request.start - 1) as usize)
        .take(req.request.max as usize)
        .map(ClanSearchInfo::from)
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
    // Construct a new clan with a random ID, from the request.
    let clan = Clan::from(req.request);
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Item(clan.into()))
}

/// Disband a clan.
#[post("/clan_manager_update/sec/disband_clan")]
pub async fn disband_clan(database: Data<Database>, clan: Clan, user: Jid) -> Response<()> {
    // Check if the clan has an owner
    // Theoretically, this should never fail.
    let Some(owner) = clan.owner()
    else { return Response::error(ErrorCode::InternalServerError) };

    // Check if the user is allowed to disband the clan (i.e. if they are the owner)
    if owner.jid != user {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Disband the clan
    if let Err(e) = clan.delete(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Update a clan's info.
#[post("/clan_manager_update/sec/update_clan_info")]
pub async fn update_clan_info(database: Data<Database>, req: Request<UpdateClanInfo>, mut clan: Clan, user: Jid) -> Response<()> {
    // Check if the user is allowed to update the clan's info
    if !clan.is_mod(&user) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Update the clan's info
    clan.description = req.request.description;

    // Save the updated clan to the database
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}