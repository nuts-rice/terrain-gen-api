use actix_web::HttpResponse;

use std::fs;

pub async fn items() -> HttpResponse {
    let html_data = content_loader::read_file("./templates/main.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Items for heightmap stuff </h1>")
}

mod content_loader {

    pub fn read_file(file_path: &str) -> String {
        let data: String = fs::read_to_string(file_path).expect("unable to read file");
        return data;
    }
}
