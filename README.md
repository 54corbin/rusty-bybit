# Rusty-Bybit SDK

An unofficial Rust SDK for the [Bybit V5 API](https://bybit-exchange.github.io/docs/v5/guide).

## Features

- **Unified Trading Account Support** - Single account type for all trading
- **Type-Safe API** - Strongly-typed enums prevent errors at compile time
- **Full Async/Await** - Built on tokio for non-blocking operations
- **Comprehensive Error Handling** - Detailed error types for better debugging
- **HMAC-SHA256 Authentication** - Secure signature generation for private endpoints

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rusty-bybit = "0.1"
```

## Quick Start

### Public Endpoints

```rust
use rusty_bybit::BybitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BybitClient::testnet();

    let time = client.get_server_time().await?;
    println!("Server time: {}", time.time_second);

    let tickers = client.get_tickers("linear").await?;
    println!("Got {} tickers", tickers.list.len());

    Ok(())
}
```

### Authenticated Endpoints

```rust
use rusty_bybit::BybitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("BYBIT_API_KEY")?;
    let api_secret = std::env::var("BYBIT_API_SECRET")?;

    let client = BybitClient::testnet()
        .with_credentials(api_key, api_secret);

    let balance = client.get_wallet_balance(None).await?;
    if let Some(account) = balance.list.first() {
        println!("Total equity: {}", account.total_equity);
    }

    Ok(())
}
```

### Creating Orders

```rust
use rusty_bybit::{BybitClient, CreateOrderRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BybitClient::testnet()
        .with_credentials("api_key".to_string(), "api_secret".to_string());

    let request = CreateOrderRequest {
        category: "linear".to_string(),
        symbol: "BTCUSDT".to_string(),
        side: "Buy".to_string(),
        order_type: "Limit".to_string(),
        qty: Some("0.001".to_string()),
        price: Some("28000".to_string()),
        time_in_force: Some("GTC".to_string()),
        ..Default::default()
    };

    let response = client.create_order(&request).await?;
    println!("Order ID: {}", response.order_id);

    Ok(())
}
```

## API Endpoints

### Market Data

- `get_server_time()` - Get Bybit server time
- `get_tickers(category)` - Get tickers for a market category
- `get_orderbook(category, symbol, limit)` - Get orderbook
- `get_instruments(category)` - Get instrument info
- `get_kline(category, symbol, interval)` - Get kline data

### Trading

- `create_order(request)` - Create a new order
- `cancel_order(category, order_id, symbol)` - Cancel a specific order
- `cancel_all_orders(category, symbol)` - Cancel all orders for a symbol
- `get_order(category, order_id)` - Get order details
- `get_open_orders(category)` - Get all open orders

### Account

- `get_wallet_balance(account_type)` - Get wallet balance
- `get_position(category, symbol)` - Get position info
- `set_leverage(category, symbol, buy_leverage, sell_leverage)` - Set leverage
- `get_execution_list(category, symbol)` - Get execution history
- `get_closed_pnl(category, symbol)` - Get closed PnL

## Environment

### Testnet

```rust
let client = BybitClient::testnet();
```

### Mainnet

```rust
let client = BybitClient::mainnet();
```

### Custom URL

```rust
let client = BybitClient::new("https://api.bybit.com".to_string());
```

## API Reference

See the [crate documentation](https://docs.rs/rusty-bybit) for detailed API reference.

## Examples

See the `examples/` directory for more usage examples:

- `basic_usage.rs` - Basic market data and authenticated requests
- `market_data.rs` - Market data endpoints
- `order_management.rs` - Order creation and management
- `account_management.rs` - Wallet and position management

## Error Handling

The SDK provides comprehensive error types via `BybitError`:

```rust
use rusty_bybit::{BybitClient, BybitError};

#[tokio::main]
async fn main() {
    let client = BybitClient::testnet();

    match client.get_server_time().await {
        Ok(time) => println!("Time: {}", time.time_second),
        Err(BybitError::ApiError { ret_code, ret_msg }) => {
            eprintln!("API error {}: {}", ret_code, ret_msg);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Breaking Changes

See [CHANGELOG.md](CHANGELOG.md) for version history and breaking changes.

## Bybit API Documentation

- [Official Documentation](https://bybit-exchange.github.io/docs/v5/guide)
- [API Reference](https://bybit-exchange.github.io/docs/v5/api-explorer/v5/category)

## Contributing

Contributions are welcome! Please ensure:

- Code passes `cargo clippy` and `cargo fmt`
- All tests pass: `cargo test`
- Documentation is updated for public API changes

## License

MIT License
