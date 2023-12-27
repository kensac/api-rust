#[cfg(test)]
mod tests {
    use api_rust::{base_types::*, prisma::PrismaClient};
    use axum::http::StatusCode;
    use std::sync::Arc;

    // Test for AppState creation
    #[tokio::test]
    async fn test_app_state_creation() {
        // Use MockPrismaClient instead of PrismaClient for testing
        let client = PrismaClient::_builder()
            .build()
            .await
            .expect("Didn't connect to database");
        let app_state = AppState {
            client: Arc::new(client),
            reqwest_client: reqwest::Client::new(),
        };

        assert!(
            Arc::strong_count(&app_state.client) > 0,
            "Prisma client should be initialized."
        );
    }

    // Tests for the response types
    // Note: These tests are basic and may need to be extended based on actual usage.

    // Test for GetResponse
    #[test]
    fn test_get_response() {
        let response: GetResponse<&str> = Ok((StatusCode::OK, "Data"));
        assert!(response.is_ok());
        if let Ok((status, data)) = response {
            assert_eq!(status, StatusCode::OK);
            assert_eq!(data, "Data");
        }
    }

    // Test for CreateResponse
    #[test]
    fn test_create_response() {
        let response: CreateResponse = Ok((StatusCode::CREATED, ()));
        assert!(response.is_ok());
        if let Ok((status, _)) = response {
            assert_eq!(status, StatusCode::CREATED);
        }
    }

    // Test for DeleteResponse
    #[test]
    fn test_delete_response() {
        let response: DeleteResponse = Ok((StatusCode::NO_CONTENT, ()));
        assert!(response.is_ok());
        if let Ok((status, _)) = response {
            assert_eq!(status, StatusCode::NO_CONTENT);
        }
    }

    // Test for UpdateResponse
    #[test]
    fn test_update_response() {
        let response: UpdateResponse = Ok((StatusCode::OK, ()));
        assert!(response.is_ok());
        if let Ok((status, _)) = response {
            assert_eq!(status, StatusCode::OK);
        }
    }
}
