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
            clan::{Clan, Platform, MAX_CLAN_DESCRIPTION_LENGTH, MAX_CLAN_MEMBERSHIP, MAX_CLAN_NAME_LENGTH, MAX_CLAN_OWNERSHIP, MAX_CLAN_TAG_LENGTH},
            player::{ExtendedJid, Jid, Role, Status},
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
    let jid = Jid::from(req.request.ticket.clone());
    
    // EXTRA: log the player's Jid in the `Players` collection, for future lookups
    let jid_ext = ExtendedJid::from(jid.clone());
    let player = match database.players.find_one(doc! {
        "username": jid_ext.username.clone(),
        "domain": jid_ext.domain.clone(),
        "region": jid_ext.region.clone(),
    }).await {
        Ok(player) => player,
        Err(e) => {
            log::error!("Failed to look-up player `{}` in the database: {}", jid, e);
            return Response::error(ErrorCode::InternalServerError);
        }
    };

    // Store the player's Jid in the database, if it doesn't exist
    if player.is_none() {
        match database.players.insert_one(jid_ext).await {
            Ok(_) => log::info!("Inserted player `{}` into the database", jid),
            Err(e) => log::error!("Failed to log player `{}` into the database: {}", jid, e),
        }
    }

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
    let mut items: Vec<ClanPlayerInfo> = data
        .into_iter()
        .skip((req.request.start - 1).max(0) as usize)
        .take(req.request.max as usize)
        .map(|clan| ClanPlayerInfo::from((clan, jid.clone())))
        .collect();

    // Make sure the game doesn't know they're a member of another clan on a different platform
    let platform = Platform::from(req.request.ticket);
    for c in &mut items {
        if c.status == Status::Member as u32 && c.platform != platform {
            c.role = Role::NonMember as u32;
            c.status = Status::Unknown as u32;
        }
    }

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
    let mut clan = Clan::from(req.request);

    // Limit the clan name and tag to their maximum lengths
    clan.name = clan.name.chars().take(MAX_CLAN_NAME_LENGTH).collect();
    clan.tag = clan.tag.chars().take(MAX_CLAN_TAG_LENGTH).collect();

    // Make sure Unicode chars don't exceed the limits
    if clan.name.as_bytes().len() > MAX_CLAN_NAME_LENGTH || clan.tag.as_bytes().len() > MAX_CLAN_TAG_LENGTH {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check the clans the author is in
    let Ok(cursor) = database.clans.find(doc! { "members.jid": author.to_string() }).await
    else { return Response::error(ErrorCode::InternalServerError) };

    let clans: Vec<Clan> = cursor
        .filter_map(|clan| async move { clan.ok() })
        .collect()
        .await;

    let clans_owned_len = clans.iter().filter(|c| c.owner().map_or(false, |o| o.jid == author)).count();
    let clans_member_len = clans.iter().filter(|c| c.status_of(&author).map_or(false, |s| s == &Status::Member)).count();

    // Check if the author is already in too many clans
    if clans_member_len >= MAX_CLAN_MEMBERSHIP {
        return Response::error(ErrorCode::ClanJoinedLimitReached);
    }

    // Check if the author already owns too many clans
    if clans_owned_len >= MAX_CLAN_OWNERSHIP {
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

    // Update the clan's info, making sure to limit the description's length
    clan.description = req.request.description.chars().take(MAX_CLAN_DESCRIPTION_LENGTH).collect();

    // Save the updated clan to the database
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}