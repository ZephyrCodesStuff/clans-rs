//! Routes handling special functions that are NOT meant
//! to be accessed by the game client.
//! 
//! These endpoints are authenticated by the server, and
//! are meant specifically for use by the Destination Home
//! revival project's Discord bot.

use actix_web::{put, web::{Data, Json}};
use futures_util::StreamExt;
use mongodb::bson::doc;

use crate::{database::Database, structs::{entities::{clan::{Clan, Platform, MAX_CLAN_OWNERSHIP}, player::Role}, requests::admin::CreateClan, responses::{admin::Response, error::{ErrorCode, SUCCESS}}}};

/// Create a clan.
#[put("/admin/clan/create")]
pub async fn create_clan(database: Data<Database>, data: Json<CreateClan>) -> Response {
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

    let Ok(author) = database.players.find_one(filter).await
    else { return Response::from(ErrorCode::InternalServerError) };

    // If the player was not found, return an error
    if author.is_none() {
        return Response::from(ErrorCode::InvalidNpId);
    }
    
    let author = author.unwrap();
    let clan = Clan::from((data.into_inner(), author.clone()));

    // Find all the clans where the author is a leader
    let Ok(mut clans) = database.clans.find(doc! {}).await
    else { return Response::from(ErrorCode::InternalServerError) };

    let mut clans_vec: Vec<Clan> = vec![];
    while let Some(Ok(clan)) = clans.next().await {
        clans_vec.push(clan);
    }

    // Since we don't have the full JID, we have to search manually.
    let owned = clans_vec.iter().filter(|clan| {
        clan.members.iter().any(|member| {
            member.jid == author.clone().into() && member.role == Role::Leader
        })
    });

    // Check if the author already owns too many clans
    if owned.count() >= MAX_CLAN_OWNERSHIP {
        return Response::from(ErrorCode::ClanLeaderLimitReached);
    }

    // Save the clan to the database.
    if let Err(e) = clan.save(&database).await { return Response::from(e); }

    Response::from(SUCCESS)
}