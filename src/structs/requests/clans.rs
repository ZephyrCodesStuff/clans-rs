//! Request structs for clan related requests.

use serde::{Deserialize, Deserializer};

use crate::structs::{
    entities::{
        clan::{Clan, Id, Platform},
        player::{Jid, Player, Role, Status},
    },
    ticket::Ticket,
};

/// Request to create a clan.
#[derive(Debug, Deserialize)]
pub struct CreateClan {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The name of the clan.
    pub name: String,

    /// The clan's tag.
    pub tag: String,
}

impl From<CreateClan> for Clan {
    fn from(request: CreateClan) -> Self {
        let mut clan = Self::default();

        clan.name = request.name;
        clan.tag = request.tag;
        clan.platform = Platform::from(request.ticket.clone());

        clan.members = vec![Player {
            jid: Jid::from(request.ticket),
            role: Role::Leader,
            status: Status::Member,
            ..Default::default()
        }];

        clan
    }
}

/// Request to get a list of clans.
#[derive(Debug, Deserialize)]
pub struct GetClanList {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// How many clans to skip.
    pub start: i32,

    /// How many clans to return.
    pub max: i32,
}

/// Request to get a list of clans.
#[derive(Debug, Deserialize)]
pub struct ClanSearch {
    /// How many clans to skip.
    pub start: i32,

    /// How many clans to return.
    pub max: i32,

    /// A custom filter to apply to the search.
    pub filter: Option<ClanSearchFilter>,
}

/// Enum of operators to apply to a clan search filter.
#[derive(Debug)]
pub enum ClanSearchFilterOperator {
    /// Everything.
    All,

    /// Name equal to.
    Equal,

    /// Name different from.
    NotEqual,

    /// Name starts with.
    GreaterThan,

    /// Name starts with, or equal to.
    GreaterThanOrEqual,

    /// Name ends with.
    LessThan,

    /// Name ends with, or equal to.
    LessThanOrEqual,

    /// Name contains.
    Like,
}

impl Default for ClanSearchFilterOperator {
    fn default() -> Self {
        Self::All
    }
}

impl TryFrom<&str> for ClanSearchFilterOperator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "eq" => Ok(Self::Equal),
            "ne" => Ok(Self::NotEqual),
            "gt" => Ok(Self::GreaterThan),
            "ge" => Ok(Self::GreaterThanOrEqual),
            "lt" => Ok(Self::LessThan),
            "le" => Ok(Self::LessThanOrEqual),
            "lk" => Ok(Self::Like),
            _ => Err(()),
        }
    }
}

/// A custom filter to apply to a clan search.
#[derive(Debug, Default, Deserialize)]
pub struct ClanSearchFilter {
    /// The name of the filter.
    pub name: ClanSearchFilterName,
}

/// The inner filter's properties.
#[derive(Debug, Default, Deserialize)]
pub struct ClanSearchFilterName {
    /// The operator to apply.
    #[serde(rename = "op")]
    pub operator: ClanSearchFilterOperator,

    /// The value of the operator.
    #[serde(rename = "value")]
    pub value: String,
}

impl ClanSearchFilterOperator {
    /// Convert the operator to a BSON filter.
    pub fn to_filter(&self, value: &str) -> mongodb::bson::Document {
        use mongodb::bson::doc;

        // Escape regex characters to prevent injection
        let escaped_value = regex::escape(value);

        match self {
            // Case-insensitive exact match
            Self::Equal => doc! {
                "$regex": format!("^{escaped_value}$"),
                "$options": "i"
            },
            Self::NotEqual => doc! {
                "$not": {
                    "$regex": format!("^{escaped_value}$"),
                    "$options": "i"
                }
            },
            // Current "GreaterThan" behavior: Starts With (Case Insensitive)
            Self::GreaterThan | Self::GreaterThanOrEqual => doc! {
                "$regex": format!("^{escaped_value}"),
                "$options": "i"
            },
            // Current "LessThan" behavior: Ends With (Case Insensitive)
            Self::LessThan | Self::LessThanOrEqual => doc! {
                "$regex": format!("{escaped_value}$"),
                "$options": "i"
            },
            // Contains (Case Insensitive)
            Self::Like => doc! {
                "$regex": escaped_value,
                "$options": "i"
            },
            // All matches everything
            Self::All => doc! {},
        }
    }
}

impl<'de> Deserialize<'de> for ClanSearchFilterOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Get as string to parse the filter.
        let op = String::deserialize(deserializer)?;

        // Try to parse the operator.
        let operator = Self::try_from(op.as_str())
            .map_err(|()| serde::de::Error::custom("Invalid operator"))?;

        Ok(operator)
    }
}

/// Request to get info about a clan.
#[derive(Debug, Deserialize)]
pub struct GetClanInfo {
    /// The ID of the clan.
    pub id: Id,
}

/// Request to update a clan's info.
#[derive(Debug, Deserialize)]
pub struct UpdateClanInfo {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The new description of the clan.
    pub description: String,
}

/// Request to disband a clan.
#[derive(Debug, Deserialize)]
pub struct DisbandClan {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,
}
