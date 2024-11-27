//! Routes pertaining to a clan's blacklist, such as:
//!
//! - Getting a clan's blacklist
//! - Adding a player to a clan's blacklist
//! - Removing a player from a clan's blacklist
//! - ...

use actix_web::{post, web::Data};
use mongodb::bson::doc;

use crate::{database::Database, structs::{
    entities::{clan::Clan, player::{Jid, Role, Status}}, requests::{base::Request, blacklist::{DeleteBlacklistEntry, GetBlacklist, RecordBlacklistEntry}}, responses::{
        base::{Content, List, Response},
        entities::BlacklistEntry, error::ErrorCode,
    }
}};

/// Get a clan's blacklist.
#[post("/clan_manager_view/sec/get_blacklist")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_blacklist(database: Data<Database>, req: Request<GetBlacklist>) -> Response<BlacklistEntry> {
    let clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Collect all valid entries
    let items = clan.blacklist
        .iter()
        .skip((req.request.start - 1).max(0) as usize)
        .take(req.request.max as usize)
        .map(|m| BlacklistEntry::from(m.to_owned()))
        .collect::<Vec<BlacklistEntry>>();

    let list = List {
        results: items.len() as u32,
        total: clan.blacklist.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Add a player to a clan's blacklist.
/// 
/// - The author needs to:
///     - Be a SubLeader or higher
/// 
/// - The player needs to:
///     - Not be a member of the clan
#[post("/clan_manager_update/sec/record_blacklist_entry")]
pub async fn record_blacklist_entry(database: Data<Database>, req: Request<RecordBlacklistEntry>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);
    let Ok(target) = Jid::try_from(req.request.jid.clone())
    else { return Response::error(ErrorCode::InvalidNpId) };

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to add to the blacklist
    if !clan.role_of(&jid).map_or(false, |role| role >= &Role::SubLeader) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the player is a member of the clan
    if clan.status_of(&jid).map_or(false, |status| status == &Status::Member) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Add the player to the blacklist
    clan.blacklist.push(target);

    // Update the clan
    if database.clans.replace_one(doc! { "id": clan.id() }, clan).await.is_err() {
        return Response::error(ErrorCode::InternalServerError);
    }

    Response::success(Content::Empty)
}

/// Remove a player from a clan's blacklist.
/// 
/// - The author needs to:
///     - Be a SubLeader or higher
/// 
/// - The player needs to:
///     - Not be a member of the clan
///     - Be blacklisted
#[post("/clan_manager_update/sec/delete_blacklist_entry")]
pub async fn delete_blacklist_entry(database: Data<Database>, req: Request<DeleteBlacklistEntry>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);
    let Ok(target) = Jid::try_from(req.request.jid.clone())
    else { return Response::error(ErrorCode::InvalidNpId) };

    // Find the clan
    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to remove from the blacklist
    if !clan.role_of(&jid).map_or(false, |role| role >= &Role::SubLeader) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the player is a member of the clan
    if clan.status_of(&target).map_or(false, |status| status == &Status::Member) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Check if the player is blacklisted
    if !clan.is_blacklisted(&target) {
        return Response::error(ErrorCode::NoSuchBlacklistEntry);
    }

    // Remove the player from the blacklist
    clan.blacklist.retain(|j| j != &target);

    // Update the clan
    if database.clans.replace_one(doc! { "id": clan.id() }, clan).await.is_err() {
        return Response::error(ErrorCode::InternalServerError);
    }

    Response::success(Content::Empty)
}
