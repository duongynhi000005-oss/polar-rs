// SPDX-License-Identifier: GPL-3.0

//! Models related to customer sessions.

use serde::{Deserialize, Serialize};

/// A request to create a customer session.
///
/// This structure defines the parameters used to generate a session
/// for accessing the customer portal.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCustomerSessionRequest {
    /// The identifier of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// The email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
}

/// A customer session created by the Polar API.
///
/// This structure represents a temporary session that allows a customer
/// to access the customer portal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSession {
    /// The session token.
    pub token: String,
    /// The URL to the customer portal.
    pub customer_portal_url: String,
}
