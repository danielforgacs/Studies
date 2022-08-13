use actix_web::{
        web,
        HttpServer,
        App,
        HttpResponse,
    };

const URL: &str = "127.0.0.1:8888";

async fn alive() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("http://{}", URL);
    HttpServer::new(
        move || {
            App::new()
                .route("/alive", web::get().to(alive))
        }
    )
        .bind(URL)?
        .run()
        .await
}
