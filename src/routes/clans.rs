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
            clan::{
                Clan, Platform, MAX_CLAN_DESCRIPTION_LENGTH, MAX_CLAN_MEMBERSHIP,
                MAX_CLAN_NAME_LENGTH, MAX_CLAN_OWNERSHIP, MAX_CLAN_TAG_LENGTH,
            },
            player::{ExtendedJid, Jid, Role, Status},
        },
        requests::{
            base::Request,
            clans::{
                ClanSearch, CreateClan, DisbandClan, GetClanInfo, GetClanList, UpdateClanInfo,
            },
        },
        responses::{
            base::{Content, List, Response},
            entities::{ClanInfo, ClanPlayerInfo, ClanSearchInfo, IdEntity},
            error::ErrorCode,
        },
    },
};

/// View basic information about a clan.
#[post("/clan_manager_view/func/get_clan_info")]
pub async fn get_clan_info(
    database: Data<Database>,
    req: Request<GetClanInfo>,
) -> Response<ClanInfo> {
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await else {
        return Response::error(ErrorCode::InternalServerError);
    };

    if clan.is_none() {
        return Response::error(ErrorCode::NoSuchClan);
    }

    let info = ClanInfo::from(clan.unwrap());
    Response::success(Content::Item(info))
}

/// Get a list of clans.
#[post("/clan_manager_view/sec/get_clan_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_clan_list(
    database: Data<Database>,
    req: Request<GetClanList>,
) -> Response<ClanPlayerInfo> {
    let jid = Jid::from(req.request.ticket.clone());

    // EXTRA: log the player's Jid in the `Players` collection, for future lookups
    let jid_ext = ExtendedJid::from(jid.clone());
    let player = match database
        .players
        .find_one(doc! {
            "username": jid_ext.username.clone(),
            "domain": jid_ext.domain.clone(),
            "region": jid_ext.region.clone(),
        })
        .await
    {
        Ok(player) => player,
        Err(e) => {
            log::error!("Failed to look-up player `{jid}` in the database: {e}");
            return Response::error(ErrorCode::InternalServerError);
        }
    };

    // Store the player's Jid in the database, if it doesn't exist
    if player.is_none() {
        match database.players.insert_one(jid_ext).await {
            Ok(_) => log::info!("Inserted player `{jid}` into the database"),
            Err(e) => log::error!("Failed to log player `{jid}` into the database: {e}"),
        }
    }

    // Find all the clans where the user is relevant
    let Ok(mut clans) = database
        .clans
        .find(doc! {
            "members.jid": jid.to_string(),
        })
        .await
    else {
        return Response::error(ErrorCode::InternalServerError);
    };

    // Collect all valid entries
    let mut data: Vec<Clan> = vec![];
    while let Some(clan) = clans.next().await {
        match clan {
            Ok(clan) => data.push(clan),
            Err(e) => log::error!("Error while fetching clan: {e}"),
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
pub async fn clan_search(
    database: Data<Database>,
    req: Request<ClanSearch>,
) -> Response<ClanSearchInfo> {
    let mut filter_doc = doc! {};

    if let Some(filter) = &req.request.filter {
        let mut value = filter.name.value.trim().to_lowercase();
        let mut platform = None;

        // Clan names are actually returned from the API with a suffix
        // indicating their platform of creation. This is so players
        // in-game can know which platform a clan is from.
        //
        // We want clans of all platforms to be visible, but not cross-joinable,
        // so we do support searching but will block joining in the other endpoints.
        if value.ends_with("[ps3]") {
            value = value.strip_suffix("[ps3]").unwrap().trim().to_string();
            platform = Some("Console");
        } else if value.ends_with("[pc]") {
            value = value.strip_suffix("[pc]").unwrap().trim().to_string();
            platform = Some("Emulator");
        }

        filter_doc = filter.name.operator.to_filter(&value);

        if let Some(p) = platform {
            filter_doc.insert("platform", p);
        }
    }

    let Ok(total) = database.clans.count_documents(filter_doc.clone()).await else {
        return Response::error(ErrorCode::InternalServerError);
    };

    let skip = (req.request.start - 1).max(0) as u64;
    let limit = i64::from(req.request.max.max(1));

    let Ok(mut cursor) = database
        .clans
        .find(filter_doc)
        .skip(skip)
        .limit(limit)
        .await
    else {
        return Response::error(ErrorCode::InternalServerError);
    };

    let mut data: Vec<Clan> = vec![];
    while let Some(clan) = cursor.next().await {
        if let Ok(clan) = clan {
            data.push(clan);
        }
    }

    // Format them from the perspective of the player (Clan -> ClanSearchInfo)
    let items: Vec<ClanSearchInfo> = data.into_iter().map(ClanSearchInfo::from).collect();

    let list = List {
        results: items.len() as u32,
        total: total as u32,

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
    if clan.name.len() > MAX_CLAN_NAME_LENGTH || clan.tag.len() > MAX_CLAN_TAG_LENGTH {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check the clans the author is in
    let Ok(clans) = author.clans(database.clone()).await else {
        return Response::error(ErrorCode::InternalServerError);
    };

    let clans_owned_len = clans
        .iter()
        .filter(|c| c.owner().is_some_and(|o| o.jid == author))
        .count();
    let clans_member_len = clans
        .iter()
        .filter(|c| c.status_of(&author) == Some(&Status::Member))
        .count();

    // Check if the author is already in too many clans
    if clans_member_len >= MAX_CLAN_MEMBERSHIP {
        return Response::error(ErrorCode::ClanJoinedLimitReached);
    }

    // Check if the author already owns too many clans
    if clans_owned_len >= MAX_CLAN_OWNERSHIP {
        return Response::error(ErrorCode::ClanLeaderLimitReached);
    }

    // Save the clan to the database.
    if let Err(e) = clan.save(&database).await {
        return Response::error(e);
    }

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
    let Some(owner) = clan.owner() else {
        return Response::error(ErrorCode::InternalServerError);
    };

    if owner.jid != jid {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Disband the clan
    if let Err(e) = clan.delete(&database).await {
        return Response::error(e);
    }

    Response::success(Content::Empty)
}

/// Update a member's information in a clan.
///
/// - The author needs to:
///     - Be a member of the clan
#[post("/clan_manager_update/sec/update_clan_info")]
pub async fn update_clan_info(
    database: Data<Database>,
    req: Request<UpdateClanInfo>,
) -> Response<()> {
    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to update the clan's info
    if !(clan.status_of(&Jid::from(req.request.ticket)) == Some(&Status::Member)) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Update the clan's info, making sure to limit the description's length
    clan.description = req
        .request
        .description
        .chars()
        .take(MAX_CLAN_DESCRIPTION_LENGTH)
        .collect();

    // Save the updated clan to the database
    if let Err(e) = clan.save(&database).await {
        return Response::error(e);
    }

    Response::success(Content::Empty)
}
