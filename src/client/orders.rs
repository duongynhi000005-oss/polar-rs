// SPDX-License-Identifier: GPL-3.0

//! Client methods related to orders.

use crate::{
    PolarClient,
    error::Result,
    models::{
        ListParams, PaginatedResponse,
        order::{Order, UpdateOrderRequest},
    },
};

impl PolarClient {
    /// Retrieve an order by its ID.
    ///
    /// Returns an [`Order`] object representing the specified order.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::PolarClient;
    /// # async fn example(client: &PolarClient) {
    /// let order = client.get_order("order_123").await.unwrap();
    /// # }
    /// ```
    pub async fn get_order(&self, id: &str) -> Result<Order> {
        self.get(&format!("/orders/{}", id)).await
    }

    /// List all orders with pagination support.
    ///
    /// Returns a [`PaginatedResponse`] containing a list of orders.
    /// Use `ListParams` to control page and limit.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::ListParams};
    /// # async fn example(client: &PolarClient) {
    /// let orders = client.list_orders(&Default::default()).await.unwrap();
    /// # }
    /// ```
    pub async fn list_orders(&self, params: &ListParams) -> Result<PaginatedResponse<Order>> {
        self.get_with_params("/orders/", params).await
    }

    /// Update an existing order by its ID.
    ///
    /// Sends a PATCH request with the fields to update (e.g., billing address)
    /// and returns the updated [`Order`] object.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::order::UpdateOrderRequest};
    /// # async fn example(client: &PolarClient) {
    /// let req = UpdateOrderRequest {
    ///     billing_address: None,
    /// };
    /// let updated = client.update_order("order_123", req).await.unwrap();
    /// # }
    /// ```
    pub async fn update_order(&self, id: &str, req: UpdateOrderRequest) -> Result<Order> {
        self.patch(&format!("/orders/{}", id), &req).await
    }
}
