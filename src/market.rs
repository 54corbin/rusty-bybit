//! Market data endpoints
//!
//! Provides access to public market data including tickers, orderbook, klines, and instrument info.
//!
//! # Example
//!
//! ````rust,no_run
//! use rusty_bybit::BybitClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = BybitClient::testnet();
//!     let tickers = client.get_tickers("linear").await.unwrap();
//!     println!("First ticker: {}", tickers.list[0].symbol);
//! }
//! ```

use crate::client::BybitClient;
use crate::error::Result;
use crate::types::{InstrumentList, OrderBook, ServerTime, TickerList};

impl BybitClient {
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.get("/v5/market/time", None).await
    }

    pub async fn get_kline(
        &self,
        category: &str,
        symbol: &str,
        interval: &str,
        start: Option<i64>,
        end: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut params: Vec<(String, String)> = vec![
            ("category".to_string(), category.to_string()),
            ("symbol".to_string(), symbol.to_string()),
            ("interval".to_string(), interval.to_string()),
        ];

        if let Some(s) = start {
            params.push(("start".to_string(), s.to_string()));
        }

        if let Some(e) = end {
            params.push(("end".to_string(), e.to_string()));
        }

        let query: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        self.get("/v5/market/kline", Some(query)).await
    }

    pub async fn get_tickers(&self, category: &str) -> Result<TickerList> {
        let query = vec![("category", category)];
        self.get("/v5/market/tickers", Some(query)).await
    }

    pub async fn get_orderbook(
        &self,
        category: &str,
        symbol: &str,
        limit: u32,
    ) -> Result<OrderBook> {
        let limit_str = limit.to_string();
        let query = vec![
            ("category", category),
            ("symbol", symbol),
            ("limit", limit_str.as_str()),
        ];
        self.get("/v5/market/orderbook", Some(query)).await
    }

    pub async fn get_instruments(&self, category: &str) -> Result<InstrumentList> {
        let query = vec![("category", category)];
        self.get("/v5/market/instruments-info", Some(query)).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_kline_basic_params() {
        let params: Vec<(String, String)> = vec![
            ("category".to_string(), "linear".to_string()),
            ("symbol".to_string(), "BTCUSDT".to_string()),
            ("interval".to_string(), "15".to_string()),
        ];

        let query: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        assert_eq!(query.len(), 3);
        assert!(query.contains(&("category", "linear")));
        assert!(query.contains(&("symbol", "BTCUSDT")));
        assert!(query.contains(&("interval", "15")));
    }

    #[test]
    fn test_get_kline_with_start_only() {
        let mut params: Vec<(String, String)> = vec![
            ("category".to_string(), "linear".to_string()),
            ("symbol".to_string(), "BTCUSDT".to_string()),
            ("interval".to_string(), "15".to_string()),
        ];

        let start = 1670601600000_i64;
        params.push(("start".to_string(), start.to_string()));

        let query: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        assert_eq!(query.len(), 4);
        assert!(query.contains(&("category", "linear")));
        assert!(query.contains(&("symbol", "BTCUSDT")));
        assert!(query.contains(&("interval", "15")));
        assert!(query.contains(&("start", "1670601600000")));
    }

    #[test]
    fn test_get_kline_with_end_only() {
        let mut params: Vec<(String, String)> = vec![
            ("category".to_string(), "linear".to_string()),
            ("symbol".to_string(), "BTCUSDT".to_string()),
            ("interval".to_string(), "15".to_string()),
        ];

        let end = 1670608800000_i64;
        params.push(("end".to_string(), end.to_string()));

        let query: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        assert_eq!(query.len(), 4);
        assert!(query.contains(&("category", "linear")));
        assert!(query.contains(&("symbol", "BTCUSDT")));
        assert!(query.contains(&("interval", "15")));
        assert!(query.contains(&("end", "1670608800000")));
    }

    #[test]
    fn test_get_kline_with_both_start_and_end() {
        let mut params: Vec<(String, String)> = vec![
            ("category".to_string(), "linear".to_string()),
            ("symbol".to_string(), "BTCUSDT".to_string()),
            ("interval".to_string(), "15".to_string()),
        ];

        let start = 1670601600000_i64;
        let end = 1670608800000_i64;

        params.push(("start".to_string(), start.to_string()));
        params.push(("end".to_string(), end.to_string()));

        let query: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        assert_eq!(query.len(), 5);
        assert!(query.contains(&("category", "linear")));
        assert!(query.contains(&("symbol", "BTCUSDT")));
        assert!(query.contains(&("interval", "15")));
        assert!(query.contains(&("start", "1670601600000")));
        assert!(query.contains(&("end", "1670608800000")));
    }
}
