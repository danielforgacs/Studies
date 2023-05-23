use actix_web::HttpResponse;

pub async fn items() -> HttpResponse {
    HttpResponse::Ok()
        .body("<h1>Items</h1")
}
