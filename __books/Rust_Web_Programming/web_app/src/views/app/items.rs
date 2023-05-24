use actix_web::HttpResponse;
use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let mut html_data = read_file("./templates/main.html");

    let js_data = read_file("./javascript/main.js");
    html_data = html_data.replace("{{JAVASCRIPT}}", &js_data);

    let css_data = read_file("./css/main.css");
    html_data = html_data.replace("{{CSS}}", &css_data);

    let base_css_data = read_file("./css/base.css");
    html_data = html_data.replace("{{BASE_CSS}}", &base_css_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
