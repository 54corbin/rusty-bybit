use rusty_bybit::BybitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Bybit API SDK Example\n");

    let client = BybitClient::testnet();

    let server_time = client.get_server_time().await?;
    println!("Server time: {}", server_time.time_second);

    let tickers = client.get_tickers("linear").await?;
    println!("\nGot {} tickers", tickers.list.len());
    if let Some(ticker) = tickers.list.first() {
        println!(
            "First ticker: {} - Last price: {}",
            ticker.symbol, ticker.last_price
        );
    }

    let orderbook = client.get_orderbook("linear", "BTCUSDT", 10).await?;
    println!("\nOrderbook for BTCUSDT:");
    println!("Best bid: {} @ {}", orderbook.b[0].0, orderbook.b[0].1);
    println!("Best ask: {} @ {}", orderbook.a[0].0, orderbook.a[0].1);

    Ok(())
}

#[allow(dead_code)]
#[tokio::main]
async fn authenticated_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Bybit API SDK - Authenticated Example\n");

    let api_key = std::env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY not set");
    let api_secret = std::env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET not set");

    let client = BybitClient::testnet().with_credentials(api_key, api_secret);

    let balance = client.get_wallet_balance(None).await?;
    if let Some(account) = balance.list.first() {
        println!("Total equity: {}", account.total_equity);
    }

    let positions = client.get_position("linear", None).await?;
    println!("\nOpen positions: {}", positions.list.len());

    Ok(())
}
