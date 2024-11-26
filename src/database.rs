//! Database utility struct
//! 
//! Wrapper around the ``MongoDB`` database connection
//! and collections.

use crate::structs::entities::clan::Clan;

/// Database utility struct.
#[derive(Debug, Clone)]
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

        Self {
            database,
            clans,
        }
    }
}