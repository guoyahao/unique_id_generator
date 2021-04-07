use  crate::service::init::WORKPOLL;

#[actix_web::get("/")]
pub async fn get_next_id() -> impl actix_web::Responder {
    let mut work_object = WORKPOLL.lock().unwrap();
    let mut object = work_object.get_mut(1).unwrap();
    let mut body = String::new();

    body = format!("work_id:{},next_id:{}",object.worker_id.to_string(),object.next_id().to_string());

    //body = "work_id:{}" + object.worker_id.to_string() + " next_id:{}"+object.worker_id;
    //获取当前workid的配置对象
    actix_web::HttpResponse::Ok().body(body)
}


