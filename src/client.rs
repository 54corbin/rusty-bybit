//! HTTP client for Bybit v5 API
//!
//! Handles all HTTP requests, authentication, and response parsing.
//! Provides methods for public and authenticated API endpoints.
//!
//! # Authentication
//!
//! For authenticated endpoints, provide credentials via [`BybitClient::with_credentials`].
//! Authentication uses HMAC-SHA256 signature generation.
//!
//! # Example
//!
//! ````rust,no_run
//! use rusty_bybit::BybitClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = BybitClient::testnet();
//!     let time = client.get_server_time().await.unwrap();
//!     println!("Server time: {}", time.time_second);
//! }
//! ```

use crate::auth::{Credentials, generate_signature, get_current_timestamp_ms};
use crate::error::{BybitError, Result};
use crate::types::ApiResponse;
use reqwest::header::{HeaderMap, HeaderValue};

const RECV_WINDOW: u64 = 5000;

#[derive(Debug, Clone)]
pub struct BybitClient {
    pub base_url: String,
    http_client: reqwest::Client,
    credentials: Option<Credentials>,
}

impl BybitClient {
    pub fn new(base_url: String) -> Self {
        let http_client = reqwest::Client::builder()
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            http_client,
            credentials: None,
        }
    }

    pub fn with_credentials(mut self, api_key: String, api_secret: String) -> Self {
        self.credentials = Some(Credentials::new(api_key, api_secret));
        self
    }

    pub fn testnet() -> Self {
        Self::new("https://api-testnet.bybit.com".to_string())
    }

    pub fn mainnet() -> Self {
        Self::new("https://api.bybit.com".to_string())
    }

    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: &reqwest::Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let mut builder = self.http_client.request(method.clone(), &url);

        if let Some(q) = query {
            builder = builder.query(q);
        }

        if let Some(creds) = &self.credentials {
            let headers = self.build_auth_headers(method, path, query, body, creds)?;
            builder = builder.headers(headers);
        }

        if let Some(b) = body {
            builder = builder.json(b);
        }

        let response = builder.send().await?;
        let response_text = response.text().await?;

        let api_response: ApiResponse<T> = serde_json::from_str(&response_text)?;

        if api_response.ret_code != 0 {
            return Err(BybitError::ApiError {
                ret_code: api_response.ret_code,
                ret_msg: api_response.ret_msg,
            });
        }

        Ok(api_response.result)
    }

    pub(crate) async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: Option<Vec<(&str, &str)>>,
    ) -> Result<T> {
        self.request(&reqwest::Method::GET, path, query.as_deref(), None)
            .await
    }

    pub(crate) async fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        self.request(&reqwest::Method::POST, path, None, body.as_ref())
            .await
    }

    fn build_auth_headers(
        &self,
        method: &reqwest::Method,
        _path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<&serde_json::Value>,
        credentials: &Credentials,
    ) -> Result<HeaderMap> {
        let timestamp = get_current_timestamp_ms();

        let payload = match *method {
            reqwest::Method::GET => {
                if let Some(q) = query {
                    serde_urlencoded::to_string(q).unwrap_or_default()
                } else {
                    String::new()
                }
            }
            reqwest::Method::POST => {
                if let Some(b) = body {
                    serde_json::to_string(b).unwrap_or_default()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        };

        let signature = generate_signature(
            timestamp,
            &credentials.api_key,
            RECV_WINDOW,
            &payload,
            &credentials.api_secret,
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-BAPI-API-KEY",
            HeaderValue::try_from(credentials.api_key.as_str())
                .map_err(|e| BybitError::InvalidParameter(e.to_string()))?,
        );
        headers.insert(
            "X-BAPI-TIMESTAMP",
            HeaderValue::try_from(timestamp.to_string().as_str())
                .map_err(|e| BybitError::InvalidParameter(e.to_string()))?,
        );
        headers.insert(
            "X-BAPI-SIGN",
            HeaderValue::try_from(signature.as_str())
                .map_err(|e| BybitError::InvalidParameter(e.to_string()))?,
        );
        headers.insert(
            "X-BAPI-RECV-WINDOW",
            HeaderValue::try_from(RECV_WINDOW.to_string().as_str())
                .map_err(|e| BybitError::InvalidParameter(e.to_string()))?,
        );
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BybitClient::testnet();
        assert_eq!(client.base_url, "https://api-testnet.bybit.com");

        let client = BybitClient::mainnet();
        assert_eq!(client.base_url, "https://api.bybit.com");
    }

    #[test]
    fn test_client_with_credentials() {
        let client = BybitClient::testnet()
            .with_credentials("test_key".to_string(), "test_secret".to_string());
        assert!(client.credentials.is_some());
    }
}
