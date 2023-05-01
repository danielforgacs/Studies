mod json_serialization;
mod processes;
mod state;
mod to_do;

use actix_web::dev::Service;
use processes::process_input;
use state::{write_to_file};
use to_do::{ItemTypes};
use views::token::process_token;

pub const PERSISTENCE_FILE_NAME: &str = "./state.json";

/*
// THIS IS MAIN FROM CHAPTER 02 TO_DO APP PROJECT.
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let command = &args.get(1).expect("[ERROR] Add a command argument.");
    let title = args.get(2).expect("[ERROR] Add a title argument.");
    let mut state = read_file(PERSISTENCE_FILE_NAME);
    let status: String;
    match state.get(title) {
        Some(result) => {status = result.to_string().replace('\"', "")},
        None => {status = "pending".to_string()},
    }
    let item = to_do_factory(&status, title).expect(&status);
    println!("factory item: {:?}", &item);
    process_input(item, command.to_string(), &mut state);
}
*/

mod views;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                if req.path().contains("/item/") {
                    match process_token(&req) {
                        Ok(_) => println!("The token is passable!"),
                        Err(message) => println!("token error: {}", message),
                    }
                };
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
