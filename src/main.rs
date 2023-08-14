// #![feature(slice_pattern)]
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use clap::Parser;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use terrain_gen_api::{configuration::get_config, routes::wave_fn::wfc, routes::Heightmap};

use tracing::{debug, info};
use tracing_subscriber::prelude::*;
const MAX_SIZE: usize = 256;

#[derive(Serialize, Deserialize)]
pub struct FormData {
    exponent: i32,
    spreadRate: f32,
    seed: u64,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    _exponent: i32,
    #[arg(short = 'r', long)]
    _spread: f32,
    //TODO: user input for method (midpoint or wfc) and seed
    // #[arg(short, long)]
    // _method: String,
    #[arg(short = 's', long)]
    _seed: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct HeightmapResponse {
    url: String,
}

#[cfg(feature = "wfc")]
pub mod wfc {
    macro_rules! wfc {
    ($img:ty) => {
    if cfg!(feature = "wfc") {

       wfc::gen_wfc_image($img, "wfc_img_test.png");
    } else {
        "not wfc".to_string()
         $img
    }
    };
}
}

//TODO: route seed to heightmap , use xorshift to midpoint
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let console_lyr = console_subscriber::spawn();
    tracing_subscriber::fmt()
        // .with(console_lyr)
        .with_max_level(tracing::Level::DEBUG)
        // .with(tracing_subscriber::fmt::Layer::default().pretty())
        .init();
    let config = get_config().expect("failed to read config");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let args = Args::parse();
    let seed = args._seed.unwrap_or_else(|| rand::thread_rng().gen());
    let mut init_heightmap = Heightmap::new(args._exponent, args._spread, seed)
        .await
        .unwrap();
    tracing::info!("Seed: {}", seed);
    let _now = Instant::now();
    init_heightmap.midpnt_displacement().await.unwrap();
    let _elapsed = _now.elapsed();
    info!(
        "midpnt displacement took {} milliseconds",
        _elapsed.as_millis()
    );
    let img = init_heightmap
        .render_2d_test("./static/images/heightmap_test.png")
        .await
        .unwrap();
    if cfg!(feature = "wfc") {
        wfc::gen_wfc_image(img, "wfc_img_test.png");
    } else {
        println!("Not WFC");
    };

    info!("displaying 2d heightmap (init)");
    println!("{}", init_heightmap);
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
    let mut heightmap = Heightmap::new(params.exponent, params.spreadRate, params.seed)
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
