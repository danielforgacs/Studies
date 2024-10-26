use actix_web::{
        web,
        HttpServer,
        App,
        HttpResponse,
        middleware::Logger,
    };
use env_logger;
use dotenv;
use log;

const URL: &str = "127.0.0.1:8888";

async fn alive() -> HttpResponse {
    log::info!("Checking if alive.");
    HttpResponse::Ok().body("Ok")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    println!("http://{}", URL);
    env_logger::init();
    HttpServer::new(
        move || {
            App::new()
                .wrap(Logger::new("%a %{User-Agent}i"))
                .route("/alive", web::get().to(alive))
        }
    )
        .bind(URL)?
        .run()
        .await
}
