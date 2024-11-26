//! Database utility struct
//! 
//! Wrapper around the ``MongoDB`` database connection
//! and collections.

use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::structs::entities::clan::Clan;

/// Database utility struct.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Database {
    /// ``MongoDB`` database connection.
    database: mongodb::Database,

    /// Collection of clans.
    pub clans: mongodb::Collection<Clan>,
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

        Self {
            database,
            clans,
        }
    }
}