// SPDX-License-Identifier: GPL-3.0

//! Client methods related to customers.

use crate::{
    PolarClient,
    error::Result,
    models::{
        ListParams, PaginatedResponse,
        customer::{CreateCustomerRequest, Customer, UpdateCustomerRequest},
    },
};

impl PolarClient {
    /// Create a new customer.
    ///
    /// Sends a request to create a customer in the Polar system and
    /// returns the created [`Customer`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::customer::CreateCustomerRequest};
    /// # async fn example(client: &PolarClient) {
    /// let req = CreateCustomerRequest {
    ///     email: "user@example.com".into(),
    ///     ..Default::default()
    /// };
    /// let customer = client.create_customer(req).await.unwrap();
    /// # }
    /// ```
    pub async fn create_customer(&self, req: CreateCustomerRequest) -> Result<Customer> {
        self.post("/customers/", &req).await
    }

    /// Retrieve a customer by their ID.
    pub async fn get_customer(&self, id: &str) -> Result<Customer> {
        self.get(&format!("/customers/{}", id)).await
    }

    /// Retrieve a customer by their external ID.
    pub async fn get_customer_by_external_id(&self, external_id: &str) -> Result<Customer> {
        self.get(&format!("/customers/external/{}", external_id))
            .await
    }

    /// Update an existing customer by their ID.
    pub async fn update_customer(&self, id: &str, req: UpdateCustomerRequest) -> Result<Customer> {
        self.patch(&format!("/customers/{}", id), &req).await
    }

    /// Update an existing customer by their external ID.
    pub async fn update_customer_by_external_id(
        &self,
        external_id: &str,
        req: UpdateCustomerRequest,
    ) -> Result<Customer> {
        self.patch(&format!("/customers/external/{}", external_id), &req)
            .await
    }

    /// Delete a customer by their ID.
    pub async fn delete_customer(&self, id: &str) -> Result<()> {
        self.delete(&format!("/customers/{}", id)).await
    }

    /// List all customers.
    ///
    /// Returns a paginated list of customers. Use `ListParams` to control pagination.
    pub async fn list_customers(&self, params: &ListParams) -> Result<PaginatedResponse<Customer>> {
        self.get_with_params("/customers/", params).await
    }
}
