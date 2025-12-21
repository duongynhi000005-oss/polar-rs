// SPDX-License-Identifier: GPL-3.0

//! Helper HTTP methods for the PolarClient.

use crate::{
    PolarClient,
    error::{PolarError, Result},
    models::ListParams,
};

impl PolarClient {
    /// Send a GET request to the specified path and return the deserialized JSON response.
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| PolarError::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Send a GET request with query parameters to the specified path and return the deserialized JSON response.
    pub async fn get_with_params<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &ListParams,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| PolarError::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Send a POST request with a JSON body to the specified path and return the deserialized JSON response.
    pub async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| PolarError::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Send a PATCH request with a JSON body to the specified path and deserialize the response.
    pub async fn patch<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .patch(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| PolarError::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Send a DELETE request to the specified path and deserialize the response.
    pub async fn delete<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .delete(&url)
            .send()
            .await
            .map_err(|e| PolarError::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Handle an HTTP response, returning the deserialized JSON or an error.
    ///
    /// This method checks the HTTP status code. If the response indicates
    /// an error, it returns a [`PolarError::Api`] with the status and body.
    /// Otherwise, it attempts to deserialize the JSON into type `T`.
    pub async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(PolarError::Api {
                status: status.as_u16(),
                message: text,
            });
        }

        response
            .json()
            .await
            .map_err(|e| PolarError::Deserialization(e.to_string()))
    }
}
