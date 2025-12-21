use polar_rs::{WebhookVerifier, webhook::WebhookEvent};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_webhook_verification_success() {
    let secret = "whsec_test_secret";
    let verifier = WebhookVerifier::new(secret);

    // Simulated webhook payload
    let body = json!({
        "type": "order.paid",
        "data": {
            "id": "order_123",
            "amount": 1000
        }
    })
    .to_string();

    let webhook_id = "msg_123";
    let webhook_timestamp = "1234567890";

    // Manually compute the expected signature
    let signed_content = format!("{}.{}.{}", webhook_id, webhook_timestamp, body);
    let expected_sig = compute_hmac_signature(secret, &signed_content);

    let mut headers = HashMap::new();
    headers.insert(
        "webhook-signature".to_string(),
        format!("v1,{}", expected_sig),
    );
    headers.insert("webhook-id".to_string(), webhook_id.to_string());
    headers.insert(
        "webhook-timestamp".to_string(),
        webhook_timestamp.to_string(),
    );

    let result = verifier.verify(&headers, body.as_bytes());
    assert!(result.is_ok());
}

#[test]
fn test_webhook_verification_invalid_signature() {
    let secret = "whsec_test_secret";
    let verifier = WebhookVerifier::new(secret);

    let body = json!({
        "type": "order.paid",
        "data": {}
    })
    .to_string();

    let mut headers = HashMap::new();
    headers.insert(
        "webhook-signature".to_string(),
        "v1,invalid_signature".to_string(),
    );
    headers.insert("webhook-id".to_string(), "msg_123".to_string());
    headers.insert("webhook-timestamp".to_string(), "1234567890".to_string());

    let result = verifier.verify(&headers, body.as_bytes());
    assert!(result.is_err());
}

#[test]
fn test_webhook_verification_missing_headers() {
    let secret = "whsec_test_secret";
    let verifier = WebhookVerifier::new(secret);

    let body = b"test body";
    let headers = HashMap::new();

    let result = verifier.verify(&headers, body);
    assert!(result.is_err());

    match result.unwrap_err() {
        polar_rs::PolarError::InvalidWebhook(msg) => {
            assert!(msg.contains("Missing webhook-signature"));
        }
        _ => panic!("Expected InvalidWebhook error"),
    }
}

#[test]
fn test_webhook_parse_event_success() {
    let secret = "whsec_test_secret";
    let verifier = WebhookVerifier::new(secret);

    let body = json!({
        "type": "subscription.active",
        "data": {
            "id": "sub_123",
            "status": "active"
        }
    })
    .to_string();

    let webhook_id = "msg_123";
    let webhook_timestamp = "1234567890";
    let signed_content = format!("{}.{}.{}", webhook_id, webhook_timestamp, body);
    let signature = compute_hmac_signature(secret, &signed_content);

    let mut headers = HashMap::new();
    headers.insert("webhook-signature".to_string(), format!("v1,{}", signature));
    headers.insert("webhook-id".to_string(), webhook_id.to_string());
    headers.insert(
        "webhook-timestamp".to_string(),
        webhook_timestamp.to_string(),
    );

    let result = verifier.parse_event(&headers, body.as_bytes());
    assert!(result.is_ok());

    let event = result.unwrap();
    assert_eq!(event.event_type, "subscription.active");
    assert_eq!(event.data["id"], "sub_123");
}

#[test]
fn test_webhook_parse_event_invalid_json() {
    let secret = "whsec_test_secret";
    let verifier = WebhookVerifier::new(secret);

    let body = b"not valid json";
    let webhook_id = "msg_123";
    let webhook_timestamp = "1234567890";
    let signed_content = format!(
        "{}.{}.{}",
        webhook_id,
        webhook_timestamp,
        std::str::from_utf8(body).unwrap()
    );
    let signature = compute_hmac_signature(secret, &signed_content);

    let mut headers = HashMap::new();
    headers.insert("webhook-signature".to_string(), format!("v1,{}", signature));
    headers.insert("webhook-id".to_string(), webhook_id.to_string());
    headers.insert(
        "webhook-timestamp".to_string(),
        webhook_timestamp.to_string(),
    );

    let result = verifier.parse_event(&headers, body);
    assert!(result.is_err());

    match result.unwrap_err() {
        polar_rs::PolarError::Deserialization(_) => {}
        _ => panic!("Expected Deserialization error"),
    }
}

#[test]
fn test_webhook_multiple_signatures() {
    let secret = "whsec_test_secret";
    let verifier = WebhookVerifier::new(secret);

    let body = json!({"type": "test"}).to_string();
    let webhook_id = "msg_123";
    let webhook_timestamp = "1234567890";
    let signed_content = format!("{}.{}.{}", webhook_id, webhook_timestamp, body);
    let valid_sig = compute_hmac_signature(secret, &signed_content);

    let mut headers = HashMap::new();
    // Multiple signatures, one valid
    headers.insert(
        "webhook-signature".to_string(),
        format!("v1,invalid_sig v1,{}", valid_sig),
    );
    headers.insert("webhook-id".to_string(), webhook_id.to_string());
    headers.insert(
        "webhook-timestamp".to_string(),
        webhook_timestamp.to_string(),
    );

    let result = verifier.verify(&headers, body.as_bytes());
    assert!(result.is_ok());
}

// Helper function to compute HMAC signature for tests
fn compute_hmac_signature(secret: &str, data: &str) -> String {
    use base64::{Engine as _, engine::general_purpose};
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let result = mac.finalize();
    general_purpose::STANDARD.encode(result.into_bytes())
}
