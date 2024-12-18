//! Database utility struct
//! 
//! Wrapper around the ``MongoDB`` database connection
//! and collections.

use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::structs::entities::{clan::Clan, player::Jid};

/// Database utility struct.
#[derive(Debug, Clone)]
#[allow(dead_code)]
#[allow(clippy::struct_field_names)]
pub struct Database {
    /// ``MongoDB`` database connection.
    database: mongodb::Database,

    /// Collection of clans.
    pub clans: mongodb::Collection<Clan>,

    /// Collections of players and their regions.
    /// 
    /// This is necessary because we need to be able to find them,
    /// when creating clans outside of the game. Else the game will
    /// ignore the clan we've made.
    pub players: mongodb::Collection<Jid>,
}

impl Database {
    /// Initialize the database connection.
    /// 
    /// ## Panic
    /// This function will panic if the ``MONGO_URI`` environment variable
    /// is not set, or if the connection to the database fails.
    pub async fn init() -> Self {
        let mongo_uri = std::env::var("MONGO_URI")
            .unwrap_or_else(|_| String::from("mongodb://localhost:27017"));

        let client = mongodb::Client::with_uri_str(&mongo_uri).await.unwrap();
        let database = client.default_database()
            .unwrap_or_else(|| client.database("clans"));

        let clans = database.collection("clans");
        
        // Make sure the clans collection has a unique index on ``id``.
        // This will prevent duplicate clans from being created.
        let index = IndexModel::builder()
            .keys(doc! { "id": 1 })
            .options(IndexOptions::builder()
                .unique(true)
                .build())
            .build();

        clans.create_index(index).await.unwrap();

        let players = database.collection("players");

        Self {
            database,
            clans,
            players,
        }
    }
}