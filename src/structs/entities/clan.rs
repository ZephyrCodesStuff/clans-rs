//! These structs represent the data of a clan in the game,
//! as completely as possible.
//! 
//! They are what's stored into the database.

use actix_web::{web::{Buf, Data}, FromRequest};
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{database::Database, structs::{requests::base::Request, responses::error::ErrorCode}};

use super::player::{Jid, Player, Role, Status};

/// Maximum number of clans that can exist in the game.
const MAX_CLAN_COUNT: u32 = 1_000_000;

/// A clan ID.
/// 
/// Should be limited to [`MAX_CLAN_COUNT`], as the game
/// rejects any ID that surpasses it.
pub type Id = u32;

/// Represents a clan in the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clan {
    /// Unique identifier of the clan. Should not be changed manually,
    /// because values outside the range of [`MAX_CLAN_COUNT`] **will**
    /// halt the game.
    /// 
    /// See: [`Id`](type.Id.html)
    id: Id,
    
    /// Displayed name of the clan.
    /// 
    /// This appears below the player's name.
    pub name: String,

    /// Tag used to identify the clan.
    /// 
    /// NOTE: This is **not** what appears below the player's name.
    pub tag: String,

    /// Description of the clan.
    /// 
    /// This appears when the player clicks on the clan,
    /// and selects to view the clan's information.
    pub description: String,

    /// Members currenty in the clan.
    /// 
    /// This will always include, at least, the clan's leader.
    pub members: Vec<Player>,

    /// Players that are banned from joining the clan.
    pub blacklist: Vec<Jid>,

    /// Creation date of the clan, in UTC.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date_created: DateTime<Utc>,

    /// Last time the clan was updated, in UTC.
    /// 
    /// NOTE: This isn't needed by the game, but it's useful for statistics.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_updated: DateTime<Utc>,
    
    /// If this flag is `true`, then the clan should
    /// automatically accept any player that requests to join.
    pub auto_accept: bool,

    /// Unknown use.
    pub int_attr1: u32,

    /// Unknown use.
    pub int_attr2: u32,

    /// Unknown use.
    pub int_attr3: u32,

    /// Unknown use.
    pub size: u32,
}

impl Default for Clan {
    fn default() -> Self {
        let range = 1..MAX_CLAN_COUNT;
        let rng = &mut rand::thread_rng();

        Self {
            id: rng.gen_range(range),
            name: String::new(),
            tag: String::new(),
            description: String::new(),
            members: Vec::new(),
            blacklist: Vec::new(),
            date_created: Utc::now(),
            last_updated: Utc::now(),
            auto_accept: false,
            int_attr1: 0,
            int_attr2: 0,
            int_attr3: 0,
            size: 0,
        }
    }
}

/// Extractor helper, to parse the XML body of the request as such:
/// ```xml
/// <clan>
///     <id>{id}</id>
/// </clan>
/// ```
#[derive(Debug, Deserialize)]
pub struct IdOnly {
    /// The ID of the clan.
    pub id: Id,
}

/// Implement an extractor for the clan, from the request
impl FromRequest for Clan {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + 'static>>;
    
    /// Get the clan ID from the request body, then fetch the clan from the database.
    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let fut = actix_web::web::Bytes::from_request(req, payload);
        let database = req.app_data::<Data<Database>>().unwrap().clone();

        Box::pin(async move {
            let bytes = fut.await?;

            let request: Request<IdOnly> = serde_xml_rs::from_reader(bytes.reader())
                .map_err(actix_web::error::ErrorInternalServerError)?;

            let clan_id = request.request.id;

            // Fetch the clan from the database
            let Ok(Some(clan)) = database.clans.find_one(doc! { "id": clan_id }).await
            else { return Err(actix_web::error::ErrorInternalServerError("Failed to fetch the clan while trying to extract it from the request.")) };

            Ok(clan)
        })
    }
}

impl Clan {
    /// Save the clan in the database.
    /// 
    /// This will replace the clan's document altogether and,
    /// if the clan doesn't exist, it will create a new one.
    pub async fn save(&self, database: &Data<Database>) -> Result<(), ErrorCode> {
        database.clans.replace_one(doc! { "id": self.id }, self.clone())
            .upsert(true) // Create the document if it doesn't exist
            .await
            .map_err(|_| ErrorCode::InternalServerError)
            .map(|_| ())
    }

    /// Delete the clan from the database.
    pub async fn delete(&self, database: &Data<Database>) -> Result<(), ErrorCode> {
        database.clans.delete_one(doc! { "id": self.id })
            .await
            .map_err(|_| ErrorCode::InternalServerError)
            .map(|_| ())
    }

    /// Returns the clan's ID.
    pub const fn id(&self) -> Id {
        self.id
    }

    /// Returns the role of the given player, in the clan.
    pub fn role_of(&self, jid: &Jid) -> Option<&Role> {
        self.members.iter()
            .find(|player| player.jid == *jid)
            .map(|player| &player.role)
    }

    /// Returns the status of the given player, in the clan.
    pub fn status_of(&self, jid: &Jid) -> Option<&Status> {
        self.members.iter()
            .find(|player| player.jid == *jid)
            .map(|player| &player.status)
    }

    /// Returns whether a player is a member of the clan.
    pub fn is_member(&self, jid: &Jid) -> bool {
        self.members.iter()
            .any(|player| player.jid == *jid && player.status == Status::Member)
    }

    /// Returns whether a player is allowed to perform administrative actions.
    pub fn is_mod(&self, jid: &Jid) -> bool {
        self.members.iter()
            .any(|player| player.jid == *jid && player.role >= Role::SubLeader)
    }

    /// Returns whether a player is blacklisted from the clan.
    pub fn is_blacklisted(&self, jid: &Jid) -> bool {
        self.blacklist.iter()
            .any(|blacklisted| blacklisted == jid)
    }

    /// Returns the owner of the clan.
    /// 
    /// Ideally, this should never be `None`.
    pub fn owner(&self) -> Option<&Player> {
        self.members.iter()
            .find(|player| player.role == Role::Leader)
    }
}