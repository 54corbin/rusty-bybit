//! Common data structures for Bybit v5 API
//!
//! Defines all request/response types used throughout the SDK.
//!
//! # Response Wrappers
//!
//! Most API responses use wrapper objects containing a `list` field:
//! - `TickerList` - wraps `Vec<Ticker>`
//! - `InstrumentList` - wraps `Vec<InstrumentInfo>`
//! - `PositionList` - wraps `Vec<Position>`
//! - `OrderList` - wraps `Vec<Order>`
//! - `WalletBalance` - wraps `Vec<AccountBalance>`

use serde::{Deserialize, Serialize};

/// Bybit server time response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTime {
    #[serde(rename = "timeSecond")]
    pub time_second: String,
    #[serde(rename = "timeNano")]
    pub time_nano: String,
}

/// Empty result for API calls that don't return data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResult;

/// Product category for Bybit API endpoints
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Category {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "inverse")]
    Inverse,
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "option")]
    Option,
}

/// Bybit API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: T,
    #[serde(rename = "retExtInfo", default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub b: Vec<(String, String)>,
    pub a: Vec<(String, String)>,
    pub ts: i64,
    pub u: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentInfo {
    pub symbol: String,
    #[serde(rename = "contractType")]
    pub contract_type: String,
    pub status: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    #[serde(rename = "settleCoin")]
    pub settle_coin: String,
    #[serde(rename = "priceScale")]
    pub price_scale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "indexPrice")]
    pub index_price: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "bid1Price")]
    pub bid1_price: String,
    #[serde(rename = "bid1Size")]
    pub bid1_size: String,
    #[serde(rename = "ask1Price")]
    pub ask1_price: String,
    #[serde(rename = "ask1Size")]
    pub ask1_size: String,
}

/// Wrapper for ticker list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerList {
    pub list: Vec<Ticker>,
    pub next_page_cursor: Option<String>,
}

/// Wrapper for instrument list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentList {
    pub list: Vec<InstrumentInfo>,
    pub next_page_cursor: Option<String>,
}

/// Wrapper for wallet balance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    pub list: Vec<AccountBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: String,
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: String,
    #[serde(rename = "totalEquity")]
    pub total_equity: String,
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    #[serde(rename = "totalAvailableBalance")]
    pub total_available_balance: String,
    #[serde(rename = "totalPerpUPL")]
    pub total_perp_upl: String,
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: String,
    #[serde(rename = "totalMaintenanceMargin")]
    pub total_maintenance_margin: String,
    pub coin: Vec<CoinBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinBalance {
    pub coin: String,
    pub wallet_balance: String,
    #[serde(rename = "transferBalance")]
    pub transfer_balance: String,
}

/// Wrapper for position list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionList {
    pub list: Vec<Position>,
    pub category: String,
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    #[serde(rename = "positionIdx")]
    pub position_idx: u64,
    #[serde(rename = "positionStatus")]
    pub position_status: String,
    pub side: String,
    pub size: String,
    #[serde(rename = "positionValue")]
    pub position_value: String,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,
}

/// Order side: Buy or Sell
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Side {
    #[serde(rename = "Buy")]
    Buy,
    #[serde(rename = "Sell")]
    Sell,
}

/// Order type: Market or Limit
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    #[serde(rename = "Market")]
    Market,
    #[serde(rename = "Limit")]
    Limit,
}

/// Time in force strategy for orders
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    GTC,
    #[serde(rename = "IOC")]
    IOC,
    #[serde(rename = "FOK")]
    FOK,
    #[serde(rename = "PostOnly")]
    PostOnly,
    #[serde(rename = "RPI")]
    RPI,
}

/// Order status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    #[serde(rename = "New")]
    New,
    #[serde(rename = "PartiallyFilled")]
    PartiallyFilled,
    #[serde(rename = "Filled")]
    Filled,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Rejected")]
    Rejected,
}

/// Wrapper for order list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderList {
    pub list: Vec<Order>,
    pub next_page_cursor: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub order_link_id: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub price: String,
    pub qty: String,
    pub time_in_force: String,
    pub create_type: String,
    pub cancel_type: String,
    pub status: String,
    pub leaves_qty: String,
    pub cum_exec_qty: String,
    pub avg_price: String,
    pub created_time: String,
    pub updated_time: String,
    #[serde(rename = "positionIdx")]
    pub position_idx: u64,
    #[serde(rename = "triggerPrice")]
    pub trigger_price: Option<String>,
    #[serde(rename = "takeProfit")]
    pub take_profit: Option<String>,
    #[serde(rename = "stopLoss")]
    pub stop_loss: Option<String>,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "closeOnTrigger")]
    pub close_on_trigger: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub category: String,
    pub symbol: String,
    pub side: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "positionIdx", skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<u64>,
    #[serde(rename = "orderLinkId", skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(rename = "triggerPrice", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "takeProfit", skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(rename = "stopLoss", skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "closeOnTrigger", skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
}

impl CreateOrderRequest {
    pub fn builder() -> CreateOrderRequestBuilder {
        CreateOrderRequestBuilder::default()
    }
}

/// Builder for CreateOrderRequest with fluent API
#[derive(Debug, Default)]
pub struct CreateOrderRequestBuilder {
    category: Option<String>,
    symbol: Option<String>,
    side: Option<String>,
    order_type: Option<String>,
    qty: Option<String>,
    price: Option<String>,
    time_in_force: Option<String>,
    position_idx: Option<u64>,
    order_link_id: Option<String>,
    trigger_price: Option<String>,
    take_profit: Option<String>,
    stop_loss: Option<String>,
    reduce_only: Option<bool>,
    close_on_trigger: Option<bool>,
    trigger_by: Option<String>,
    tp_trigger_by: Option<String>,
    sl_trigger_by: Option<String>,
    market_unit: Option<String>,
    slippage_tolerance_type: Option<String>,
    slippage_tolerance: Option<String>,
    trigger_direction: Option<i32>,
    order_filter: Option<String>,
}

impl CreateOrderRequestBuilder {
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn side(mut self, side: impl Into<String>) -> Self {
        self.side = Some(side.into());
        self
    }

    pub fn order_type(mut self, order_type: impl Into<String>) -> Self {
        self.order_type = Some(order_type.into());
        self
    }

    pub fn qty(mut self, qty: impl Into<String>) -> Self {
        self.qty = Some(qty.into());
        self
    }

    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn time_in_force(mut self, time_in_force: impl Into<String>) -> Self {
        self.time_in_force = Some(time_in_force.into());
        self
    }

    pub fn position_idx(mut self, position_idx: u64) -> Self {
        self.position_idx = Some(position_idx);
        self
    }

    pub fn order_link_id(mut self, order_link_id: impl Into<String>) -> Self {
        self.order_link_id = Some(order_link_id.into());
        self
    }

    pub fn trigger_price(mut self, trigger_price: impl Into<String>) -> Self {
        self.trigger_price = Some(trigger_price.into());
        self
    }

    pub fn take_profit(mut self, take_profit: impl Into<String>) -> Self {
        self.take_profit = Some(take_profit.into());
        self
    }

    pub fn stop_loss(mut self, stop_loss: impl Into<String>) -> Self {
        self.stop_loss = Some(stop_loss.into());
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn close_on_trigger(mut self, close_on_trigger: bool) -> Self {
        self.close_on_trigger = Some(close_on_trigger);
        self
    }

    pub fn trigger_by(mut self, trigger_by: impl Into<String>) -> Self {
        self.trigger_by = Some(trigger_by.into());
        self
    }

    pub fn tp_trigger_by(mut self, tp_trigger_by: impl Into<String>) -> Self {
        self.tp_trigger_by = Some(tp_trigger_by.into());
        self
    }

    pub fn sl_trigger_by(mut self, sl_trigger_by: impl Into<String>) -> Self {
        self.sl_trigger_by = Some(sl_trigger_by.into());
        self
    }

    pub fn market_unit(mut self, market_unit: impl Into<String>) -> Self {
        self.market_unit = Some(market_unit.into());
        self
    }

    pub fn slippage_tolerance_type(mut self, slippage_tolerance_type: impl Into<String>) -> Self {
        self.slippage_tolerance_type = Some(slippage_tolerance_type.into());
        self
    }

    pub fn slippage_tolerance(mut self, slippage_tolerance: impl Into<String>) -> Self {
        self.slippage_tolerance = Some(slippage_tolerance.into());
        self
    }

    pub fn trigger_direction(mut self, trigger_direction: i32) -> Self {
        self.trigger_direction = Some(trigger_direction);
        self
    }

    pub fn order_filter(mut self, order_filter: impl Into<String>) -> Self {
        self.order_filter = Some(order_filter.into());
        self
    }

    pub fn build(self) -> CreateOrderRequest {
        CreateOrderRequest {
            category: self.category.unwrap_or_else(|| "linear".to_string()),
            symbol: self.symbol.expect("symbol is required"),
            side: self.side.expect("side is required"),
            order_type: self.order_type.expect("order_type is required"),
            qty: self.qty,
            price: self.price,
            time_in_force: self.time_in_force,
            position_idx: self.position_idx,
            order_link_id: self.order_link_id,
            trigger_price: self.trigger_price,
            take_profit: self.take_profit,
            stop_loss: self.stop_loss,
            reduce_only: self.reduce_only,
            close_on_trigger: self.close_on_trigger,
            trigger_by: self.trigger_by,
            tp_trigger_by: self.tp_trigger_by,
            sl_trigger_by: self.sl_trigger_by,
            market_unit: self.market_unit,
            slippage_tolerance_type: self.slippage_tolerance_type,
            slippage_tolerance: self.slippage_tolerance,
            trigger_direction: self.trigger_direction,
            order_filter: self.order_filter,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub order_link_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_serialization() {
        let linear_json = serde_json::to_string(&Category::Linear).unwrap();
        assert_eq!(linear_json, r#""linear""#);

        let inverse_json = serde_json::to_string(&Category::Inverse).unwrap();
        assert_eq!(inverse_json, r#""inverse""#);

        let spot_json = serde_json::to_string(&Category::Spot).unwrap();
        assert_eq!(spot_json, r#""spot""#);
    }

    #[test]
    fn test_category_deserialization() {
        let linear: Category = serde_json::from_str(r#""linear""#).unwrap();
        assert_eq!(linear, Category::Linear);

        let inverse: Category = serde_json::from_str(r#""inverse""#).unwrap();
        assert_eq!(inverse, Category::Inverse);

        let spot: Category = serde_json::from_str(r#""spot""#).unwrap();
        assert_eq!(spot, Category::Spot);
    }

    #[test]
    fn test_side_serialization() {
        let buy_json = serde_json::to_string(&Side::Buy).unwrap();
        assert_eq!(buy_json, r#""Buy""#);

        let sell_json = serde_json::to_string(&Side::Sell).unwrap();
        assert_eq!(sell_json, r#""Sell""#);
    }

    #[test]
    fn test_side_deserialization() {
        let buy: Side = serde_json::from_str(r#""Buy""#).unwrap();
        assert_eq!(buy, Side::Buy);

        let sell: Side = serde_json::from_str(r#""Sell""#).unwrap();
        assert_eq!(sell, Side::Sell);
    }

    #[test]
    fn test_order_type_serialization() {
        let market_json = serde_json::to_string(&OrderType::Market).unwrap();
        assert_eq!(market_json, r#""Market""#);

        let limit_json = serde_json::to_string(&OrderType::Limit).unwrap();
        assert_eq!(limit_json, r#""Limit""#);
    }

    #[test]
    fn test_order_type_deserialization() {
        let market: OrderType = serde_json::from_str(r#""Market""#).unwrap();
        assert_eq!(market, OrderType::Market);

        let limit: OrderType = serde_json::from_str(r#""Limit""#).unwrap();
        assert_eq!(limit, OrderType::Limit);
    }

    #[test]
    fn test_time_in_force_serialization() {
        let gtc_json = serde_json::to_string(&TimeInForce::GTC).unwrap();
        assert_eq!(gtc_json, r#""GTC""#);

        let ioc_json = serde_json::to_string(&TimeInForce::IOC).unwrap();
        assert_eq!(ioc_json, r#""IOC""#);

        let fok_json = serde_json::to_string(&TimeInForce::FOK).unwrap();
        assert_eq!(fok_json, r#""FOK""#);
    }

    #[test]
    fn test_order_status_serialization() {
        let new_json = serde_json::to_string(&OrderStatus::New).unwrap();
        assert_eq!(new_json, r#""New""#);

        let filled_json = serde_json::to_string(&OrderStatus::Filled).unwrap();
        assert_eq!(filled_json, r#""Filled""#);

        let cancelled_json = serde_json::to_string(&OrderStatus::Cancelled).unwrap();
        assert_eq!(cancelled_json, r#""Cancelled""#);
    }

    #[test]
    fn test_server_time_serialization() {
        let time = ServerTime {
            time_second: "1234567890".to_string(),
            time_nano: "1234567890123456789".to_string(),
        };

        let json = serde_json::to_string(&time).unwrap();
        assert!(json.contains("\"timeSecond\":\"1234567890\""));
        assert!(json.contains("\"timeNano\":\"1234567890123456789\""));
    }

    #[test]
    fn test_server_time_deserialization() {
        let json = r#"{"timeSecond":"1234567890","timeNano":"1234567890123456789"}"#;
        let time: ServerTime = serde_json::from_str(json).unwrap();
        assert_eq!(time.time_second, "1234567890");
        assert_eq!(time.time_nano, "1234567890123456789");
    }

    #[test]
    fn test_ticker_list_serialization() {
        let ticker_list = TickerList {
            list: vec![],
            next_page_cursor: None,
        };

        let json = serde_json::to_string(&ticker_list).unwrap();
        assert!(json.contains("\"list\":[]"));
    }

    #[test]
    fn test_create_order_request_default() {
        let request = CreateOrderRequest {
            category: "linear".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "Buy".to_string(),
            order_type: "Limit".to_string(),
            ..Default::default()
        };

        assert_eq!(request.category, "linear");
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, "Buy");
        assert_eq!(request.order_type, "Limit");
        assert!(request.qty.is_none());
        assert!(request.price.is_none());
    }

    #[test]
    fn test_create_order_request_with_all_fields() {
        let request = CreateOrderRequest {
            category: "linear".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "Buy".to_string(),
            order_type: "Limit".to_string(),
            qty: Some("0.001".to_string()),
            price: Some("28000".to_string()),
            time_in_force: Some("GTC".to_string()),
            reduce_only: Some(false),
            take_profit: Some("30000".to_string()),
            stop_loss: Some("27000".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"qty\":\"0.001\""));
        assert!(json.contains("\"price\":\"28000\""));
        assert!(json.contains("\"reduceOnly\":false"));
    }

    #[test]
    fn test_create_order_request_builder_basic() {
        let request = CreateOrderRequest::builder()
            .category("linear")
            .symbol("BTCUSDT")
            .side("Buy")
            .order_type("Limit")
            .build();

        assert_eq!(request.category, "linear");
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, "Buy");
        assert_eq!(request.order_type, "Limit");
    }

    #[test]
    #[should_panic(expected = "symbol is required")]
    fn test_create_order_request_builder_missing_symbol() {
        let _ = CreateOrderRequest::builder()
            .category("linear")
            .side("Buy")
            .order_type("Limit")
            .build();
    }

    #[test]
    #[should_panic(expected = "side is required")]
    fn test_create_order_request_builder_missing_side() {
        let _ = CreateOrderRequest::builder()
            .category("linear")
            .symbol("BTCUSDT")
            .order_type("Limit")
            .build();
    }

    #[test]
    #[should_panic(expected = "order_type is required")]
    fn test_create_order_request_builder_missing_order_type() {
        let _ = CreateOrderRequest::builder()
            .category("linear")
            .symbol("BTCUSDT")
            .side("Buy")
            .build();
    }

    #[test]
    fn test_create_order_request_builder_with_optional_fields() {
        let request = CreateOrderRequest::builder()
            .category("linear")
            .symbol("BTCUSDT")
            .side("Buy")
            .order_type("Limit")
            .qty("0.001")
            .price("28000")
            .time_in_force("GTC")
            .position_idx(1)
            .order_link_id("my_order")
            .take_profit("30000")
            .stop_loss("27000")
            .reduce_only(false)
            .close_on_trigger(false)
            .build();

        assert_eq!(request.qty, Some("0.001".to_string()));
        assert_eq!(request.price, Some("28000".to_string()));
        assert_eq!(request.time_in_force, Some("GTC".to_string()));
        assert_eq!(request.position_idx, Some(1));
        assert_eq!(request.order_link_id, Some("my_order".to_string()));
        assert_eq!(request.take_profit, Some("30000".to_string()));
        assert_eq!(request.stop_loss, Some("27000".to_string()));
        assert_eq!(request.reduce_only, Some(false));
        assert_eq!(request.close_on_trigger, Some(false));
    }

    #[test]
    fn test_create_order_request_builder_default_category() {
        let request = CreateOrderRequest::builder()
            .symbol("BTCUSDT")
            .side("Buy")
            .order_type("Limit")
            .build();

        assert_eq!(request.category, "linear");
    }

    #[test]
    fn test_create_order_request_builder_chaining() {
        let request = CreateOrderRequest::builder()
            .symbol("BTCUSDT")
            .side("Buy")
            .order_type("Limit")
            .qty("0.001")
            .price("28000")
            .time_in_force("GTC")
            .build();

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, "Buy");
        assert_eq!(request.order_type, "Limit");
        assert_eq!(request.qty, Some("0.001".to_string()));
        assert_eq!(request.price, Some("28000".to_string()));
        assert_eq!(request.time_in_force, Some("GTC".to_string()));
    }

    #[test]
    fn test_create_order_request_optional_fields_skipped_in_json() {
        let request = CreateOrderRequest::builder()
            .symbol("BTCUSDT")
            .side("Buy")
            .order_type("Market")
            .build();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"side\":\"Buy\""));
        assert!(json.contains("\"orderType\":\"Market\""));
        assert!(!json.contains("\"price\""));
        assert!(!json.contains("\"qty\""));
    }
}
