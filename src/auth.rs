//! Authentication utilities for Bybit v5 API
//!
//! Provides HMAC-SHA256 signature generation for authenticated requests.
//!
//! # Signature Generation
//!
//! Bybit v5 API uses HMAC-SHA256 signatures with the format:
//! `timestamp + api_key + recv_window + payload`
//!
//! # Example
//!
//! ````rust
//! use rusty_bybit::auth::generate_signature;
//!
//! let signature = generate_signature(
//!     1658384314791,
//!     "api_key",
//!     5000,
//!     "category=option&symbol=BTC-29JUL22-25000-C",
//!     "api_secret"
//! );
//! ```

use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub api_key: String,
    pub api_secret: String,
}

impl Credentials {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }
}

pub fn generate_signature(
    timestamp: i64,
    api_key: &str,
    recv_window: u64,
    payload: &str,
    secret: &str,
) -> String {
    let sign_str = format!("{}{}{}{}", timestamp, api_key, recv_window, payload);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("Invalid key length");
    mac.update(sign_str.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

pub fn get_current_timestamp_ms() -> i64 {
    Utc::now().timestamp_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_signature() {
        let timestamp = 1658384314791;
        let api_key = "XXXXXXXXXX";
        let recv_window = 5000;
        let query_string = "category=option&symbol=BTC-29JUL22-25000-C";
        let secret = "test_secret";

        let signature = generate_signature(timestamp, api_key, recv_window, query_string, secret);
        assert!(!signature.is_empty());
        assert_eq!(signature.len(), 64);
    }

    #[test]
    fn test_generate_signature_post() {
        let timestamp = 1658385579423;
        let api_key = "XXXXXXXXXX";
        let recv_window = 5000;
        let body = "{\"category\": \"option\"}";
        let secret = "test_secret";

        let signature = generate_signature(timestamp, api_key, recv_window, body, secret);
        assert!(!signature.is_empty());
        assert_eq!(signature.len(), 64);
    }
}
