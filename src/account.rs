//! Account management endpoints
//!
//! Provides access to wallet balance, positions, leverage settings, and execution history.
//!
//! # Example
//!
//! ````rust,no_run
//! use rusty_bybit::BybitClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = BybitClient::testnet()
//!         .with_credentials("api_key".to_string(), "api_secret".to_string());
//!     let balance = client.get_wallet_balance(None).await.unwrap();
//!     if let Some(account) = balance.list.first() {
//!         println!("Total equity: {}", account.total_equity);
//!     }
//! }
//! ```

use crate::client::BybitClient;
use crate::error::Result;
use crate::types::{PositionList, WalletBalance};

impl BybitClient {
    pub async fn get_wallet_balance(&self, account_type: Option<&str>) -> Result<WalletBalance> {
        let query = account_type.map(|t| vec![("accountType", t)]);
        self.get("/v5/account/wallet-balance", query).await
    }

    pub async fn get_position(&self, category: &str, symbol: Option<&str>) -> Result<PositionList> {
        let mut query = vec![("category", category)];
        if let Some(s) = symbol {
            query.push(("symbol", s));
        }
        self.get("/v5/position/list", Some(query)).await
    }

    pub async fn set_leverage(
        &self,
        category: &str,
        symbol: &str,
        buy_leverage: &str,
        sell_leverage: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "category": category,
            "symbol": symbol,
            "buyLeverage": buy_leverage,
            "sellLeverage": sell_leverage,
        });
        self.post("/v5/position/set-leverage", Some(body)).await
    }

    pub async fn get_execution_list(
        &self,
        category: &str,
        symbol: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut query = vec![("category", category)];
        if let Some(s) = symbol {
            query.push(("symbol", s));
        }
        self.get("/v5/execution/list", Some(query)).await
    }

    pub async fn get_closed_pnl(
        &self,
        category: &str,
        symbol: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut query = vec![("category", category)];
        if let Some(s) = symbol {
            query.push(("symbol", s));
        }
        self.get("/v5/position/closed-pnl", Some(query)).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_account_module_exists() {}
}
