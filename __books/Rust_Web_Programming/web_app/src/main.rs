mod views;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( ||
        App::new()
            .configure(views::views_factory)
    )
    .bind(("0.0.0.0", 8080)).map_err(|err| {
        println!("Error binding: {}", err);
        err
    })?
    .run()
    .await
}
