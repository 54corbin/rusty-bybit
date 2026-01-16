use rusty_bybit::BybitClient;

#[tokio::test]
async fn test_get_server_time() {
    let client = BybitClient::testnet();
    let time = client.get_server_time().await.unwrap();
    assert!(!time.time_second.is_empty());
    assert!(!time.time_nano.is_empty());
}

#[tokio::test]
async fn test_get_tickers() {
    let client = BybitClient::testnet();
    let tickers = client.get_tickers("linear").await.unwrap();
    assert!(!tickers.list.is_empty());
}

#[tokio::test]
async fn test_get_orderbook() {
    let client = BybitClient::testnet();
    let orderbook = client.get_orderbook("linear", "BTCUSDT", 5).await.unwrap();
    assert!(!orderbook.b.is_empty());
    assert!(!orderbook.a.is_empty());
}

#[tokio::test]
async fn test_get_instruments() {
    let client = BybitClient::testnet();
    let instruments = client.get_instruments("linear").await.unwrap();
    assert!(!instruments.list.is_empty());
}

#[tokio::test]
async fn test_get_kline() {
    let client = BybitClient::testnet();
    let klines = client.get_kline("linear", "BTCUSDT", "15").await.unwrap();
    assert!(klines.is_object() || klines.is_array());
}
