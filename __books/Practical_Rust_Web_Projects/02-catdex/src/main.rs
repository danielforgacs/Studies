use actix_files::{Files};
use actix_web::{App, HttpServer, HttpResponse};
use actix_web::web;
use serde_json::json;
use handlebars::Handlebars;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Catdex",
        "cats": [
            {
                "name": "British short hair",
                "image_path": "/static/image/british-short-hair.jpg",
            },
            {
                "name": "Persian",
                "image_path": "/static/image/persian.jpg",
            },
            {
                "name": "ragdoll",
                "image_path": "/static/image/ragdoll.jpg",
            },
        ]
    });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://localhost:8090");

    let mut hb = Handlebars::new();
    hb.register_templates_directory(".html", "./static/").unwrap();
    let hb_ref = web::Data::new(hb);

    HttpServer::new(move || {
        App::new()
        .app_data(hb_ref.clone())
        .service(
            Files::new("/static", "static")
            // This lists (servs) files in the static folder
            // in the /static path for debugging
            .show_files_listing()
        )
        .route("/", web::get().to(index))
    })
    .bind("localhost:8090")?
    .run()
    .await
}
