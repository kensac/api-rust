#[cfg(test)]
mod tests {
    use api_rust::utils::handle_404;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_handle_404() {
        let response = handle_404().await;
        assert_eq!(response.0, StatusCode::NOT_FOUND);
        assert_eq!(response.1, "We couldn't find the resource you requested.");
    }

    use api_rust::utils::hello_world;

    #[tokio::test]
    async fn test_hello_world() {
        let response = hello_world().await;
        assert_eq!(response, "Hello, World!");
    }

    use api_rust::utils::health_check;
    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, StatusCode::OK);
    }

    use api_rust::utils::get_port;
    use std::env;
    #[test]
    fn test_get_port() {
        env::set_var("PORT", "8080");
        assert_eq!(get_port(), 8080);

        env::remove_var("PORT");
        assert_eq!(get_port(), 3000); // default port
    }
}
