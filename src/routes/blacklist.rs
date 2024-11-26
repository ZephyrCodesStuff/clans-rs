//! Routes pertaining to a clan's blacklist, such as:
//!
//! - Getting a clan's blacklist
//! - Adding a player to a clan's blacklist
//! - Removing a player from a clan's blacklist
//! - ...

use actix_web::{post, web::Data};
use mongodb::bson::doc;

use crate::{database::Database, structs::{
    entities::player::Jid, requests::{base::Request, blacklist::{DeleteBlacklistEntry, GetBlacklist, RecordBlacklistEntry}}, responses::{
        base::{Content, List, Response},
        entities::BlacklistEntry, error::ErrorCode,
    }
}};

/// Get a clan's blacklist.
#[post("/clan_manager_view/sec/get_blacklist")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_blacklist(database: Data<Database>, req: Request<GetBlacklist>) -> Response<BlacklistEntry> {
    // Find the clan
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await
    else { return Response::error(ErrorCode::InternalServerError) };

    if clan.is_none() {
        return Response::error(ErrorCode::NoSuchClan);
    }

    let clan = clan.unwrap();

    // Collect all valid entries
    let items = clan.blacklist
        .iter()
        .skip((req.request.start - 1) as usize)
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
#[post("/clan_manager_update/sec/record_blacklist_entry")]
pub async fn record_blacklist_entry(database: Data<Database>, req: Request<RecordBlacklistEntry>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    // Find the clan
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await
    else { return Response::error(ErrorCode::InternalServerError) };

    if clan.is_none() {
        return Response::error(ErrorCode::NoSuchClan);
    }

    let mut clan = clan.unwrap();

    // Check if the user is allowed to add to the blacklist
    if !clan.is_mod(&jid) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the player is a member of the clan
    if !clan.is_member(&Jid::from(req.request.jid.clone())) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Add the player to the blacklist
    clan.blacklist.push(Jid::from(req.request.jid));

    // Update the clan
    if database.clans.replace_one(doc! { "id": clan.id() }, clan).await.is_err() {
        return Response::error(ErrorCode::InternalServerError);
    }

    Response::success(Content::Empty)
}

/// Remove a player from a clan's blacklist.
#[post("/clan_manager_update/sec/delete_blacklist_entry")]
pub async fn delete_blacklist_entry(database: Data<Database>, req: Request<DeleteBlacklistEntry>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    // Find the clan
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await
    else { return Response::error(ErrorCode::InternalServerError) };

    if clan.is_none() {
        return Response::error(ErrorCode::NoSuchClan);
    }

    let mut clan = clan.unwrap();

    // Check if the user is allowed to remove from the blacklist
    if !clan.is_mod(&jid) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the user is actually blacklisted
    if !clan.is_blacklisted(&Jid::from(req.request.jid.clone())) {
        return Response::error(ErrorCode::NoSuchBlacklistEntry);
    }

    // Remove the player from the blacklist
    clan.blacklist.retain(|j| j != &Jid::from(req.request.jid.clone()));

    // Update the clan
    if database.clans.replace_one(doc! { "id": clan.id() }, clan).await.is_err() {
        return Response::error(ErrorCode::InternalServerError);
    }

    Response::success(Content::Empty)
}
