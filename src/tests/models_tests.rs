use polar_rs::models::*;
use serde_json::json;

#[test]
fn test_checkout_request_serialization() {
    let mut request = CreateCheckoutRequest::new(vec!["prod_123".to_string()]);
    request.customer_email = Some("test@example.com".to_string());
    request.external_customer_id = Some("user_123".to_string());
    request.success_url = Some("https://example.com/success".to_string());

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["products"][0], "prod_123");
    assert_eq!(json["customer_email"], "test@example.com");
    assert_eq!(json["external_customer_id"], "user_123");
}

#[test]
fn test_checkout_request_optional_fields_omitted() {
    let request = CreateCheckoutRequest::new(vec!["prod_123".to_string()]);

    let json = serde_json::to_value(&request).unwrap();
    assert!(!json.as_object().unwrap().contains_key("customer_email"));
    assert!(!json.as_object().unwrap().contains_key("success_url"));
}

#[test]
fn test_customer_state_deserialization() {
    let json = json!({
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
                "created_at": "2024-01-01T00:00:00Z"
            }
        ],
        "granted_benefits": []
    });

    let state: CustomerState = serde_json::from_value(json).unwrap();
    assert_eq!(state.customer.id, "cus_123");
    assert_eq!(state.customer.external_id, Some("user_123".to_string()));
    assert_eq!(state.subscriptions.len(), 1);
    assert_eq!(state.granted_benefits.len(), 0);
}

#[test]
fn test_billing_address_serialization() {
    let address = BillingAddress {
        country: "US".to_string(),
        line1: Some("123 Main St".to_string()),
        line2: None,
        city: Some("New York".to_string()),
        state: Some("NY".to_string()),
        postal_code: Some("10001".to_string()),
    };

    let json = serde_json::to_value(&address).unwrap();
    assert_eq!(json["country"], "US");
    assert_eq!(json["line1"], "123 Main St");
    assert_eq!(json["city"], "New York");
}

#[test]
fn test_paginated_response_deserialization() {
    let json = json!({
        "items": [
            {
                "id": "prod_1",
                "name": "Product 1",
                "is_recurring": false,
                "created_at": "2024-01-01T00:00:00Z"
            }
        ],
        "pagination": {
            "total_count": 100,
            "max_page": 10
        }
    });

    let response: PaginatedResponse<Product> = serde_json::from_value(json).unwrap();
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.pagination.total_count, 100);
    assert_eq!(response.pagination.max_page, 10);
}

#[test]
fn test_list_params_serialization() {
    let params = ListParams {
        page: Some(2),
        limit: Some(50),
    };

    let json = serde_json::to_value(&params).unwrap();
    assert_eq!(json["page"], 2);
    assert_eq!(json["limit"], 50);
}

#[test]
fn test_list_params_default() {
    let params = ListParams::default();
    let json = serde_json::to_value(&params).unwrap();

    // Default values should not be serialized
    assert!(!json.as_object().unwrap().contains_key("page"));
    assert!(!json.as_object().unwrap().contains_key("limit"));
}
