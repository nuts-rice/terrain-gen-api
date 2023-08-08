use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(height_map::new_heightmap)
            .route("/health_check", web::get().to(health_check::health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
