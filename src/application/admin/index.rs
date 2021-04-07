#[actix_web::get("/")]
pub async fn hello() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello")
}