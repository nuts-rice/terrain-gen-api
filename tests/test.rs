use terrain_gen_api::telemetry::get_subscriber;

#[feature(once_cell)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    use once_cell::sync::Lazy;
    use terrain_gen_api::telemetry::init_subscriber;

    /*
        let stdout_log = tracing_subscriber::fmt::layer().pretty();

        let file: File::create("debug.log")?;
        let debug_log = tracing_subscriber::fmt::layer()
            .with_writer(Arc::new(file))
    */

    static TRACING: Lazy<()> = Lazy::new(|| {
        let default_filter_level = "info".to_string();
        let test_size = 1;
        if std::env::var("TEST_LOG").is_ok() {
            let subscriber = get_subscriber(test_size, default_filter_level, std::io::stdout);
            init_subscriber(subscriber);
        } else {
            let subscriber = get_subscriber(test_size, default_filter_level, std::io::sink);
            init_subscriber(subscriber);
        };
    });

    pub struct TestApp {
        pub address: String,
    }

    async fn spawn_app() -> TestApp {
        Lazy::force(&TRACING);
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);
        let server = terrain_gen_api::startup::run(listener).expect("failed to bind address");
        //handle on spawned future
        let _ = tokio::spawn(server);
        TestApp { address }
    }

    #[tokio::test]
    async fn health_check_test() {
        //        tracing_subscriber::fmt()
        //            .with_max_level(tracing::Level::INFO)
        //            .init();

        let app = spawn_app().await;
        let client = reqwest::Client::new();
        let right = client
            .get(&format!("{}/health_check", &app.address))
            .send()
            .await
            .expect("failed to execute request");

        assert!(right.status().is_success());
        assert_eq!(Some(0), right.content_length());
    }

    #[tokio::test]
    async fn heightmap_with_valid_parameters_test() {
        let app = spawn_app().await;
        let client = reqwest::Client::new();
        //figure out params we need to build heightmap with midpnt displacement
        let body = "size=99&spread_rate=0.3";
        let right = client
            .post(&format!("{}/height_map", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(200, right.status().as_u16());
    }

    #[tokio::test]
    async fn heightmap_fields_present_but_invalid_test() {
        let app = spawn_app().await;
        let client = reqwest::Client::new();
        let should_panic = vec![
            ("size=&spread_rate=0.3", "empty size"),
            ("size=99&spread_rate=,", "empty spread rate"),
        ];
        for (element, description) in should_panic {
            let right = client
                .post(&format!("{}/height_map", &app.address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(element)
                .send()
                .await
                .expect("Failed to execute request");
            assert_eq!(
                400,
                right.status().as_u16(),
                "should've panic when payload was {}",
                description
            )
        }
    }

    #[tokio::test]
    async fn heightmap_with_invalid_parameters_test() {
        let app = spawn_app().await;
        let client = reqwest::Client::new();
        let should_panic = vec![
            ("size=100", "missing number of subdivisions"),
            ("spread_rate", "missing spread rate"),
        ];
        for (element, error_message) in should_panic {
            let right = client
                .post(&format!("{}/height_map", &app.address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(element)
                .send()
                .await
                .expect("Failed to execute request");
            assert_eq!(
                400,
                right.status().as_u16(),
                "should've panic when payload was {}",
                error_message
            );
        }
    }
}
/*
    #[actix_rt::test]
    async fn curve_with_valid_parameters() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        //let address = spawn_app();
        //let client = reqwest::Client::new();
        //TODO: figure form to hex
        //let body = "coefficients=07";
        todo!()
    }

    #[actix_rt::test]
    async fn eea_tabular_test() {
       todo!()
    }
}
*/
