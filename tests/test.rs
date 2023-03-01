#[cfg(test)]

mod tests {
    use log::*;
    use std::net::TcpListener;
    fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let server = terrain_gen_api::run(listener).expect("failed to bind address");
        //handle on spawned future
        let _ = tokio::spawn(server);
        format!("http://127.0.0.1:{}", port)
    }
    #[actix_rt::test]
    async fn health_check_test() {
        let address = spawn_app();
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/health_check", &address))
            .send()
            .await
            .expect("failed to execute request");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }

    #[actix_rt::test]
    async fn heightmap_with_valid_parameters_test() {
        todo!()
    }
}
