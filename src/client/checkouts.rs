// SPDX-License-Identifier: GPL-3.0

//! Client methods related to checkouts.

use crate::{
    PolarClient,
    error::Result,
    models::{
        ListParams, PaginatedResponse,
        checkout::{CheckoutSession, CreateCheckoutRequest},
    },
};

impl PolarClient {
    /// Create a new checkout session.
    ///
    /// This method initiates a checkout flow for one or more products
    /// and returns a [`CheckoutSession`] containing the session URL.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::checkout::CreateCheckoutRequest};
    /// # async fn example(client: &PolarClient) {
    /// let req = CreateCheckoutRequest {
    ///     products: vec!["prod_123".into()],
    ///     ..Default::default()
    /// };
    /// let session = client.create_checkout_session(req).await.unwrap();
    /// # }
    /// ```
    pub async fn create_checkout_session(
        &self,
        req: CreateCheckoutRequest,
    ) -> Result<CheckoutSession> {
        self.post("/checkouts/", &req).await
    }

    /// Retrieve an existing checkout session by its ID.
    ///
    /// This method fetches the details of a previously created
    /// checkout session.
    pub async fn get_checkout_session(&self, id: &str) -> Result<CheckoutSession> {
        self.get(&format!("/checkouts/{}", id)).await
    }

    /// List all checkout sessions.
    ///
    /// Returns a paginated list of checkout sessions. Use `ListParams`
    /// to control pagination.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::ListParams};
    /// # async fn example(client: &PolarClient) {
    /// let sessions = client.list_checkout_sessions(&Default::default()).await.unwrap();
    /// # }
    /// ```
    pub async fn list_checkout_sessions(
        &self,
        params: &ListParams,
    ) -> Result<PaginatedResponse<CheckoutSession>> {
        self.get_with_params("/checkouts/", params).await
    }
}
