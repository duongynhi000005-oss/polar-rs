// SPDX-License-Identifier: GPL-3.0

//! Models related to customer state.

use crate::models::{benefit::BenefitGrant, customer::Customer, subscription::Subscription};
use serde::{Deserialize, Deserializer, Serialize};

/// The current state of a customer.
///
/// This structure represents an aggregated view of a customer,
/// including active subscriptions and granted benefits.
///
/// It supports both the live Polar API response (where customer properties
/// are at top level and subscriptions are under `active_subscriptions`) as well
/// as the nested `{ "customer": { ... } }` payload.
#[derive(Debug, Clone, Serialize)]
pub struct CustomerState {
    /// The customer associated with this state.
    pub customer: Customer,

    /// The subscriptions associated with the customer.
    pub subscriptions: Vec<Subscription>,

    /// The benefits granted to the customer.
    pub granted_benefits: Vec<BenefitGrant>,
}

impl CustomerState {
    /// Helper to get the customer's active subscriptions (alias for `subscriptions`).
    pub fn active_subscriptions(&self) -> &[Subscription] {
        &self.subscriptions
    }
}

impl<'de> Deserialize<'de> for CustomerState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = serde_json::Value::deserialize(deserializer)?;

        // 1. Resolve Customer: either nested in "customer" or flattened at top-level
        let customer: Customer = if let Some(cust_val) = val.get("customer").filter(|v| v.is_object()) {
            serde_json::from_value(cust_val.clone()).map_err(serde::de::Error::custom)?
        } else {
            serde_json::from_value(val.clone()).map_err(serde::de::Error::custom)?
        };

        // 2. Resolve Subscriptions: check "active_subscriptions", then "subscriptions"
        let mut subscriptions: Vec<Subscription> = if let Some(subs_val) = val
            .get("active_subscriptions")
            .or_else(|| val.get("subscriptions"))
            .filter(|v| v.is_array())
        {
            serde_json::from_value(subs_val.clone()).map_err(serde::de::Error::custom)?
        } else {
            Vec::new()
        };

        // Populate customer_id on subscriptions if omitted
        for sub in &mut subscriptions {
            if sub.customer_id.is_empty() {
                sub.customer_id = customer.id.clone();
            }
        }

        // 3. Resolve Granted Benefits
        let mut granted_benefits: Vec<BenefitGrant> = if let Some(b_val) = val
            .get("granted_benefits")
            .filter(|v| v.is_array())
        {
            serde_json::from_value(b_val.clone()).map_err(serde::de::Error::custom)?
        } else {
            Vec::new()
        };

        // Populate customer_id on granted benefits if omitted
        for grant in &mut granted_benefits {
            if grant.customer_id.is_empty() {
                grant.customer_id = customer.id.clone();
            }
        }

        Ok(CustomerState {
            customer,
            subscriptions,
            granted_benefits,
        })
    }
}
