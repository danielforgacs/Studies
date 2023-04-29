use actix_web::dev::ServiceRequest;

fn check_password(password: String) -> Result<String, &'static str> {
    if password == "token" {
        Ok(password)
    } else {
        Err("Token not authorised")
    }
}
