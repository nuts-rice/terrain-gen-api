use std::net::TcpListener;
use terrain_gen_api::configuration::get_config;
use terrain_gen_api::startup::run;
use terrain_gen_api::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(1.into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_config().expect("failed to read configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    //TODO: set to configuration with application port
    //let address = format!("127.0.0.1:{}")
    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;
    Ok(())
}
