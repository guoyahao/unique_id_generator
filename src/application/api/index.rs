use  crate::service::init::WORKPOLL;

#[actix_web::get("/")]
pub async fn get_next_id() -> impl actix_web::Responder {
    let mut work_object = WORKPOLL.lock().unwrap();
    let object = work_object.get_mut(1).unwrap();
    let body = format!("work_id:{},next_id:{}",object.worker_id.to_string(),object.next_id().to_string());
    //获取当前worked的配置对象
    actix_web::HttpResponse::Ok().body(body)
}

#[actix_web::get("/id")]
pub async fn get_id() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("hello word")
}