mod application;
mod service;

use actix_web::{App, HttpServer};
use crate::application::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    service::init::run();
    HttpServer::new(|| {
        App::new()
            .service(api::index::get_next_id)
            .service(api::index::get_id)
    })
        .bind("127.0.0.1:8083")?
        .run()
        .await
}