use rusty_bybit::BybitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Bybit Account Management Examples\n");

    let api_key = std::env::var("BYBIT_API_KEY").unwrap_or_else(|_| {
        eprintln!("Warning: BYBIT_API_KEY not set, using placeholder");
        "your_api_key".to_string()
    });

    let api_secret = std::env::var("BYBIT_API_SECRET").unwrap_or_else(|_| {
        eprintln!("Warning: BYBIT_API_SECRET not set, using placeholder");
        "your_api_secret".to_string()
    });

    let client = BybitClient::testnet().with_credentials(api_key, api_secret);

    println!("1. Getting wallet balance...");
    match client.get_wallet_balance(None).await {
        Ok(balance) => {
            println!("   Wallet balance retrieved successfully!");
            for account in balance.list.iter() {
                println!("\n   Account Type: {}", account.account_type);
                println!("   Total Equity: {}", account.total_equity);
                println!("   Total Wallet Balance: {}", account.total_wallet_balance);
                println!("   Total Margin Balance: {}", account.total_margin_balance);
                println!(
                    "   Total Available Balance: {}",
                    account.total_available_balance
                );
                println!("   Total Unrealized PnL: {}", account.total_perp_upl);
                println!("   Total Initial Margin: {}", account.total_initial_margin);
                println!(
                    "   Total Maintenance Margin: {}",
                    account.total_maintenance_margin
                );

                println!("\n   Coin Balances:");
                for coin in &account.coin {
                    println!("     {}: {}", coin.coin, coin.wallet_balance);
                }
            }
        }
        Err(e) => println!("   Error getting wallet balance: {}", e),
    }

    println!("\n2. Getting positions for linear market...");
    match client.get_position("linear", None).await {
        Ok(positions) => {
            println!("   Total positions: {}", positions.list.len());
            for position in positions.list.iter() {
                println!("\n   Position Details:");
                println!("     Symbol: {}", position.symbol);
                println!("     Side: {}", position.side);
                println!("     Size: {}", position.size);
                println!("     Position Value: {}", position.position_value);
                println!("     Unrealized PnL: {}", position.unrealised_pnl);
                println!("     Status: {}", position.position_status);
            }
        }
        Err(e) => println!("   Error getting positions: {}", e),
    }

    println!("\n3. Getting specific position for BTCUSDT...");
    match client.get_position("linear", Some("BTCUSDT")).await {
        Ok(positions) => {
            if !positions.list.is_empty() {
                let position = &positions.list[0];
                println!("   Position found:");
                println!("     Symbol: {}", position.symbol);
                println!("     Side: {}", position.side);
                println!("     Size: {}", position.size);
                println!("     Unrealized PnL: {}", position.unrealised_pnl);
            } else {
                println!("   No position found for BTCUSDT");
            }
        }
        Err(e) => println!("   Error getting position: {}", e),
    }

    println!("\n4. Setting leverage for BTCUSDT...");
    match client.set_leverage("linear", "BTCUSDT", "10", "10").await {
        Ok(_) => println!("   Leverage set successfully!"),
        Err(e) => println!("   Error setting leverage: {}", e),
    }

    println!("\n5. Getting execution list...");
    match client.get_execution_list("linear", None).await {
        Ok(executions) => {
            if let Some(list) = executions.get("list").and_then(|v| v.as_array()) {
                println!("   Total executions: {}", list.len());
                for exec in list.iter().take(3) {
                    if let Some(obj) = exec.as_object() {
                        if let (Some(order_id), Some(symbol), Some(side), Some(exec_qty)) = (
                            obj.get("orderId").and_then(|v| v.as_str()),
                            obj.get("symbol").and_then(|v| v.as_str()),
                            obj.get("side").and_then(|v| v.as_str()),
                            obj.get("execQty").and_then(|v| v.as_str()),
                        ) {
                            println!(
                                "     Order: {} - {} {} @ qty: {}",
                                order_id, side, symbol, exec_qty
                            );
                        }
                    }
                }
            }
        }
        Err(e) => println!("   Error getting execution list: {}", e),
    }

    println!("\n6. Getting execution list for BTCUSDT...");
    match client.get_execution_list("linear", Some("BTCUSDT")).await {
        Ok(executions) => {
            if let Some(list) = executions.get("list").and_then(|v| v.as_array()) {
                println!("   BTCUSDT executions: {}", list.len());
            }
        }
        Err(e) => println!("   Error getting execution list: {}", e),
    }

    println!("\n7. Getting closed PnL...");
    match client.get_closed_pnl("linear", None).await {
        Ok(closed_pnl) => {
            if let Some(list) = closed_pnl.get("list").and_then(|v| v.as_array()) {
                println!("   Total closed PnL records: {}", list.len());
                for pnl in list.iter().take(3) {
                    if let Some(obj) = pnl.as_object() {
                        if let (Some(symbol), Some(side), Some(closed_pnl_value)) = (
                            obj.get("symbol").and_then(|v| v.as_str()),
                            obj.get("side").and_then(|v| v.as_str()),
                            obj.get("closedPnl").and_then(|v| v.as_str()),
                        ) {
                            println!("     {} {} - PnL: {}", side, symbol, closed_pnl_value);
                        }
                    }
                }
            }
        }
        Err(e) => println!("   Error getting closed PnL: {}", e),
    }

    println!("\n8. Getting closed PnL for BTCUSDT...");
    match client.get_closed_pnl("linear", Some("BTCUSDT")).await {
        Ok(closed_pnl) => {
            if let Some(list) = closed_pnl.get("list").and_then(|v| v.as_array()) {
                println!("   BTCUSDT closed PnL records: {}", list.len());
                if let Some(first) = list.first() {
                    if let Some(obj) = first.as_object() {
                        if let Some(closed_pnl_value) =
                            obj.get("closedPnl").and_then(|v| v.as_str())
                        {
                            println!("   Latest closed PnL: {}", closed_pnl_value);
                        }
                    }
                }
            }
        }
        Err(e) => println!("   Error getting closed PnL: {}", e),
    }

    println!("\nAccount management examples completed!");

    Ok(())
}
