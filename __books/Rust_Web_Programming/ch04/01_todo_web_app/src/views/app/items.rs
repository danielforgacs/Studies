use super::content_loader::read_file;
use super::content_loader::add_component;
use actix_web::HttpResponse;

pub async fn items() -> HttpResponse {
    let js_data = read_file("./javascript/main.js");
    let css_data = read_file("./css/main.css");
    let base_css_data = read_file("./css/base.css");
    let html_data = read_file("./templates/main.html")
    .replace("{{JAVASCRIPT}}", &js_data)
    .replace("{{BASE_CSS}}", &base_css_data)
    .replace("{{CSS}}", &css_data);
    let html_data = add_component("header".to_string(), html_data);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
