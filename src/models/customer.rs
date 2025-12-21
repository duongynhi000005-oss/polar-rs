// SPDX-License-Identifier: GPL-3.0

//! Models related to customers.

use crate::models::checkout::BillingAddress;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A request to create a new customer.
///
/// This structure defines the parameters required to create a customer
/// in the Polar system.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCustomerRequest {
    /// The email address of the customer.
    pub email: String,
    /// An external identifier for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The full name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Additional metadata associated with the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// A request to update an existing customer.
///
/// This structure defines the parameters that can be updated for
/// a customer in the Polar system.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCustomerRequest {
    /// The new email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The new external identifier for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The new full name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The new billing address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Updated metadata associated with the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// A customer in the Polar system.
///
/// This structure represents a customer record including
/// identifiers, contact information, billing address, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// The unique identifier of the customer.
    pub id: String,
    /// The email address of the customer.
    pub email: String,
    /// The external identifier of the customer, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The full name of the customer, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The billing address of the customer, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /// Additional metadata associated with the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The timestamp when the customer was created.
    pub created_at: String,
}
