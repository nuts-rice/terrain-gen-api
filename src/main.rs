


use std::net::TcpListener;
use terrain_gen_api::startup::run;
use terrain_gen_api::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(1.into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    //TODO: set to configuration with application port
    //let address = format!("127.0.0.1:{}")
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await?;
    Ok(())
}
