// SPDX-License-Identifier: GPL-3.0

//! Error types used across the Polar API client.
//!
//! This module defines the unified error type returned by all public
//! operations in this crate.

/// A specialized `Result` type for Polar operations.
///
/// This type alias is used throughout the crate to standardize error
/// handling and reduce repetition.
pub type Result<T> = std::result::Result<T, PolarError>;

/// Errors that can occur when interacting with the Polar API.
#[derive(Debug, thiserror::Error)]
pub enum PolarError {
    /// Errors originating from network or transport failures.
    #[error("Network error: {0}")]
    Network(String),

    /// Errors returned by the Polar API.
    ///
    /// This variant is typically used when the API responds with a
    /// non-success HTTP status code.
    #[error("API error (status {status}): {message}")]
    Api {
        /// The HTTP status code returned by the API.
        status: u16,
        /// The error message returned by the API.
        message: String,
    },

    /// Errors that occur during JSON deserialization.
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// Errors related to webhook validation or parsing.
    #[error("Invalid webhook: {0}")]
    InvalidWebhook(String),
}
