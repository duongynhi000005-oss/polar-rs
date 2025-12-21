// SPDX-License-Identifier: GPL-3.0

//! Client methods related to subscriptions.

use crate::{
    PolarClient,
    error::Result,
    models::{
        ListParams, PaginatedResponse,
        subscription::{Subscription, UpdateSubscriptionRequest},
    },
};

impl PolarClient {
    /// Retrieve a subscription by its ID.
    ///
    /// Returns a [`Subscription`] object representing the specified subscription.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::PolarClient;
    /// # async fn example(client: &PolarClient) {
    /// let subscription = client.get_subscription("sub_123").await.unwrap();
    /// # }
    /// ```
    pub async fn get_subscription(&self, id: &str) -> Result<Subscription> {
        self.get(&format!("/subscriptions/{}", id)).await
    }

    /// List all subscriptions with pagination support.
    ///
    /// Returns a [`PaginatedResponse`] containing a list of subscriptions.
    /// Use `ListParams` to control page and limit.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::ListParams};
    /// # async fn example(client: &PolarClient) {
    /// let subs = client.list_subscriptions(&Default::default()).await.unwrap();
    /// # }
    /// ```
    pub async fn list_subscriptions(
        &self,
        params: &ListParams,
    ) -> Result<PaginatedResponse<Subscription>> {
        self.get_with_params("/subscriptions/", params).await
    }

    /// Update an existing subscription by its ID.
    ///
    /// Sends a PATCH request with the fields to update (e.g., product ID)
    /// and returns the updated [`Subscription`] object.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::subscription::UpdateSubscriptionRequest};
    /// # async fn example(client: &PolarClient) {
    /// let req = UpdateSubscriptionRequest {
    ///     product_id: Some("prod_456".into()),
    /// };
    /// let updated = client.update_subscription("sub_123", req).await.unwrap();
    /// # }
    /// ```
    pub async fn update_subscription(
        &self,
        id: &str,
        req: UpdateSubscriptionRequest,
    ) -> Result<Subscription> {
        self.patch(&format!("/subscriptions/{}", id), &req).await
    }

    /// Cancel a subscription by its ID.
    ///
    /// Sends a DELETE request and returns the canceled [`Subscription`] object.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::PolarClient;
    /// # async fn example(client: &PolarClient) {
    /// let canceled = client.cancel_subscription("sub_123").await.unwrap();
    /// # }
    /// ```
    pub async fn cancel_subscription(&self, id: &str) -> Result<Subscription> {
        self.delete(&format!("/subscriptions/{}", id)).await
    }
}
