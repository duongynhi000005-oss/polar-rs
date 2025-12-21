//! Basic example showing checkout creation and customer state checking
//!
//! Run with: cargo run --example basic_usage

use polar_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("POLAR_ACCESS_TOKEN").expect("POLAR_ACCESS_TOKEN must be set");

    let client = PolarClient::new(access_token);

    // Create a checkout session
    println!("Creating checkout session...");

    let checkout_request = CreateCheckoutRequest {
        products: vec!["prod_YOUR_PRODUCT_ID".to_string()],
        customer_email: Some("customer@example.com".to_string()),
        external_customer_id: Some("user_123".to_string()),
        success_url: Some("https://yourapp.com/success".to_string()),
        ..Default::default()
    };

    match client.create_checkout_session(checkout_request).await {
        Ok(checkout) => {
            println!("Checkout created successfully");
            println!("ID: {}", checkout.id);
            println!("URL: {}", checkout.url);
            println!("Status: {}", checkout.status);
        }
        Err(e) => {
            eprintln!("Error creating checkout: {}", e);
            return Err(e.into());
        }
    }

    // Check customer state
    println!("\nChecking customer state...");

    match client.get_customer_state_by_external_id("user_123").await {
        Ok(state) => {
            println!("Customer: {} ({})", state.customer.email, state.customer.id);
            println!("Active subscriptions: {}", state.subscriptions.len());

            for sub in &state.subscriptions {
                println!("  Subscription: {}", sub.id);
                println!("  Status: {}", sub.status);
                println!("  Product: {}", sub.product_id);
            }

            println!("Granted benefits: {}", state.granted_benefits.len());
        }
        Err(e) => {
            eprintln!("Error fetching customer state: {}", e);
        }
    }

    Ok(())
}
