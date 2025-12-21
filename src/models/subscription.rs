// SPDX-License-Identifier: GPL-3.0

//! Models related to subscriptions.

use serde::{Deserialize, Serialize};

/// A request to update an existing subscription.
///
/// This structure defines fields that can be modified for a subscription,
/// such as the associated product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionRequest {
    /// The identifier of the product associated with the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

/// A subscription in the Polar system.
///
/// This structure represents a recurring subscription for a customer,
/// including its status, associated product, and billing period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// The unique identifier of the subscription.
    pub id: String,
    /// The current status of the subscription (e.g., active, canceled).
    pub status: String,
    /// The identifier of the product associated with the subscription.
    pub product_id: String,
    /// The identifier of the customer who owns the subscription.
    pub customer_id: String,
    /// The start timestamp of the current billing period.
    pub current_period_start: String,
    /// The end timestamp of the current billing period.
    pub current_period_end: String,
    /// Whether the subscription will be canceled at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// The timestamp when the subscription was created.
    pub created_at: String,
}
