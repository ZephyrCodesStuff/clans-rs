//! Routes handling special functions that are NOT meant
//! to be accessed by the game client.
//! 
//! These endpoints are authenticated by the server, and
//! are meant specifically for use by the Destination Home
//! revival project's Discord bot.

use actix_web::{put, web::{Data, Json}};
use mongodb::bson::doc;

use crate::{database::Database, structs::{entities::{clan::{Clan, Platform, MAX_CLAN_MEMBERSHIP, MAX_CLAN_NAME_LENGTH, MAX_CLAN_OWNERSHIP, MAX_CLAN_TAG_LENGTH}, player::{Jid, Status}}, requests::admin::CreateClan, responses::{admin::Response, error::{ErrorCode, SUCCESS}}}};

/// Create a clan.
#[put("/admin/clan/create")]
pub async fn create_clan(database: Data<Database>, mut data: Json<CreateClan>) -> Response {
    // Look-up the player in the database
    let filter = match data.clan_platform {
        Platform::Console => doc! {
            "username": data.username.clone(),
            "domain": {"$ne": "un"},
            "region": {"$ne": "br"},
        },
        Platform::Emulator => doc! {
            "username": data.username.clone(),
            "domain": "un",
            "region": "br",
        }
    };

    // Limit the clan name and tag to their maximum lengths
    data.clan_name = data.clan_name.chars().take(MAX_CLAN_NAME_LENGTH).collect();
    data.clan_tag = data.clan_tag.chars().take(MAX_CLAN_TAG_LENGTH).collect();

    // Make sure Unicode chars don't exceed the limits
    if data.clan_name.len() > MAX_CLAN_NAME_LENGTH || data.clan_tag.len() > MAX_CLAN_TAG_LENGTH {
        return Response::from(ErrorCode::PermissionDenied);
    }

    let Ok(author) = database.players.find_one(filter).await
    else { return Response::from(ErrorCode::InternalServerError) };

    // If the player was not found, return an error
    if author.is_none() {
        return Response::from(ErrorCode::InvalidNpId);
    }
    
    let author: Jid = author.unwrap().into();
    let clan = Clan::from((data.into_inner(), author.clone()));

    // Check the clans the author is in
    let Ok(clans) = author.clans(database.clone()).await
    else { return Response::from(ErrorCode::InternalServerError) };

    let clans_owned_len = clans.iter().filter(|c| c.owner().is_some_and(|o| o.jid == author)).count();
    let clans_member_len = clans.iter().filter(|c| c.status_of(&author) == Some(&Status::Member)).count();

    // Check if the author is already in too many clans
    if clans_member_len >= MAX_CLAN_MEMBERSHIP {
        return Response::from(ErrorCode::ClanJoinedLimitReached);
    }

    // Check if the author already owns too many clans
    if clans_owned_len >= MAX_CLAN_OWNERSHIP {
        return Response::from(ErrorCode::ClanLeaderLimitReached);
    }

    // Save the clan to the database.
    if let Err(e) = clan.save(&database).await { return Response::from(e); }

    Response::from(SUCCESS)
}