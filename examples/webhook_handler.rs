//! Example showing webhook signature verification and event handling
//!
//! Run with: cargo run --example webhook_handler

use polar_rs::{
    WebhookVerifier,
    webhook::{EventType, WebhookEvent},
};
use std::collections::HashMap;

fn main() {
    let webhook_secret =
        std::env::var("POLAR_WEBHOOK_SECRET").unwrap_or_else(|_| "whsec_test_secret".to_string());

    let verifier = WebhookVerifier::new(&webhook_secret);

    let body = r#"{
        "type": "order.paid",
        "data": {
            "id": "order_123",
            "customer_id": "cus_123",
            "amount": 1000,
            "currency": "usd"
        }
    }"#;

    let mut headers = HashMap::new();
    headers.insert("webhook-id".to_string(), "msg_123".to_string());
    headers.insert("webhook-timestamp".to_string(), "1234567890".to_string());

    let signature = compute_signature(&webhook_secret, "msg_123", "1234567890", body);
    headers.insert("webhook-signature".to_string(), format!("v1,{}", signature));

    println!("Verifying webhook signature...");

    match verifier.parse_event(&headers, body.as_bytes()) {
        Ok(event) => {
            println!("Signature verified successfully");
            println!("Event type: {}", event.event_type.as_str());

            handle_event(event);
        }
        Err(e) => {
            eprintln!("Verification failed: {}", e);
        }
    }
}

fn handle_event(event: WebhookEvent) {
    match event.event_type {
        EventType::OrderPaid => {
            println!("Processing paid order");
            if let Some(order_id) = event.data.get("id") {
                println!("Order ID: {}", order_id);
            }
        }
        EventType::SubscriptionActive => {
            println!("Subscription activated");
            if let Some(customer_id) = event.data.get("customer_id") {
                println!("Customer ID: {}", customer_id);
            }
        }
        EventType::SubscriptionCanceled => {
            println!("Subscription canceled");
        }
        EventType::BenefitGrantCreated => {
            println!("Benefit granted");
        }
        EventType::BenefitGrantRevoked => {
            println!("Benefit revoked");
        }
        EventType::Unknown => {
            println!("Unknown event type received");
        }
        _ => {
            println!("Unhandled event: {}", event.event_type.as_str());
        }
    }
}

// Only for testing purposes
fn compute_signature(secret: &str, id: &str, ts: &str, body: &str) -> String {
    use base64::{Engine, engine::general_purpose};
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let signed_content = format!("{}.{}.{}", id, ts, body);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(signed_content.as_bytes());
    general_purpose::STANDARD.encode(mac.finalize().into_bytes())
}
