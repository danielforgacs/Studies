use actix_web::{HttpServer, App, Responder, web};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    // Mutex is necessary to mutate safely across threads
    counter: Mutex<i32>,
}

async fn hello_world(data: web::Data<AppState>, counter: web::Data<AppStateWithCounter>) -> impl Responder {
    // Get counter's MutexGuard
    let mut counter = counter.counter.lock().unwrap();
    // Access counter inside MutexGuard
    *counter += 1;
    format!("Hello app: {}! visit count: {}", &data.app_name, counter)
}

async fn app_name(data: web::Data<AppState>, counter: web::Data<AppStateWithCounter>) -> impl Responder {
    // Get counter's MutexGuard
    let mut counter = counter.counter.lock().unwrap();
    // Access counter inside MutexGuard
    *counter += 1;

    format!("App name: {}. visit count: {}", &data.app_name, counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { app_name: String::from("Actix Web") }))
            // to be able to use this, the closure mused be moved!
            .app_data(counter.clone())
            .route("/", web::get().to(hello_world))
            .route("/appname", web::get().to(app_name))
    })
        .bind("localhost:8090")?
        .run()
        .await
}
