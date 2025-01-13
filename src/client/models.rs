use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::AnthropicClient;

/// Client implementation for interacting with Anthropic's model API endpoints.
impl AnthropicClient {
    /// Retrieves a list of all available models from the Anthropic API.
    ///
    /// # Returns
    /// * `Result<GetModelsBody, anyhow::Error>` - A Result containing either:
    ///   * `GetModelsBody` - The successful response containing model information
    ///   * `anyhow::Error` - Any error that occurred during the request
    ///
    /// # Errors
    /// Returns an error if:
    /// * The HTTP request fails
    /// * The response status is not 200
    /// * The response body cannot be parsed

    /// Retrieves a filtered list of models based on the provided query parameters.
    ///
    /// # Arguments
    /// * `params` - Query parameters to filter the models
    ///
    /// # Returns
    /// * `Result<GetModelsBody, anyhow::Error>` - A Result containing either:
    ///   * `GetModelsBody` - The successful response containing filtered model information
    ///   * `anyhow::Error` - Any error that occurred during the request
    ///
    /// # Errors
    /// Returns an error if:
    /// * The query parameters cannot be serialized
    /// * The HTTP request fails
    /// * The response status is not 200
    /// * The response body cannot be parsed
    pub async fn get_models(&self) -> Result<GetModelsBody, anyhow::Error> {
        let url = format!("{}/v1/models", self.api_url);
        let response = self
            .client
            .get(&url)
            .header("anthropic-version", "2023-06-01")
            .header("x-api-key", &self.api_key)
            .send()
            .await?;
        if response.status() != 200 {
            return Err(anyhow::anyhow!(response.text().await?));
        }
        let body: GetModelsBody = response.json().await?;
        Ok(body)
    }

    /// Retrieves model information from the Anthropic API with specified query parameters
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering model results ([`GetModelsQueryParams`])
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing [`GetModelsBody`] on success, or an error if:
    /// - The API request fails
    /// - Response status is not 200
    /// - Response body cannot be parsed
    ///

    pub async fn get_model_with_params(
        &self,
        params: GetModelsQueryParams,
    ) -> Result<GetModelsBody, anyhow::Error> {
        let url = format!("{}/v1/models", self.api_url);
        let response = self
            .client
            .get(&url)
            .header("anthropic-version", "2023-06-01")
            .header("x-api-key", &self.api_key)
            .query(&params)
            .send()
            .await?;
        println!("Test");
        println!("{:#?}", response.url());
        if response.status() != StatusCode::OK {
            return Err(anyhow::anyhow!(response.text().await?));
        }
        let body: GetModelsBody = response.json().await?;
        Ok(body)
    }
    pub async fn get_model_by_id(&self, model_id: String) -> Result<Model, anyhow::Error> {
        let url = format!("{}/v1/models/{}", self.api_url, model_id);
        let response = self
            .client
            .get(&url)
            .header("anthropic-version", "2023-06-01")
            .header("x-api-key", &self.api_key)
            .send()
            .await?;
        if response.status() != StatusCode::OK {
            return Err(anyhow::anyhow!(response.text().await?));
        }
        let body: Model = response.json().await?;
        Ok(body)
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct GetModelsQueryParams {
    before_id: Option<String>,
    after_id: Option<String>,
    limit: Option<i32>,
}
impl Default for GetModelsQueryParams {
    fn default() -> Self {
        GetModelsQueryParams {
            before_id: None,
            after_id: None,
            limit: None,
        }
    }
}
impl GetModelsQueryParams {
    pub fn new(before_id: Option<String>, after_id: Option<String>, limit: Option<i32>) -> Self {
        GetModelsQueryParams {
            before_id,
            after_id,
            limit,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct GetModelsBody {
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
    pub data: Vec<Model>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub model_type: ModelEnums,
    pub created_at: String,
}
#[derive(Debug, Serialize, Deserialize)]

pub enum ModelEnums {
    #[serde(rename = "model")]
    Models,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_models() {
        dotenvy::dotenv().ok();
        let client = AnthropicClient::default().unwrap();
        let models = client.get_models().await.unwrap();

        assert!(models.data.len() > 1);
    }
    #[tokio::test]

    async fn test_get_models_with_params() {
        dotenvy::dotenv().ok();
        let client = AnthropicClient::default().unwrap();
        let models = client
            .get_model_with_params(GetModelsQueryParams {
                before_id: None,
                after_id: None,
                limit: Some(1),
            })
            .await
            .unwrap();
        println!("{:#?}", models);
        assert!(models.data.len() == 1);
    }
    #[tokio::test]
    async fn test_get_models_by_id() {
        dotenvy::dotenv().ok();
        let client = AnthropicClient::default().unwrap();
        let models = client
            .get_model_by_id("claude-3-5-sonnet-20241022".to_string())
            .await
            .unwrap();
        assert_eq!(models.id, "claude-3-5-sonnet-20241022");
    }
}
