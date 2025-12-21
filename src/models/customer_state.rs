// SPDX-License-Identifier: GPL-3.0

//! Models related to customer state.

use crate::models::{benefit::BenefitGrant, customer::Customer, subscription::Subscription};
use serde::{Deserialize, Serialize};

/// The current state of a customer.
///
/// This structure represents an aggregated view of a customer,
/// including active subscriptions and granted benefits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerState {
    /// The customer associated with this state.
    pub customer: Customer,

    /// The subscriptions associated with the customer.
    pub subscriptions: Vec<Subscription>,

    /// The benefits granted to the customer.
    pub granted_benefits: Vec<BenefitGrant>,
}
