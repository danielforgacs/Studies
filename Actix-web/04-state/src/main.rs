use actix_web::{HttpServer, App, Responder, web};

struct AppState {
    app_name: String,
}

async fn hello_world(data: web::Data<AppState>) -> impl Responder {
    format!("Hello app: {}!", &data.app_name)
}

async fn app_name(data: web::Data<AppState>) -> impl Responder {
    format!("App name: {}.", &data.app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                AppState {
                    app_name: String::from("Actix Web"),
                }
            ))
            .route("/", web::get().to(hello_world))
            .route("/appname", web::get().to(app_name))
    })
        .bind("localhost:8090")?
        .run()
        .await
}
