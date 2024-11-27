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
            player::{Jid, Status},
        }, requests::{base::Request, clans::{ClanSearch, CreateClan, DisbandClan, GetClanInfo, GetClanList, UpdateClanInfo}}, responses::{
            base::{Content, List, Response},
            entities::{ClanInfo, ClanPlayerInfo, ClanSearchInfo, IdEntity}, error::ErrorCode,
        }
    },
};

/// View basic information about a clan.
#[post("/clan_manager_view/func/get_clan_info")]
pub async fn get_clan_info(database: Data<Database>, req: Request<GetClanInfo>) -> Response<ClanInfo> {
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await
    else { return Response::error(ErrorCode::InternalServerError) };

    if clan.is_none() {
        return Response::error(ErrorCode::NoSuchClan);
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
    else { return Response::error(ErrorCode::InternalServerError) };

    // Collect all valid entries
    let mut data: Vec<Clan> = vec![];
    while let Some (clan) = clans.next().await {
        match clan {
            Ok(clan) => data.push(clan),
            Err(e) => log::error!("Error while fetching clan: {}", e),
        }
    }

    let data_len = data.len();

    // Format them from the perspective of the player
    let items: Vec<ClanPlayerInfo> = data
        .into_iter()
        .skip((req.request.start - 1).max(0) as usize)
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
        .skip((req.request.start - 1).max(0) as usize)
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
pub async fn create_clan(database: Data<Database>, req: Request<CreateClan>) -> Response<IdEntity> {
    let clan = Clan::from(req.request);

    // Save the clan to the database.
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Item(clan.into()))
}

/// Disband a clan.
/// 
/// - The author needs to:
///    - Be the owner of the clan
#[post("/clan_manager_update/sec/disband_clan")]
pub async fn disband_clan(database: Data<Database>, req: Request<DisbandClan>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    let clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to disband the clan
    let Some(owner) = clan.owner()
    else { return Response::error(ErrorCode::InternalServerError) };

    if owner.jid != jid {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Disband the clan
    if let Err(e) = clan.delete(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Update a member's information in a clan.
/// 
/// - The author needs to:
///     - Be a member of the clan
#[post("/clan_manager_update/sec/update_clan_info")]
pub async fn update_clan_info(database: Data<Database>, req: Request<UpdateClanInfo>) -> Response<()> {
    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to update the clan's info
    if !clan.status_of(&Jid::from(req.request.ticket)).map_or(false, |status| status == &Status::Member) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Update the clan's info
    clan.description = req.request.description;

    // Save the updated clan to the database
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}