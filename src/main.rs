mod application;
mod service;

use actix_web::{App, HttpServer};
use crate::application::api;
use crate::service::init::WORKPOLL;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    WORKPOLL.lock().unwrap();
    HttpServer::new(|| {
        App::new()
            .service(api::index::get_next_id)
    })
        .bind("127.0.0.1:8083")?
        .run()
        .await
}