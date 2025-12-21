// SPDX-License-Identifier: GPL-3.0

//! Data models shared across Polar API endpoints.
//!
//! This module contains common response wrappers, pagination helpers,
//! and submodules defining concrete request/response types for specific
//! Polar resources such as products, subscriptions, customers, and orders.

use serde::{Deserialize, Serialize};

/// Models related to benefits.
pub mod benefit;

/// Models related to checkouts and payment sessions.
pub mod checkout;

/// Customer-related API models.
pub mod customer;

/// Models related to customer sessions.
pub mod customer_session;

/// Models related to customer state and entitlements.
pub mod customer_state;

/// Models related to license keys.
pub mod license_keys;

/// Models related to orders and purchases.
pub mod order;

/// Models related to products and pricing.
pub mod product;

/// Models related to subscriptions and recurring billing.
pub mod subscription;

/// A paginated response returned by Polar list endpoints.
///
/// Many Polar API endpoints return collections wrapped in this structure,
/// including pagination metadata for navigating large result sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The list of items for the current page.
    pub items: Vec<T>,
    /// Pagination information for the full result set.
    pub pagination: PaginationInfo,
}

/// Pagination information for a paginated response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    /// The total number of items available.
    pub total_count: i32,
    /// The maximum page number available.
    pub max_page: i32,
}

/// Parameters used when listing resources.
///
/// This struct is typically serialized into query parameters when
/// requesting paginated resources from the Polar API.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListParams {
    /// The page number to retrieve (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// The maximum number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
