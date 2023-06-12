use actix_files::Files;
use actix_web::{App, HttpServer};
use terrain_gen_api::routes::height_map;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(height_map::new_heightmap)
            .service(Files::new("images", "static/images").show_files_listing())
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
