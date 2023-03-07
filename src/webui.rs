use std::{thread, time::Duration};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "webui/build/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

async fn _index_spa() -> impl Responder {
    handle_embedded_file("index.html")
}

pub async fn start_webui() -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", 5478);
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        open::that("http://localhost:5478").unwrap();
    });
    HttpServer::new(move || App::new().service(index).service(dist))
        .bind(addr)?
        .run()
        .await
}
