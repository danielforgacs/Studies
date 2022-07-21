use actix_web::{App, HttpServer, Responder, HttpResponse};
use actix_web::web;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://localhost:8080");
    HttpServer::new(|| {
        App::new()
        .route("/hello", web::get().to(hello))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
