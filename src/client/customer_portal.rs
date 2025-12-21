// SPDX-License-Identifier: GPL-3.0

//! Client methods related to the customer portal.

use crate::{
    PolarClient, PolarError,
    error::Result,
    models::{
        customer_session::{CreateCustomerSessionRequest, CustomerSession},
        license_keys::{
            ActivateLicenseKeyRequest, LicenseKeyActivation, LicenseKeyValidation,
            ValidateLicenseKeyRequest,
        },
    },
};

impl PolarClient {
    /// Create a customer session for the customer portal.
    ///
    /// This method generates a session token and URL that allows a
    /// customer to access the customer portal.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::customer_session::CreateCustomerSessionRequest};
    /// # async fn example(client: &PolarClient) {
    /// let req = CreateCustomerSessionRequest {
    ///     customer_id: Some("cus_123".into()),
    ///     customer_email: None,
    /// };
    /// let session = client.create_customer_session(req).await.unwrap();
    /// # }
    /// ```
    pub async fn create_customer_session(
        &self,
        req: CreateCustomerSessionRequest,
    ) -> Result<CustomerSession> {
        self.post("/customer-sessions/", &req).await
    }

    /// Validate a license key via the customer portal.
    ///
    /// This endpoint checks whether a license key is valid. It is
    /// an unauthenticated endpoint and returns a [`LicenseKeyValidation`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::license_keys::ValidateLicenseKeyRequest};
    /// # async fn example(client: &PolarClient) {
    /// let req = ValidateLicenseKeyRequest {
    ///     key: "license_123".into(),
    ///     organization_id: None,
    ///     activation_id: None,
    /// };
    /// let validation = client.validate_license_key(req).await.unwrap();
    /// # }
    /// ```
    pub async fn validate_license_key(
        &self,
        req: ValidateLicenseKeyRequest,
    ) -> Result<LicenseKeyValidation> {
        // Note: This is an unauthenticated endpoint
        let url = format!("{}/customer-portal/license-keys/validate", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| PolarError::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Activate a license key via the customer portal.
    ///
    /// This endpoint activates a license key for a given organization
    /// and returns a [`LicenseKeyActivation`] object.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::{PolarClient, models::license_keys::ActivateLicenseKeyRequest};
    /// # async fn example(client: &PolarClient) {
    /// let req = ActivateLicenseKeyRequest {
    ///     key: "license_123".into(),
    ///     organization_id: "org_456".into(),
    ///     label: "My activation".into(),
    /// };
    /// let activation = client.activate_license_key(req).await.unwrap();
    /// # }
    /// ```
    pub async fn activate_license_key(
        &self,
        req: ActivateLicenseKeyRequest,
    ) -> Result<LicenseKeyActivation> {
        let url = format!("{}/customer-portal/license-keys/activate", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| PolarError::Network(e.to_string()))?;

        self.handle_response(response).await
    }
}
