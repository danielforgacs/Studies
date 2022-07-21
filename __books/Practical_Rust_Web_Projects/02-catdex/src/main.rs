use actix_files::{NamedFile};
use actix_web::{App, HttpServer, Result};
use actix_web::web;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://localhost:8090");
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("localhost:8090")?
    .run()
    .await
}
