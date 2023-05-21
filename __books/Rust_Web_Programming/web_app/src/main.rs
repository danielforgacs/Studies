mod to_do;
mod state;
mod views;
mod processes;
mod json_serialization;
mod jwt;

use to_do::enums::TaskStatus;
use actix_web::{App, HttpServer};
use actix_service::Service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( ||
        App::new()
            // .wrap_fn(|req, srv| {
            //     println!("request: {:?}", req);
            //     let future = srv.call(req);
            //     async {
            //         let result = future.await?;
            //         Ok(result)
            //     }
            // })
            .configure(views::views_factory)
    )
    .bind(("0.0.0.0", 8080)).map_err(|err| {
        println!("Error binding: {}", err);
        err
    })?
    .run()
    .await
}
