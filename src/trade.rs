//! Trading endpoints for order management
//!
//! Provides functionality for creating, canceling, and querying orders.
//!
//! # Example
//!
//! ````rust,no_run
//! use rusty_bybit::BybitClient;
//! use rusty_bybit::CreateOrderRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = BybitClient::testnet();
//!     let request = CreateOrderRequest {
//!         category: "linear".to_string(),
//!         symbol: "BTCUSDT".to_string(),
//!         side: "Buy".to_string(),
//!         order_type: "Limit".to_string(),
//!         qty: Some("0.001".to_string()),
//!         price: Some("28000".to_string()),
//!         ..Default::default()
//!     };
//!     let response = client.create_order(&request).await.unwrap();
//!     println!("Order ID: {}", response.order_id);
//! }
//! ```

use crate::client::BybitClient;
use crate::error::Result;
use crate::types::{CreateOrderRequest, CreateOrderResponse, OrderList};

impl BybitClient {
    pub async fn create_order(&self, request: &CreateOrderRequest) -> Result<CreateOrderResponse> {
        let body = serde_json::to_value(request)?;
        self.post("/v5/order/create", Some(body)).await
    }

    pub async fn cancel_order(
        &self,
        category: &str,
        order_id: &str,
        symbol: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "category": category,
            "orderId": order_id,
            "symbol": symbol,
        });
        self.post("/v5/order/cancel", Some(body)).await
    }

    pub async fn cancel_all_orders(
        &self,
        category: &str,
        symbol: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "category": category,
            "symbol": symbol,
        });
        self.post("/v5/order/cancel-all", Some(body)).await
    }

    pub async fn get_order(&self, category: &str, order_id: &str) -> Result<OrderList> {
        let query = vec![("category", category), ("orderId", order_id)];
        self.get("/v5/order/realtime", Some(query)).await
    }

    pub async fn get_open_orders(&self, category: &str) -> Result<OrderList> {
        let query = vec![("category", category)];
        self.get("/v5/order/realtime", Some(query)).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trade_module_exists() {}
}
