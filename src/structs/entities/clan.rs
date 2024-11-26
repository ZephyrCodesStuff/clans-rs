//! Clan entity module.

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::utils::xml_format::ToXml;

/// Maximum number of clans that can exist in the game.
const MAX_CLAN_COUNT: u32 = 1_000_000;

/// A clan ID.
/// 
/// Should be limited to [`MAX_CLAN_COUNT`], as the game
/// rejects any ID that surpasses it.
pub type Id = u32;

impl ToXml for Id {
    fn to_xml(&self, name: Option<&str>) -> String {
        format!("<{}>{}</{}>", name.unwrap_or("id"), self, name.unwrap_or("id"))
    }
}

/// Represents the basic information for a clan,
/// needed to display it in a search result.
/// 
/// Essentially a slimmer version of the [`Clan`](struct.Clan.html) struct.
#[derive(Debug, Serialize)]
pub struct BasicInfo {
    /// See: [`Clan::id`](struct.Clan.html#structfield.id)
    pub id: Id,

    /// See: [`Clan::name`](struct.Clan.html#structfield.name)
    pub name: String,

    /// See: [`Clan::tag`](struct.Clan.html#structfield.tag)
    pub tag: String,

    /// See: [`Clan::members`](struct.Clan.html#structfield.members)
    pub members: u32
}

impl ToXml for BasicInfo {
    fn to_xml(&self, name: Option<&str>) -> String {
        format!(
            r#"<{} id="{}"><name>{}</name><tag>{}</tag><members>{}</members></info>"#,
            name.unwrap_or("clan"), self.id, self.name, self.tag, self.members
        )
    }
}

/// Represents a clan in the game.
#[derive(Debug, Serialize)]
pub struct Clan {
    /// Unique identifier of the clan.
    /// 
    /// See: [`Id`](type.Id.html)
    pub id: Id,
    
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

    /// Number of members in the clan.
    pub members: u32,

    /// Creation date of the clan, in UTC.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date_created: DateTime<Utc>,
    
    /// If this flag is `true`, then the clan should
    /// automatically accept any player that requests to join.
    pub auto_accept: bool,

    /// Unknown use.
    pub int_attr1: u32,

    /// Unknown use.
    pub int_attr2: u32,

    /// Unknown use.
    pub int_attr3: u32,
}

impl ToXml for Clan {
    fn to_xml(&self, name: Option<&str>) -> String {
        format!(
            r#"<{} id="{}"><name>{}</name><tag>{}</tag><description>{}</description><members>{}</members><date_created>{}</date_created><auto_accept>{}</auto_accept><int_attr1>{}</int_attr1><int_attr2>{}</int_attr2><int_attr3>{}</int_attr3></{}>"#,
            name.unwrap_or("clan"), self.id, self.name, self.tag, self.description, self.members, self.date_created.format("%a, %d %b %Y %H:%M:%S GMT"), self.auto_accept, self.int_attr1, self.int_attr2, self.int_attr3, name.unwrap_or("clan")
        )
    }
}