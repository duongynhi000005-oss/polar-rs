// SPDX-License-Identifier: GPL-3.0

//! Client methods related to products.

use crate::{
    PolarClient,
    error::Result,
    models::{ListParams, PaginatedResponse, product::Product},
};

impl PolarClient {
    /// List all products with pagination support.
    ///
    /// Returns a [`PaginatedResponse`] containing a list of products.
    /// Use `ListParams` to control page and limit.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::ListParams};
    /// # async fn example(client: &PolarClient) {
    /// let products = client.list_products(&Default::default()).await.unwrap();
    /// # }
    /// ```
    pub async fn list_products(&self, params: &ListParams) -> Result<PaginatedResponse<Product>> {
        self.get_with_params("/products/", params).await
    }

    /// Retrieve a product by its ID.
    ///
    /// Returns a [`Product`] object representing the specified product.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::PolarClient;
    /// # async fn example(client: &PolarClient) {
    /// let product = client.get_product("prod_123").await.unwrap();
    /// # }
    /// ```
    pub async fn get_product(&self, id: &str) -> Result<Product> {
        self.get(&format!("/products/{}", id)).await
    }
}
