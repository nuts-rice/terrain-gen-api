use actix_files::Files;
use actix_web::{App, HttpServer};
use tracing::info;

use terrain_gen_api::routes::height_map;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    info!("spawing server at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(height_map::new_heightmap)
            .service(Files::new("images", "static/images").show_files_listing())
            .service(Files::new("/", "./static/www/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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
