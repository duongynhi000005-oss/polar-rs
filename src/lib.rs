// SPDX-License-Identifier: GPL-3.0

//! # Polar Rust SDK
//!
//! A Rust library for interacting with the [polar.sh](https://polar.sh) payment
//! and monetization platform.
//!
//! This crate provides:
//! - A [`PolarClient`] for making authenticated API requests
//! - Strongly typed request/response models
//! - Error handling via [`PolarError`]
//! - Webhook signature verification via [`WebhookVerifier`]
//!
//! Most users should start with the [`prelude`] module for convenient imports.

/// HTTP client and API interaction logic.
pub mod client;

/// Error types and result aliases used throughout the crate.
pub mod error;

/// Data models representing Polar API requests and responses.
pub mod models;

/// Webhook verification utilities.
pub mod webhook;

pub use client::PolarClient;
pub use error::{PolarError, Result};
pub use webhook::{EventType, WebhookEvent, WebhookVerifier};

/// Commonly used imports for working with this crate.
///
/// Importing the prelude brings the most frequently needed types into scope:
///
/// ```rust
/// use polar_rs::prelude::*;
/// ```
///
/// This includes:
/// - [`PolarClient`] for API access
/// - [`PolarError`] and [`Result`] for error handling
/// - All public models
/// - [`WebhookVerifier`] for validating incoming webhooks
pub mod prelude {
    pub use crate::client::PolarClient;
    pub use crate::error::PolarError;
    pub use crate::models::*;
    pub use crate::webhook::WebhookVerifier;

    // Models re-export
    pub use crate::models::benefit::{Benefit, BenefitGrant};
    pub use crate::models::checkout::{BillingAddress, CheckoutSession, CreateCheckoutRequest};
    pub use crate::models::customer::{CreateCustomerRequest, Customer, UpdateCustomerRequest};
    pub use crate::models::customer_session::{CreateCustomerSessionRequest, CustomerSession};
    pub use crate::models::customer_state::CustomerState;
    pub use crate::models::license_keys::{
        ActivateLicenseKeyRequest, LicenseKeyActivation, LicenseKeyValidation,
        ValidateLicenseKeyRequest,
    };
    pub use crate::models::order::{Order, UpdateOrderRequest};
    pub use crate::models::product::Product;
    pub use crate::models::subscription::{Subscription, UpdateSubscriptionRequest};
}
