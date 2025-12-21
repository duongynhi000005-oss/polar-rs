// SPDX-License-Identifier: GPL-3.0

//! Models related to products.

use serde::{Deserialize, Serialize};

/// A product in the Polar system.
///
/// This structure represents a purchasable product, including
/// metadata and pricing characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// The unique identifier of the product.
    pub id: String,
    /// The name of the product.
    pub name: String,
    /// A description of the product, if available.
    pub description: Option<String>,
    /// Indicates whether the product is a recurring subscription.
    pub is_recurring: bool,
    /// The timestamp when the product was created.
    pub created_at: String,
}
