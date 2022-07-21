use actix_files::{NamedFile, Files};
use actix_web::{App, HttpServer, Result};
use actix_web::web;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://localhost:8090");
    HttpServer::new(|| {
        App::new()
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
