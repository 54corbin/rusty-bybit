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
    ) -> Result<serde_json::Value> {
        let query = vec![
            ("category", category),
            ("symbol", symbol),
            ("interval", interval),
        ];
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
    fn test_market_module_exists() {}
}
