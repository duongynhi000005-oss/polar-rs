// SPDX-License-Identifier: GPL-3.0

//! Models related to license keys.

use serde::{Deserialize, Serialize};

/// A request to validate a license key.
///
/// This structure defines the parameters used to check the validity
/// of a license key for a customer or organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateLicenseKeyRequest {
    /// The license key to validate.
    pub key: String,
    /// The identifier of the organization associated with the key, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// The activation identifier, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
}

/// The result of validating a license key.
///
/// This structure represents whether a license key is valid
/// and optionally provides associated details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseKeyValidation {
    /// Whether the license key is valid.
    pub valid: bool,
    /// The license key that was validated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The customer associated with the license key, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}

/// A request to activate a license key.
///
/// This structure defines the parameters required to activate
/// a license key for a specific organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateLicenseKeyRequest {
    /// The license key to activate.
    pub key: String,
    /// The identifier of the organization activating the key.
    pub organization_id: String,
    /// A label to associate with the activation.
    pub label: String,
}

/// A record of an activated license key.
///
/// This structure represents a successful license key activation
/// including its identifier and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseKeyActivation {
    /// The unique identifier of the activation.
    pub id: String,
    /// The identifier of the license key being activated.
    pub license_key_id: String,
    /// The label associated with the activation.
    pub label: String,
    /// The timestamp when the license key was activated.
    pub created_at: String,
}
