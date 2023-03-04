#[cfg(test)]
mod tests {

    use std::net::TcpListener;

    use tracing::subscriber::set_global_default;
    use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

    /*
        let stdout_log = tracing_subscriber::fmt::layer().pretty();

        let file: File::create("debug.log")?;
        let debug_log = tracing_subscriber::fmt::layer()
            .with_writer(Arc::new(file))
    */

    fn spawn_app() -> String {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let fmt_layer = BunyanFormattingLayer::new("terrain-gen-api".into(), std::io::stdout);
        let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(fmt_layer);
        set_global_default(subscriber)
            .expect("oh no you posted cringe S: now you will lose subscriber ");
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let server = terrain_gen_api::startup::run(listener).expect("failed to bind address");
        //handle on spawned future
        let _ = tokio::spawn(server);
        format!("http://127.0.0.1:{}", port)
    }

    #[actix_rt::test]
    async fn health_check_test() {
        //        tracing_subscriber::fmt()
        //            .with_max_level(tracing::Level::INFO)
        //            .init();

        let address = spawn_app();
        let client = reqwest::Client::new();
        tracing::info!("client requesting health check...");
        let response = client
            .get(&format!("{}/health_check", &address))
            .send()
            .await
            .expect("failed to execute request");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        tracing::info!("health check sucessful");
    }

    #[actix_rt::test]
    async fn heightmap_with_valid_parameters_test() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        //figure out params we need to build heightmap with midpnt displacement
        let body = "size=100&nsubdivs=20";
        let left = client
            .post(&format!("{}/height_map", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(200, left.status().as_u16());
    }

    #[actix_rt::test]
    async fn heightmap_with_invalid_parameters_test() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        let should_panic = vec![
            ("size=100", "missing number of subdivisions"),
            ("nsubdivs=20", "missing size"),
        ];
        for (element, error_message) in should_panic {
            let left = client
                .post(&format!("{}/height_map", &address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(element)
                .send()
                .await
                .expect("Failed to execute request");
            assert_eq!(
                400,
                left.status().as_u16(),
                "should've panic when payload was {}",
                error_message
            );
        }
    }
}
