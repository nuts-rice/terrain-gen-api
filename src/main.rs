use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use terrain_gen_api::configuration::get_config;

use tracing::info;

#[derive(Serialize, Deserialize)]
pub struct FormData {
    exponent: i32,
    spreadRate: f32,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let config = get_config().expect("failed to read config");
    let address = format!("{}:{}", config.application.host, config.application.port);

    info!("spawing server at {}", address);
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/post_input").route(web::post().to(handle_form)))
    })
    .bind(address)?
    .run()
    .await
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/www/index.html")))
}

async fn handle_form(params: web::Form<FormData>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
        "heightmap has size of {} by {} and spread rate of {}",
        params.exponent, params.exponent, params.spreadRate
    )))
}

//let subscriber = get_subscriber(1, "info".into(), std::io::stdout);
//init_subscriber(subscriber);
//let configuration = get_config().expect("failed to read configuration");
//let address = format!(
//    "{}:{}",
//    configuration.application.host, configuration.application.port
//);
////TODO: set to configuration with application port
// let address = format!("127.0.0.1:{}")
// let listener = TcpListener::bind(address)?;
// run(listener)?.await?;
//
//
