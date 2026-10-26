use polar_rs::prelude::*;
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

#[tokio::test]
async fn test_create_checkout_session_success() {
    let mock_server = MockServer::start().await;

    let mock_response = json!({
        "id": "checkout_123",
        "url": "https://polar.sh/checkout/checkout_123",
        "customer_id": "cus_123",
        "product_id": "prod_123",
        "status": "open",
        "customer_email": "test@example.com"
    });

    Mock::given(method("POST"))
        .and(path("/v1/checkouts/"))
        .and(header("Authorization", "Bearer test_token"))
        .and(header("Accept", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));

    let request = CreateCheckoutRequest::new(vec!["prod_123".to_string()]);
    let mut request = request;
    request.customer_email = Some("test@example.com".to_string());
    request.external_customer_id = Some("user_123".to_string());
    request.success_url = Some("https://example.com/success".to_string());

    let result = client.create_checkout_session(request).await;

    assert!(result.is_ok());
    let checkout = result.unwrap();
    assert_eq!(checkout.id, "checkout_123");
    assert_eq!(checkout.url, "https://polar.sh/checkout/checkout_123");
    assert_eq!(checkout.status, "open");
}

#[tokio::test]
async fn test_create_checkout_session_api_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/checkouts/"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid product_id"
        })))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));

    let request = CreateCheckoutRequest::new(vec!["invalid_prod".to_string()]);

    let result = client.create_checkout_session(request).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        PolarError::Api { status, .. } => assert_eq!(status, 400),
        _ => panic!("Expected API error"),
    }
}

#[tokio::test]
async fn test_get_customer_state_by_external_id() {
    let mock_server = MockServer::start().await;

    let mock_response = json!({
        "customer": {
            "id": "cus_123",
            "email": "test@example.com",
            "external_id": "user_123",
            "name": "Test User",
            "created_at": "2024-01-01T00:00:00Z"
        },
        "subscriptions": [
            {
                "id": "sub_123",
                "status": "active",
                "product_id": "prod_123",
                "customer_id": "cus_123",
                "current_period_start": "2024-01-01T00:00:00Z",
                "current_period_end": "2024-02-01T00:00:00Z",
                "cancel_at_period_end": false,
                "created_at": "2024-01-01T00:00:00Z"
            }
        ],
        "granted_benefits": [
            {
                "id": "grant_123",
                "benefit_id": "benefit_123",
                "customer_id": "cus_123",
                "subscription_id": "sub_123",
                "granted_at": "2024-01-01T00:00:00Z"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/v1/customers/external/user_123/state"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));
    let result = client.get_customer_state_by_external_id("user_123").await;

    assert!(result.is_ok());
    let state = result.unwrap();
    assert_eq!(state.customer.id, "cus_123");
    assert_eq!(state.customer.external_id, Some("user_123".to_string()));
    assert_eq!(state.subscriptions.len(), 1);
    assert_eq!(state.subscriptions[0].status, "active");
    assert_eq!(state.granted_benefits.len(), 1);
}

#[tokio::test]
async fn test_get_customer_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/customers/external/nonexistent/state"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Customer not found"
        })))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));
    let result = client
        .get_customer_state_by_external_id("nonexistent")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        PolarError::Api { status, .. } => assert_eq!(status, 404),
        _ => panic!("Expected API error"),
    }
}

#[tokio::test]
async fn test_list_subscriptions_with_pagination() {
    let mock_server = MockServer::start().await;

    let mock_response = json!({
        "items": [
            {
                "id": "sub_1",
                "status": "active",
                "product_id": "prod_123",
                "customer_id": "cus_123",
                "current_period_start": "2024-01-01T00:00:00Z",
                "current_period_end": "2024-02-01T00:00:00Z",
                "created_at": "2024-01-01T00:00:00Z"
            },
            {
                "id": "sub_2",
                "status": "active",
                "product_id": "prod_456",
                "customer_id": "cus_456",
                "current_period_start": "2024-01-01T00:00:00Z",
                "current_period_end": "2024-02-01T00:00:00Z",
                "created_at": "2024-01-01T00:00:00Z"
            }
        ],
        "pagination": {
            "total_count": 10,
            "max_page": 5
        }
    });

    Mock::given(method("GET"))
        .and(path("/v1/subscriptions/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));
    let params = ListParams {
        page: Some(1),
        limit: Some(2),
    };

    let result = client.list_subscriptions(&params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.items.len(), 2);
    assert_eq!(response.pagination.total_count, 10);
    assert_eq!(response.pagination.max_page, 5);
}

#[tokio::test]
async fn test_update_customer_by_external_id() {
    let mock_server = MockServer::start().await;

    let mock_response = json!({
        "id": "cus_123",
        "email": "newemail@example.com",
        "external_id": "user_123",
        "name": "Updated Name",
        "created_at": "2024-01-01T00:00:00Z"
    });

    Mock::given(method("PATCH"))
        .and(path("/v1/customers/external/user_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));
    let update_req = UpdateCustomerRequest {
        email: Some("newemail@example.com".to_string()),
        name: Some("Updated Name".to_string()),
        external_id: None,
        billing_address: None,
        metadata: None,
    };

    let result = client
        .update_customer_by_external_id("user_123", update_req)
        .await;

    assert!(result.is_ok());
    let customer = result.unwrap();
    assert_eq!(customer.email, "newemail@example.com");
    assert_eq!(customer.name, Some("Updated Name".to_string()));
}

#[tokio::test]
async fn test_cancel_subscription() {
    let mock_server = MockServer::start().await;

    let mock_response = json!({
        "id": "sub_123",
        "status": "canceled",
        "product_id": "prod_123",
        "customer_id": "cus_123",
        "current_period_start": "2024-01-01T00:00:00Z",
        "current_period_end": "2024-02-01T00:00:00Z",
        "cancel_at_period_end": true,
        "created_at": "2024-01-01T00:00:00Z"
    });

    Mock::given(method("DELETE"))
        .and(path("/v1/subscriptions/sub_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));
    let result = client.cancel_subscription("sub_123").await;

    assert!(result.is_ok());
    let subscription = result.unwrap();
    assert_eq!(subscription.status, "canceled");
}

#[tokio::test]
async fn test_create_customer_session() {
    let mock_server = MockServer::start().await;

    let mock_response = json!({
        "token": "cust_portal_token_123",
        "customer_portal_url": "https://polar.sh/portal/cust_portal_token_123"
    });

    Mock::given(method("POST"))
        .and(path("/v1/customer-sessions/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));
    let request = CreateCustomerSessionRequest {
        customer_id: Some("cus_123".to_string()),
        customer_email: None,
    };

    let result = client.create_customer_session(request).await;

    assert!(result.is_ok());
    let session = result.unwrap();
    assert_eq!(session.token, "cust_portal_token_123");
    assert!(session.customer_portal_url.contains("portal"));
}

#[tokio::test]
async fn test_validate_license_key() {
    let mock_server = MockServer::start().await;

    let mock_response = json!({
        "valid": true,
        "key": "POLAR-XXXX-XXXX",
        "customer_id": "cus_123"
    });

    Mock::given(method("POST"))
        .and(path("/v1/customer-portal/license-keys/validate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let client = PolarClient::with_base_url("test_token", format!("{}/v1", mock_server.uri()));
    let request = ValidateLicenseKeyRequest {
        key: "POLAR-XXXX-XXXX".to_string(),
        organization_id: Some("org_123".to_string()),
        activation_id: None,
    };

    let result = client.validate_license_key(request).await;

    assert!(result.is_ok());
    let validation = result.unwrap();
    assert!(validation.valid);
    assert_eq!(validation.key, Some("POLAR-XXXX-XXXX".to_string()));
}
