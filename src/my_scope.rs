use actix_web::{get, web, HttpResponse, Responder, dev::HttpServiceFactory};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}

#[get("/user")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Getting users")
}

async fn manual() -> impl Responder {
    HttpResponse::Ok().body("Manual route!")
}

pub fn scope<T: HttpServiceFactory>() {
    web::scope("/app")
        .service(index)
        .route("/manual", web::get().to(manual));
}
