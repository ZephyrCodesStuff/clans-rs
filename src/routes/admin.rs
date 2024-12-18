//! Routes handling special functions that are NOT meant
//! to be accessed by the game client.
//! 
//! These endpoints are authenticated by the server, and
//! are meant specifically for use by the Destination Home
//! revival project's Discord bot.

use actix_web::{put, web::{Data, Json}};
use futures_util::StreamExt;
use mongodb::bson::doc;

use crate::{database::Database, structs::{entities::{clan::{Clan, MAX_CLAN_OWNERSHIP}, player::{Jid, Role}}, requests::admin::CreateClan, responses::{admin::Response, error::{ErrorCode, SUCCESS}}}};

/// The region of the player who creates a clan with the Admin endpoint.
/// 
/// We set this to `un` because that's RPCS3's default, and also generic enough.
pub const FORGED_JID_DOMAIN: &str = "un";

/// The domain of the player who creates a clan with the Admin endpoint.
/// 
/// We set this to `br` because that's RPCS3's default.
pub const FORGED_JID_REGION: &str = "br";

/// Create a clan.
#[put("/admin/clan/create")]
pub async fn create_clan(database: Data<Database>, data: Json<CreateClan>) -> Response {
    let author = Jid::from(data.clone());
    let clan = Clan::from(data.into_inner());

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
            member.jid == author && member.role == Role::Leader
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