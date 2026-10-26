// SPDX-License-Identifier: GPL-3.0

//! Models related to checkouts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A request to create a checkout session.
///
/// This structure defines the parameters used when initiating a
/// checkout flow for one or more products.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCheckoutRequest {
    /// The identifiers of the products being purchased.
    pub products: Vec<String>,
    /// The email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    /// An external identifier for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_customer_id: Option<String>,
    /// The URL to redirect the customer to after a successful checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    /// The billing address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_billing_address: Option<BillingAddress>,
    /// A discount code to apply to the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_code: Option<String>,
    /// Additional metadata associated with the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// A checkout session created by the Polar API.
///
/// This structure represents an active or completed checkout flow.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutSession {
    /// The unique identifier of the checkout session.
    pub id: String,
    /// The URL where the customer can complete the checkout.
    pub url: String,
    /// The identifier of the associated customer, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// The identifier of the purchased product.
    pub product_id: String,
    /// The current status of the checkout session.
    pub status: String,
    /// The email address of the customer, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
}

/// A billing address used during checkout.
///
/// This structure represents a postal address used for billing
/// and tax calculation purposes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillingAddress {
    /// The country code of the billing address.
    pub country: String,
    /// The first line of the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// The second line of the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// The city of the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The state or region of the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The postal or ZIP code of the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

impl CreateCheckoutRequest {
    /// Create a new checkout request for the given products.
    pub fn new(products: Vec<String>) -> Self {
        Self {
            products,
            ..Default::default()
        }
    }
}
