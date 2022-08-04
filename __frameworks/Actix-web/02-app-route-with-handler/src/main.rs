use actix_web::{App, HttpServer, Responder};
use actix_web::web;

async fn hello() -> impl Responder {
    "Hello Joker!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        move || {
            App::new()
                .route("/", web::get().to(hello))
        }
    )
        .bind("127.0.0.1:8888")?
        .run()
        .await
}

