// SPDX-License-Identifier: GPL-3.0

//! Client for interacting with the Polar API.
//!
//! This module defines the main [`PolarClient`] type, which is used to
//! authenticate requests and access all Polar API resources.

use reqwest::{Client as HttpClient, header};

/// Modules related to benefits.
pub mod benefits;

/// Modules related to checkouts.
pub mod checkouts;

/// Modules related to the customer portal.
pub mod customer_portal;

/// Modules related to customer state.
pub mod customer_state;

/// Modules related to customers.
pub mod customers;

/// Internal helpers used by the client implementation.
mod helpers;

/// Modules related to orders.
pub mod orders;

/// Modules related to products.
pub mod products;

/// Modules related to subscriptions.
pub mod subscriptions;

/// A client for making authenticated requests to the Polar API.
///
/// This client handles authentication, request configuration, and
/// provides access to resource-specific API modules.
#[derive(Clone)]
pub struct PolarClient {
    /// The underlying HTTP client.
    http_client: HttpClient,
    /// The base URL for the Polar API.
    base_url: String,
}

impl PolarClient {
    /// Create a new client using the production Polar API.
    ///
    /// This method configures the client to use `https://api.polar.sh/v1`.
    pub fn new(access_token: impl Into<String>) -> Self {
        Self::with_base_url(access_token, "https://api.polar.sh/v1")
    }

    /// Create a new client using the Polar sandbox API.
    ///
    /// This is useful for testing and development without affecting
    /// production data.
    pub fn sandbox(access_token: impl Into<String>) -> Self {
        Self::with_base_url(access_token, "https://sandbox-api.polar.sh/v1")
    }

    /// Create a new client with a custom base URL.
    ///
    /// This method is primarily intended for testing or advanced use cases.
    pub fn with_base_url(access_token: impl Into<String>, base_url: impl Into<String>) -> Self {
        let token = access_token.into();
        let http_client = HttpClient::builder()
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::AUTHORIZATION,
                    header::HeaderValue::from_str(&format!("Bearer {}", token))
                        .expect("Invalid access token"),
                );
                headers.insert(
                    header::ACCEPT,
                    header::HeaderValue::from_static("application/json"),
                );
                headers
            })
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
            base_url: base_url.into(),
        }
    }
}
