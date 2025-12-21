//! Example showing customer creation, updates, and portal access
//!
//! Run with: cargo run --example customer_management

use polar_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("POLAR_ACCESS_TOKEN").expect("POLAR_ACCESS_TOKEN must be set");

    let client = PolarClient::new(access_token);

    // Create customer
    println!("Creating customer...");

    let create_request = CreateCustomerRequest {
        email: "john.doe@example.com".to_string(),
        name: Some("John Doe".to_string()),
        external_id: Some("user_123".to_string()),
        ..Default::default()
    };

    let customer = client.create_customer(create_request).await?;
    println!("Customer created: {} ({})", customer.email, customer.id);

    // Update customer
    println!("\nUpdating customer...");

    let address = BillingAddress {
        country: "US".to_string(),
        line1: Some("123 Main Street".to_string()),
        city: Some("New York".to_string()),
        state: Some("NY".to_string()),
        postal_code: Some("10001".to_string()),
        ..Default::default()
    };

    let update_request = UpdateCustomerRequest {
        name: Some("John Updated Doe".to_string()),
        billing_address: Some(address),
        ..Default::default()
    };

    let updated = client
        .update_customer_by_external_id("user_123", update_request)
        .await?;
    println!("Customer updated: {}", updated.name.unwrap_or_default());

    // Create customer portal session
    println!("\nCreating customer portal session...");

    let session_request = CreateCustomerSessionRequest {
        customer_id: Some(customer.id.clone()),
        ..Default::default()
    };

    let session = client.create_customer_session(session_request).await?;
    println!("Portal URL: {}", session.customer_portal_url);

    // List customers with pagination
    println!("\nListing customers...");

    let params = ListParams {
        page: Some(1),
        limit: Some(10),
    };

    let customers = client.list_customers(&params).await?;
    println!(
        "Found {} customers (Total: {})",
        customers.items.len(),
        customers.pagination.total_count
    );

    Ok(())
}
