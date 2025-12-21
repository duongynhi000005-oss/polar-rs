// These tests would run against a real sandbox environment
// Only run with: cargo test --features integration-tests

#[cfg(feature = "integration-tests")]
mod integration {
    use polar_rs::prelude::*;
    use std::env;

    fn get_test_client() -> PolarClient {
        let token = env::var("POLAR_SANDBOX_TOKEN")
            .expect("POLAR_SANDBOX_TOKEN must be set for integration tests");
        PolarClient::sandbox(token)
    }

    #[tokio::test]
    async fn test_real_api_list_products() {
        let client = get_test_client();
        let params = ListParams {
            page: Some(1),
            limit: Some(10),
        };

        let result = client.list_products(&params).await;
        assert!(
            result.is_ok(),
            "Failed to list products: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_real_api_invalid_token() {
        let client = PolarClient::sandbox("invalid_token");
        let params = ListParams::default();

        let result = client.list_products(&params).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            PolarError::Api { status, .. } => {
                assert!(status == 401 || status == 403);
            }
            _ => panic!("Expected API error"),
        }
    }
}
