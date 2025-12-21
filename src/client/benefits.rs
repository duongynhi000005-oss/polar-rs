// SPDX-License-Identifier: GPL-3.0

//! Client methods related to benefits.

use crate::{
    PolarClient,
    error::Result,
    models::{
        ListParams, PaginatedResponse,
        benefit::{Benefit, BenefitGrant},
    },
};

impl PolarClient {
    /// List all benefits available in the Polar system.
    ///
    /// This method retrieves a paginated list of benefits. You can
    /// control pagination via the provided `ListParams`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::PolarClient;
    /// # async fn example(client: &PolarClient) {
    /// let benefits = client.list_benefits(&Default::default()).await.unwrap();
    /// # }
    /// ```
    pub async fn list_benefits(&self, params: &ListParams) -> Result<PaginatedResponse<Benefit>> {
        self.get_with_params("/benefits/", params).await
    }

    /// List all grants associated with a specific benefit.
    ///
    /// This method retrieves a paginated list of `BenefitGrant` objects
    /// for the given `benefit_id`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polar_rs::PolarClient;
    /// # async fn example(client: &PolarClient) {
    /// let grants = client.list_benefit_grants("benefit_123", &Default::default()).await.unwrap();
    /// # }
    /// ```
    pub async fn list_benefit_grants(
        &self,
        benefit_id: &str,
        params: &ListParams,
    ) -> Result<PaginatedResponse<BenefitGrant>> {
        self.get_with_params(&format!("/benefits/{}/grants", benefit_id), params)
            .await
    }
}
