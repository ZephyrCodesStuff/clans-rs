//! clans-rs
//! 
//! Author: [zeph](https://github.com/ZephyrCodesStuff)
//! 
//! This crate implements an API that brings back to life the
//! Clan functionality of the ``PlayStation`` 3.
//! 
//! This API is intended to be used with the game ``PlayStation Home``.

mod database;
mod structs;
mod routes;
mod utils;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use database::Database;
use structs::responses::{base::Response, error::ErrorCode};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Default to ``info`` logging level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let host = std::env::var("HOST")
        .unwrap_or_else(|_| String::from("0.0.0.0"));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse::<u16>().expect("PORT must be a number");

    let database = Database::init().await;

    log::info!("Starting server at {}:{}", host, port);

    HttpServer::new(move || 
        App::new()

            // Clans
            .service(routes::clans::get_clan_info)
            .service(routes::clans::get_clan_list)
            .service(routes::clans::clan_search)
            .service(routes::clans::create_clan)
            .service(routes::clans::disband_clan)
            .service(routes::clans::update_clan_info)

            // Blacklist
            .service(routes::blacklist::get_blacklist)
            .service(routes::blacklist::record_blacklist_entry)
            .service(routes::blacklist::delete_blacklist_entry)

            // Members
            .service(routes::members::get_member_list)

            // Announcements
            .service(routes::announcements::retrieve_announcements)

            // Invites
            .service(routes::invites::send_invitation)
            .service(routes::invites::cancel_invitation)
            .service(routes::invites::request_membership)

            // Fallback handler
            .default_service(actix_web::web::to(|| async {
                Response::<()>::error(ErrorCode::NoSuchClanService)
            }))

            .wrap(Logger::default())
            .app_data(Data::new(database.clone()))
    )
        .bind((host, port))?
        .run()
        .await
}