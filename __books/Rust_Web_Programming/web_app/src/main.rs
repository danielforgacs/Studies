mod to_do;
mod state;
mod views;
mod processes;
mod json_serialization;
mod jwt;

use to_do::enums::TaskStatus;
use actix_web::{App, HttpServer};
use actix_service::Service;
use actix_cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
        .configure(views::views_factory)
        .wrap(cors)
        // .wrap_fn(|req, srv| {
        //     println!("request: {:?}", req);
        //     let future = srv.call(req);
        //     async {
        //         let result = future.await?;
        //         Ok(result)
        //     }
        // })
    })
    .bind(("0.0.0.0", 8080)).map_err(|err| {
        println!("Error binding: {}", err);
        err
    })?
    .run()
    .await
}
