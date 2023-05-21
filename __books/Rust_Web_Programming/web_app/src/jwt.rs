use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{Ready, ok};

#[derive(Debug)]
pub struct JWToken {
    pub message: String,
}

// This is the extractor for the token. this is used
// when an arument is added to a view.
// This doesn't seem to have anything to do with
// middleware that the app().wrap_fn() config does.
impl FromRequest for JWToken {
    type Error = Error;
    type Future = Ready<Result<JWToken, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let jwtoken = JWToken {
                    message: data.to_str().unwrap().to_string(),
                };
                ok(jwtoken)
            },
            None => {
                let jwtoken = JWToken {
                    message: "no token fond!".to_string(),
                };
                ok(jwtoken)
            }
        }
    }
}
