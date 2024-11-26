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


// #[post("/{clan_manager_update_view}/{sec_func}/{func}")]
// async fn sec_func(body: Bytes, func: Path<((), (), String)>) -> impl Responder {
//     let body = String::from_utf8_lossy(&body);
//     log::debug!("body: {}", body);

//     let response = fs::read_to_string(format!("responses/{}.xml", func.2))
//         .unwrap_or_else(|_| panic!("Could not read file responses/{}.xml", func.2));

//     let date = chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
//     HttpResponse::Ok()
//         .append_header(("Message-Type", "x-ps3-clan"))
//         .append_header(("Version", "1.00"))
//         .append_header(("Ignore-Level", "normal"))
//         .append_header(("Content-Type", "application/x-ps3-clan"))
//         .append_header(("Date", date))
//         .append_header(("Server", "Apache"))
//         .keep_alive()
//         .body(response)
// }

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

            // Blacklist
            .service(routes::blacklist::get_blacklist)
            .service(routes::blacklist::record_blacklist_entry)
            .service(routes::blacklist::delete_blacklist_entry)

            // Members
            .service(routes::members::get_member_list)

            .wrap(Logger::default())
            .app_data(Data::new(database.clone()))
    )
        .bind((host, port))?
        .run()
        .await
}