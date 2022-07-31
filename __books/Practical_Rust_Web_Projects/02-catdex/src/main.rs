pub mod models;
pub mod schema;

use self::models::*;
use self::schema::cats::dsl::*;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
// use diesel::pg::PgConnection;
use diesel::r2d2;
use awmp;

use actix_files::{Files};
use actix_web::{App, HttpServer, HttpResponse, Error};
use actix_web::web;
use serde::Serialize;
use handlebars::Handlebars;
use dotenv;

/*
// PgConnection comes from diesel::prelude
THE BOOK CREATES A TYPE ALIAS:
type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
*/

#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    cats: Vec<self::models::Cat>
}


async fn index(
    handlebars: web::Data<Handlebars<'_>>,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>)
    -> Result<HttpResponse, Error>
{
    let connection = pool.get().expect("Can't get connection from pool.");
    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&connection))
        .await
        .unwrap()
        .unwrap();

    let data = IndexTemplateData {
        project_name: "Catdex".to_string(),
        cats: cats_data,
    };
    let body = handlebars.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

async fn add(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, Error> {
    let body = handlebars.render("add", &{}).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

async fn add_cat_form(
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse, Error> {
    let mut file_path = parts
        .files
        .take("image")
        .pop()
        .and_then(|f| f.persist_in("./static/images").ok())
        .unwrap_or_default();
    let text_fields: std::collections::HashMap<_, _> = parts
        .texts
        .as_pairs()
        .into_iter()
        .collect();
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./static/").unwrap();
    let handlebars_webdata = web::Data::new(handlebars);

    // setting up the db connection pool
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("Set the DATABASE_URL env var.");
            return Ok(());
        }
    };
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool");

    println!("http://localhost:8090");
    println!("http://localhost:8090/add");

    HttpServer::new(move || {
        App::new()
        .app_data(handlebars_webdata.clone())
        .data(pool.clone())
        .service(
            Files::new("/static", "static")
            // This lists (servs) files in the static folder
            // in the /static path for debugging
            .show_files_listing()
        )
        .route("/", web::get().to(index))
        .route("/add", web::get().to(add))
        .route("/add_cat_form", web::post().to(add_cat_form))
    })
    .bind("localhost:8090")?
    .run()
    .await
}
