//! Error types for Bybit API
//!
//! Comprehensive error handling for SDK operations.
//!
//! # Example
//!
//! ```rust,no_run
//! use rusty_bybit::{BybitError, BybitClient};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = BybitClient::testnet();
//!
//!     match client.get_server_time().await {
//!         Ok(time) => println!("Server time: {}", time.time_second),
//!         Err(BybitError::ApiError { ret_code, ret_msg }) => {
//!             if ret_code == 10006 {
//!                 eprintln!("Rate limit exceeded: {}", ret_msg);
//!             } else if ret_code == 110004 {
//!                 eprintln!("Insufficient balance: {}", ret_msg);
//!             } else {
//!                 eprintln!("API error {}: {}", ret_code, ret_msg);
//!             }
//!         }
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```

#[derive(Debug, thiserror::Error)]
pub enum BybitError {
    RequestError(#[from] reqwest::Error),

    ApiError {
        ret_code: i32,
        ret_msg: String,
    },

    InvalidTimestamp(String),

    SerializationError(#[from] serde_json::Error),

    InvalidParameter(String),

    AuthenticationError(String),

    RateLimitExceeded {
        limit_type: String,
        limit_reset_ms: Option<u64>,
    },

    InvalidEnumValue {
        enum_name: String,
        value: String,
    },

    MissingRequiredField {
        field_name: String,
    },
}

impl std::fmt::Display for BybitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BybitError::RequestError(e) => write!(f, "HTTP request failed: {}", e),
            BybitError::ApiError { ret_code, ret_msg } => {
                write!(f, "API error (code {}): {}", ret_code, ret_msg)
            }
            BybitError::InvalidTimestamp(msg) => {
                write!(f, "Invalid timestamp: {}", msg)
            }
            BybitError::SerializationError(e) => {
                write!(f, "Serialization error: {}", e)
            }
            BybitError::InvalidParameter(msg) => {
                write!(f, "Invalid parameter: {}", msg)
            }
            BybitError::AuthenticationError(msg) => {
                write!(f, "Authentication failed: {}", msg)
            }
            BybitError::RateLimitExceeded { limit_type, .. } => {
                write!(f, "Rate limit exceeded: {}", limit_type)
            }
            BybitError::InvalidEnumValue { enum_name, value } => {
                write!(f, "Invalid enum value for {}: {}", enum_name, value)
            }
            BybitError::MissingRequiredField { field_name } => {
                write!(f, "Missing required field: {}", field_name)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, BybitError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bybit_error_display_api_error() {
        let error = BybitError::ApiError {
            ret_code: 10001,
            ret_msg: "Invalid request".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("API error"));
        assert!(display.contains("10001"));
        assert!(display.contains("Invalid request"));
    }

    #[test]
    fn test_bybit_error_display_invalid_timestamp() {
        let error = BybitError::InvalidTimestamp("Timestamp expired".to_string());

        let display = format!("{}", error);
        assert!(display.contains("Invalid timestamp"));
        assert!(display.contains("Timestamp expired"));
    }

    #[test]
    fn test_bybit_error_display_serialization_error() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let error = BybitError::SerializationError(json_error);

        let display = format!("{}", error);
        assert!(display.contains("Serialization error"));
    }

    #[test]
    fn test_bybit_error_display_invalid_parameter() {
        let error = BybitError::InvalidParameter("Invalid symbol".to_string());

        let display = format!("{}", error);
        assert!(display.contains("Invalid parameter"));
        assert!(display.contains("Invalid symbol"));
    }

    #[test]
    fn test_bybit_error_display_authentication_error() {
        let error = BybitError::AuthenticationError("Invalid signature".to_string());

        let display = format!("{}", error);
        assert!(display.contains("Authentication failed"));
        assert!(display.contains("Invalid signature"));
    }

    #[test]
    fn test_bybit_error_display_rate_limit_exceeded() {
        let error = BybitError::RateLimitExceeded {
            limit_type: "API".to_string(),
            limit_reset_ms: Some(5000),
        };

        let display = format!("{}", error);
        assert!(display.contains("Rate limit exceeded"));
        assert!(display.contains("API"));
    }

    #[test]
    fn test_bybit_error_display_invalid_enum_value() {
        let error = BybitError::InvalidEnumValue {
            enum_name: "Side".to_string(),
            value: "InvalidSide".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("Invalid enum value"));
        assert!(display.contains("Side"));
        assert!(display.contains("InvalidSide"));
    }

    #[test]
    fn test_bybit_error_display_missing_required_field() {
        let error = BybitError::MissingRequiredField {
            field_name: "symbol".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("Missing required field"));
        assert!(display.contains("symbol"));
    }

    #[test]
    fn test_bybit_error_debug() {
        let error = BybitError::ApiError {
            ret_code: 10006,
            ret_msg: "Rate limit exceeded".to_string(),
        };

        let debug = format!("{:?}", error);
        assert!(debug.contains("ApiError"));
    }
}
