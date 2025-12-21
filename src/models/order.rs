// SPDX-License-Identifier: GPL-3.0

//! Models related to orders.

use crate::models::checkout::BillingAddress;
use serde::{Deserialize, Serialize};

/// A request to update an existing order.
///
/// This structure defines fields that can be modified for an order,
/// such as the billing address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    /// The new billing address for the order, if updating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
}

/// An order in the Polar system.
///
/// This structure represents a purchase made by a customer,
/// including product, subscription, and payment details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// The unique identifier of the order.
    pub id: String,
    /// The identifier of the customer who placed the order.
    pub customer_id: String,
    /// The identifier of the purchased product.
    pub product_id: String,
    /// The identifier of the associated subscription, if any.
    pub subscription_id: Option<String>,
    /// The amount charged in the smallest currency unit (e.g., cents).
    pub amount: i64,
    /// The currency used for the order (ISO 4217 format).
    pub currency: String,
    /// The current status of the order.
    pub status: String,
    /// The timestamp when the order was created.
    pub created_at: String,
}
