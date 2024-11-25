use std::fs;

use actix_web::{middleware::Logger, post, web::{Bytes, Path}, App, HttpResponse, HttpServer, Responder};


#[post("/{clan_manager_update_view}/{sec_func}/{func}")]
async fn sec_func(body: Bytes, func: Path<((), (), String)>) -> impl Responder {
    let body = String::from_utf8_lossy(&body);
    log::debug!("body: {}", body);

    let response = fs::read_to_string(format!("responses/{}.xml", func.2))
        .expect(format!("Could not read file responses/{}.xml", func.2).as_str());

    let date = chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    HttpResponse::Ok()
        .append_header(("Message-Type", "x-ps3-clan"))
        .append_header(("Version", "1.00"))
        .append_header(("Ignore-Level", "normal"))
        .append_header(("Content-Type", "application/x-ps3-clan"))
        .append_header(("Date", date))
        .append_header(("Server", "Apache"))
        .keep_alive()
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = std::env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port = std::env::var("PORT").unwrap_or(String::from("8080"))
        .parse::<u16>().expect("PORT must be a number");

    log::info!("Starting server at {}:{}", host, port);

    HttpServer::new(|| 
        App::new()
            .service(sec_func)
            .wrap(Logger::default())
    )
        .bind((host, port))?
        .run()
        .await
}