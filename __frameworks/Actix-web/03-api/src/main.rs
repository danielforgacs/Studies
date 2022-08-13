use actix_web::{HttpServer, App};

const URL: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("http://{}", URL);
    HttpServer::new(
        move || {
            App::new()
        }
    )
        .bind(URL)?
        .run()
        .await
}
