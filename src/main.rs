#![feature(slice_pattern)]
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use clap::{Parser};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use terrain_gen_api::{configuration::get_config, routes::Heightmap};
use tracing::{debug, info};

const MAX_SIZE: usize = 256;

#[derive(Serialize, Deserialize)]
pub struct FormData {
    exponent: i32,
    spreadRate: f32,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    _exponent : i32,
    #[arg(short, long)]
    _spread : f32
}


#[derive(Serialize, Deserialize)]
pub struct HeightmapResponse {
    url: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let config = get_config().expect("failed to read config");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let args = Args::parse();
    let mut init_heightmap = Heightmap::new(args._exponent, args._spread).await.unwrap();
    info!("initializing heightmap with size of {} by {} and spread rate of {} ", args._exponent, args._exponent, args._spread);
    init_heightmap.midpnt_displacement().await.unwrap();
    init_heightmap
        .render_2d_test("./static/images/heightmap_test.png")
        .await
        .unwrap();

    info!("spawing server at {}", address);
    HttpServer::new(|| {
        App::new()
            // web::scope("/")
            .service(Files::new("/static", "./static").show_files_listing())
            .service(Files::new("/static", "./static/www").show_files_listing())
            .service(Files::new("/static", "./static/images").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
            // .service(Files::new("/static", "../static/" ).index_file("index.html"))
            .service(
                web::resource("/post_input")
                    .app_data(web::PayloadConfig::default().limit(MAX_SIZE))
                    .route(web::post().to(handle_form)),
            )
    })
    .bind(address)?
    .run()
    .await
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

async fn handle_form(params: web::Form<FormData>) -> Result<HttpResponse> {
    let mut heightmap = Heightmap::new(params.exponent, params.spreadRate)
        .await
        .unwrap();
    let _time_midpnt = Instant::now();
    heightmap.midpnt_displacement().await.unwrap();
    let elapsed = _time_midpnt.elapsed();
    tracing::info!(
        "midpoint displacement took {} milliseconds ",
        elapsed.as_millis()
    );
    let _time_render = Instant::now();
    heightmap.render("web_heightmap.png").await.unwrap();
    let elapsed_render = _time_render.elapsed();
    tracing::info!("2d render took {} milliseconds", elapsed_render.as_millis());
    let _time_3d_render = Instant::now();
    heightmap.render_3d_test().await.unwrap();
    let elapsed_render_3d = _time_3d_render.elapsed();
    tracing::info!(
        "3d render arc test took {} milliseconds",
        elapsed_render_3d.as_millis()
    );

    debug!(
        "heightmap has size of {} by {} and spread rate of {}",
        params.exponent, params.exponent, params.spreadRate
    );
    let response = Heightmap::render_3d_test(&heightmap);

    Ok(HttpResponse::Ok().json(response.await.unwrap()))
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
