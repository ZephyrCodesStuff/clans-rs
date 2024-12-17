//! Routes pertaining to clans, such as:
//!
//! - Creating a clan
//! - Searching for a clan
//! - Viewing a clan
//! - Editing a clan
//! - ...

use actix_web::{post, web::Data};
use futures_util::StreamExt;
use mongodb::bson::doc;

use crate::{
    database::Database,
    structs::{
        entities::{
            clan::{Clan, MAX_CLAN_OWNERSHIP},
            player::{Jid, Role, Status},
        }, requests::{base::Request, clans::{ClanSearch, ClanSearchFilterOperator, CreateClan, DisbandClan, GetClanInfo, GetClanList, UpdateClanInfo}}, responses::{
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
pub async fn clan_search(database: Data<Database>, req: Request<ClanSearch>) -> Response<ClanSearchInfo> {
    // Find all the clans
    let Ok(mut clans) = database.clans.find(doc! {}).await
    else { return Response::error(ErrorCode::InternalServerError) };

    // Collect all valid entries
    let mut data: Vec<Clan> = vec![];
    while let Some (clan) = clans.next().await {
        if clan.is_err() { continue; }
        let clan = clan.unwrap();

        // Check if the clan matches the search criteria
        if req.request.filter.is_none() {
            data.push(clan);
            continue;
        }

        let mut clan_name = clan.name.to_lowercase();
        clan_name = clan_name.trim().to_string();

        let mut filter_value = req.request.filter.as_ref().unwrap().name.value.to_lowercase();
        filter_value = filter_value.trim_end_matches(&clan.platform.to_string().to_lowercase()).to_string();
        filter_value = filter_value.trim().to_string();

        let filter = req.request.filter.as_ref().unwrap();
        let predicate = match filter.name.operator {
            ClanSearchFilterOperator::All => true,
            ClanSearchFilterOperator::Equal => clan_name == filter_value,
            ClanSearchFilterOperator::NotEqual => clan_name != filter_value,
            ClanSearchFilterOperator::GreaterThan | ClanSearchFilterOperator::GreaterThanOrEqual => clan_name.starts_with(&filter_value),
            ClanSearchFilterOperator::LessThan | ClanSearchFilterOperator::LessThanOrEqual => clan_name.ends_with(&filter_value),
            ClanSearchFilterOperator::Like => clan_name.contains(&filter_value),
        };

        if predicate {
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
    let author = Jid::from(req.request.ticket.clone());
    let clan = Clan::from(req.request);

    // Find all the clans where the author is a leader
    let Ok(clans) = database.clans.find(
        doc! { "members.jid": author.to_string(), "members.role": Role::Leader.to_string() }
    ).await
    else { return Response::error(ErrorCode::InternalServerError) };

    // Check if the author already owns too many clans
    if clans.count().await >= MAX_CLAN_OWNERSHIP {
        return Response::error(ErrorCode::ClanLeaderLimitReached);
    }

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