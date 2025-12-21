// SPDX-License-Identifier: GPL-3.0

//! Client methods related to customer state.

use crate::{PolarClient, error::Result, models::customer_state::CustomerState};

impl PolarClient {
    /// Retrieve the aggregated state of a customer by their ID.
    ///
    /// This includes the customer details, active subscriptions, and
    /// granted benefits.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::PolarClient;
    /// # async fn example(client: &PolarClient) {
    /// let state = client.get_customer_state("cus_123").await.unwrap();
    /// # }
    /// ```
    pub async fn get_customer_state(&self, id: &str) -> Result<CustomerState> {
        self.get(&format!("/customers/{}/state", id)).await
    }

    /// Retrieve the aggregated state of a customer by their external ID.
    ///
    /// This includes the customer details, active subscriptions, and
    /// granted benefits. Useful when your system uses an external
    /// identifier for the customer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::PolarClient;
    /// # async fn example(client: &PolarClient) {
    /// let state = client.get_customer_state_by_external_id("ext_cus_456").await.unwrap();
    /// # }
    /// ```
    pub async fn get_customer_state_by_external_id(
        &self,
        external_id: &str,
    ) -> Result<CustomerState> {
        self.get(&format!("/customers/external/{}/state", external_id))
            .await
    }
}
