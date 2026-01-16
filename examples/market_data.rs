use rusty_bybit::BybitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Bybit Market Data Examples\n");

    let client = BybitClient::testnet();

    println!("1. Getting server time...");
    let server_time = client.get_server_time().await?;
    println!("   Server time: {} seconds", server_time.time_second);
    println!("   Nano time: {}", server_time.time_nano);

    println!("\n2. Getting tickers for linear market...");
    let tickers = client.get_tickers("linear").await?;
    println!("   Total tickers: {}", tickers.list.len());
    if let Some(ticker) = tickers.list.first() {
        println!("   First ticker:");
        println!("     Symbol: {}", ticker.symbol);
        println!("     Last price: {}", ticker.last_price);
        println!("     Bid: {} @ {}", ticker.bid1_price, ticker.bid1_size);
        println!("     Ask: {} @ {}", ticker.ask1_price, ticker.ask1_size);
    }

    println!("\n3. Getting orderbook for BTCUSDT...");
    let orderbook = client.get_orderbook("linear", "BTCUSDT", 10).await?;
    println!("   Orderbook for BTCUSDT:");
    if !orderbook.b.is_empty() {
        println!("     Best bid: {} @ {}", orderbook.b[0].0, orderbook.b[0].1);
    }
    if !orderbook.a.is_empty() {
        println!("     Best ask: {} @ {}", orderbook.a[0].0, orderbook.a[0].1);
    }
    println!("   Total bids: {}", orderbook.b.len());
    println!("   Total asks: {}", orderbook.a.len());

    println!("\n4. Getting instrument info for linear market...");
    let instruments = client.get_instruments("linear").await?;
    println!("   Total instruments: {}", instruments.list.len());
    if let Some(instrument) = instruments.list.first() {
        println!("   First instrument:");
        println!("     Symbol: {}", instrument.symbol);
        println!("     Contract type: {}", instrument.contract_type);
        println!("     Status: {}", instrument.status);
        println!("     Base coin: {}", instrument.base_coin);
        println!("     Quote coin: {}", instrument.quote_coin);
    }

    println!("\n5. Getting kline data for BTCUSDT (15 min interval)...");
    let klines = client.get_kline("linear", "BTCUSDT", "15").await?;
    if let Some(list) = klines.get("list").and_then(|v| v.as_array()) {
        println!("   Total klines: {}", list.len());
        if let Some(first_kline) = list.first().and_then(|v| v.as_array())
            && first_kline.len() >= 6
        {
            println!("   Latest kline:");
            println!("     Timestamp: {}", first_kline[0]);
            println!("     Open: {}", first_kline[1]);
            println!("     High: {}", first_kline[2]);
            println!("     Low: {}", first_kline[3]);
            println!("     Close: {}", first_kline[4]);
            println!("     Volume: {}", first_kline[5]);
        }
    }

    println!("\n6. Getting tickers for inverse market...");
    let inverse_tickers = client.get_tickers("inverse").await?;
    println!("   Total inverse tickers: {}", inverse_tickers.list.len());

    println!("\n7. Getting tickers for spot market...");
    let spot_tickers = client.get_tickers("spot").await?;
    println!("   Total spot tickers: {}", spot_tickers.list.len());

    println!("\nMarket data examples completed successfully!");

    Ok(())
}
