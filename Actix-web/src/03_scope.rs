use actix_web::{HttpServer, App, Responder, web};

async fn hello_world() -> impl Responder {
    "Hello world!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/this-is-the-scope")
                    .route("", web::get().to(hello_world))
            )
    })
        .bind("localhost:8090")?
        .run()
        .await
}
