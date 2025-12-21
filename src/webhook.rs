// SPDX-License-Identifier: GPL-3.0

//! Utilities for verifying and parsing Polar webhooks.
//!
//! This module provides functionality for validating webhook signatures
//! using the Standard Webhooks specification and deserializing verified
//! webhook events.

use crate::error::{PolarError, Result};
use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

/// A verifier for Polar webhook signatures.
///
/// This type is used to validate incoming webhook requests by checking
/// their signatures against a shared webhook secret.
pub struct WebhookVerifier {
    webhook_secret: String,
}

impl WebhookVerifier {
    /// Create a new webhook verifier.
    ///
    /// The `webhook_secret` must match the secret configured in the
    /// Polar dashboard for the corresponding webhook endpoint.
    pub fn new(webhook_secret: impl Into<String>) -> Self {
        Self {
            webhook_secret: webhook_secret.into(),
        }
    }

    /// Verify the signature of a webhook request.
    ///
    /// This method validates the `webhook-signature` header according
    /// to the Standard Webhooks specification and ensures that the
    /// request body has not been tampered with.
    pub fn verify(&self, headers: &HashMap<String, String>, body: &[u8]) -> Result<()> {
        let signature_header = headers
            .get("webhook-signature")
            .or_else(|| headers.get("Webhook-Signature"))
            .ok_or_else(|| PolarError::InvalidWebhook("Missing webhook-signature header".into()))?;

        // Parse signature header (format: "v1,signature1 v1,signature2")
        let signatures: Vec<&str> = signature_header
            .split_whitespace()
            .filter(|s| s.starts_with("v1,"))
            .map(|s| s.trim_start_matches("v1,"))
            .collect();

        if signatures.is_empty() {
            return Err(PolarError::InvalidWebhook(
                "No valid v1 signatures found".into(),
            ));
        }

        // Get webhook ID and timestamp
        let webhook_id = headers
            .get("webhook-id")
            .or_else(|| headers.get("Webhook-Id"))
            .ok_or_else(|| PolarError::InvalidWebhook("Missing webhook-id header".into()))?;

        let webhook_timestamp = headers
            .get("webhook-timestamp")
            .or_else(|| headers.get("Webhook-Timestamp"))
            .ok_or_else(|| PolarError::InvalidWebhook("Missing webhook-timestamp header".into()))?;

        // Construct signed content
        let body_str = std::str::from_utf8(body)
            .map_err(|_| PolarError::InvalidWebhook("Invalid UTF-8 in body".into()))?;
        let signed_content = format!("{}.{}.{}", webhook_id, webhook_timestamp, body_str);

        // Verify at least one signature matches
        let expected_signature = self.compute_signature(&signed_content)?;

        for sig in signatures {
            if constant_time_compare(sig.as_bytes(), expected_signature.as_bytes()) {
                return Ok(());
            }
        }

        Err(PolarError::InvalidWebhook(
            "Signature verification failed".into(),
        ))
    }

    fn compute_signature(&self, signed_content: &str) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.webhook_secret.as_bytes())
            .map_err(|_| PolarError::InvalidWebhook("Invalid webhook secret".into()))?;
        mac.update(signed_content.as_bytes());
        let result = mac.finalize();
        Ok(general_purpose::STANDARD.encode(result.into_bytes()))
    }

    /// Parse and verify a webhook request, returning the event payload.
    ///
    /// This method first verifies the webhook signature and then
    /// deserializes the request body into a [`WebhookEvent`].
    pub fn parse_event(
        &self,
        headers: &HashMap<String, String>,
        body: &[u8],
    ) -> Result<WebhookEvent> {
        self.verify(headers, body)?;

        let event: WebhookEvent =
            serde_json::from_slice(body).map_err(|e| PolarError::Deserialization(e.to_string()))?;

        Ok(event)
    }
}

/// Perform a constant-time comparison of two byte slices.
///
/// This function is used to prevent timing attacks when comparing
/// webhook signatures.
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// A verified Polar webhook event.
///
/// This structure represents the top-level payload sent by Polar
/// webhook requests.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEvent {
    /// The type of the webhook event.
    #[serde(rename = "type")]
    pub event_type: EventType,
    /// The event-specific payload.
    pub data: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // Benefits
    #[serde(rename = "benefit.created")]
    BenefitCreated,
    #[serde(rename = "benefit.updated")]
    BenefitUpdated,

    // Benefit Grants
    #[serde(rename = "benefit_grant.created")]
    BenefitGrantCreated,
    #[serde(rename = "benefit_grant.updated")]
    BenefitGrantUpdated,
    #[serde(rename = "benefit_grant.revoked")]
    BenefitGrantRevoked,
    #[serde(rename = "benefit_grant.cycled")]
    BenefitGrantCycled,

    // Checkouts
    #[serde(rename = "checkout.created")]
    CheckoutCreated,
    #[serde(rename = "checkout.updated")]
    CheckoutUpdated,

    // Orders
    #[serde(rename = "order.created")]
    OrderCreated,
    #[serde(rename = "order.paid")]
    OrderPaid,
    #[serde(rename = "order.updated")]
    OrderUpdated,
    #[serde(rename = "order.refunded")]
    OrderRefunded,

    // Subscriptions
    #[serde(rename = "subscription.created")]
    SubscriptionCreated,
    #[serde(rename = "subscription.active")]
    SubscriptionActive,
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated,
    #[serde(rename = "subscription.canceled")]
    SubscriptionCanceled,
    #[serde(rename = "subscription.uncanceled")]
    SubscriptionUncanceled,
    #[serde(rename = "subscription.revoked")]
    SubscriptionRevoked,

    // Customers
    #[serde(rename = "customer.created")]
    CustomerCreated,
    #[serde(rename = "customer.updated")]
    CustomerUpdated,
    #[serde(rename = "customer.deleted")]
    CustomerDeleted,
    #[serde(rename = "customer.state_changed")]
    CustomerStateChanged,

    // Customer Seats
    #[serde(rename = "customer_seat.assigned")]
    CustomerSeatAssigned,
    #[serde(rename = "customer_seat.claimed")]
    CustomerSeatClaimed,
    #[serde(rename = "customer_seat.revoked")]
    CustomerSeatRevoked,

    // Products
    #[serde(rename = "product.created")]
    ProductCreated,
    #[serde(rename = "product.updated")]
    ProductUpdated,

    // Organizations
    #[serde(rename = "organization.updated")]
    OrganizationUpdated,

    // Refunds
    #[serde(rename = "refund.created")]
    RefundCreated,
    #[serde(rename = "refund.updated")]
    RefundUpdated,

    // Catch-all for unknown events
    #[serde(other)]
    Unknown,
}

impl EventType {
    pub fn as_str(&self) -> &str {
        match self {
            EventType::BenefitCreated => "benefit.created",
            EventType::BenefitUpdated => "benefit.updated",
            EventType::BenefitGrantCreated => "benefit_grant.created",
            EventType::BenefitGrantUpdated => "benefit_grant.updated",
            EventType::BenefitGrantRevoked => "benefit_grant.revoked",
            EventType::BenefitGrantCycled => "benefit_grant.cycled",
            EventType::CheckoutCreated => "checkout.created",
            EventType::CheckoutUpdated => "checkout.updated",
            EventType::OrderCreated => "order.created",
            EventType::OrderPaid => "order.paid",
            EventType::OrderUpdated => "order.updated",
            EventType::OrderRefunded => "order.refunded",
            EventType::SubscriptionCreated => "subscription.created",
            EventType::SubscriptionActive => "subscription.active",
            EventType::SubscriptionUpdated => "subscription.updated",
            EventType::SubscriptionCanceled => "subscription.canceled",
            EventType::SubscriptionUncanceled => "subscription.uncanceled",
            EventType::SubscriptionRevoked => "subscription.revoked",
            EventType::CustomerCreated => "customer.created",
            EventType::CustomerUpdated => "customer.updated",
            EventType::CustomerDeleted => "customer.deleted",
            EventType::CustomerStateChanged => "customer.state_changed",
            EventType::CustomerSeatAssigned => "customer_seat.assigned",
            EventType::CustomerSeatClaimed => "customer_seat.claimed",
            EventType::CustomerSeatRevoked => "customer_seat.revoked",
            EventType::ProductCreated => "product.created",
            EventType::ProductUpdated => "product.updated",
            EventType::OrganizationUpdated => "organization.updated",
            EventType::RefundCreated => "refund.created",
            EventType::RefundUpdated => "refund.updated",
            EventType::Unknown => "unknown",
        }
    }
}
