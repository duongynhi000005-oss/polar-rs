// SPDX-License-Identifier: GPL-3.0

//! Models related to benefits.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A benefit definition.
///
/// Benefits represent entitlements that can be granted to customers,
/// such as access to features, content, or services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benefit {
    /// The unique identifier of the benefit.
    pub id: String,
    /// The type of the benefit.
    #[serde(rename = "type")]
    pub benefit_type: String,
    /// A human-readable description of the benefit.
    pub description: String,
    /// Indicates whether the benefit is subject to tax.
    pub is_tax_applicable: bool,
    /// The timestamp at which the benefit was created.
    pub created_at: String,
}

/// A granted benefit instance.
///
/// Benefit grants represent the assignment of a benefit to a specific
/// customer, optionally associated with a subscription or order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitGrant {
    /// The unique identifier of the benefit grant.
    pub id: String,
    /// The identifier of the associated benefit.
    pub benefit_id: String,
    /// The identifier of the customer receiving the benefit.
    pub customer_id: String,
    /// The identifier of the associated subscription, if applicable.
    pub subscription_id: Option<String>,
    /// The identifier of the associated order, if applicable.
    pub order_id: Option<String>,
    /// The timestamp at which the benefit was granted.
    pub granted_at: String,
    /// The timestamp at which the benefit was revoked, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    /// Additional properties associated with the benefit grant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
}
