use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod my_scope;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        let scope = my_scope::scope;

        App::new().app_data(counter.clone()).service(scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
