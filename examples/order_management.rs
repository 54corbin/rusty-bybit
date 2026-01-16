use rusty_bybit::{BybitClient, CreateOrderRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Bybit Order Management Examples\n");

    let api_key = std::env::var("BYBIT_API_KEY").unwrap_or_else(|_| {
        eprintln!("Warning: BYBIT_API_KEY not set, using placeholder");
        "your_api_key".to_string()
    });

    let api_secret = std::env::var("BYBIT_API_SECRET").unwrap_or_else(|_| {
        eprintln!("Warning: BYBIT_API_SECRET not set, using placeholder");
        "your_api_secret".to_string()
    });

    let client = BybitClient::testnet().with_credentials(api_key, api_secret);

    println!("1. Creating a limit order...");
    let limit_order = CreateOrderRequest::builder()
        .category("linear")
        .symbol("BTCUSDT")
        .side("Buy")
        .order_type("Limit")
        .qty("0.001")
        .price("28000")
        .time_in_force("GTC")
        .build();

    match client.create_order(&limit_order).await {
        Ok(response) => {
            println!("   Order created successfully!");
            println!("   Order ID: {}", response.order_id);
            println!("   Order link ID: {}", response.order_link_id);
        }
        Err(e) => println!("   Error creating order: {}", e),
    }

    println!("\n2. Creating a market order...");
    let market_order = CreateOrderRequest::builder()
        .category("linear")
        .symbol("BTCUSDT")
        .side("Buy")
        .order_type("Market")
        .qty("0.001")
        .build();

    match client.create_order(&market_order).await {
        Ok(response) => {
            println!("   Market order created successfully!");
            println!("   Order ID: {}", response.order_id);
        }
        Err(e) => println!("   Error creating market order: {}", e),
    }

    println!("\n3. Creating an order with take profit and stop loss...");
    let order_with_tp_sl = CreateOrderRequest::builder()
        .category("linear")
        .symbol("BTCUSDT")
        .side("Buy")
        .order_type("Limit")
        .qty("0.001")
        .price("28000")
        .take_profit("30000")
        .stop_loss("27000")
        .build();

    match client.create_order(&order_with_tp_sl).await {
        Ok(response) => {
            println!("   Order with TP/SL created successfully!");
            println!("   Order ID: {}", response.order_id);
        }
        Err(e) => println!("   Error creating order with TP/SL: {}", e),
    }

    println!("\n4. Creating a reduce-only order...");
    let reduce_only_order = CreateOrderRequest::builder()
        .category("linear")
        .symbol("BTCUSDT")
        .side("Sell")
        .order_type("Market")
        .qty("0.001")
        .reduce_only(true)
        .build();

    match client.create_order(&reduce_only_order).await {
        Ok(response) => {
            println!("   Reduce-only order created successfully!");
            println!("   Order ID: {}", response.order_id);
        }
        Err(e) => println!("   Error creating reduce-only order: {}", e),
    }

    println!("\n5. Creating an order with custom order link ID...");
    let order_with_link_id = CreateOrderRequest::builder()
        .category("linear")
        .symbol("BTCUSDT")
        .side("Buy")
        .order_type("Limit")
        .qty("0.001")
        .price("28000")
        .order_link_id("my_custom_order_link_id")
        .build();

    match client.create_order(&order_with_link_id).await {
        Ok(response) => {
            println!("   Order with custom link ID created successfully!");
            println!("   Order ID: {}", response.order_id);
            println!("   Order link ID: {}", response.order_link_id);
        }
        Err(e) => println!("   Error creating order with custom link ID: {}", e),
    }

    println!("\n6. Getting open orders...");
    match client.get_open_orders("linear").await {
        Ok(orders) => {
            println!("   Open orders: {}", orders.list.len());
            for order in orders.list.iter().take(3) {
                println!(
                    "     {} {} {} @ {} - Status: {}",
                    order.side, order.symbol, order.order_type, order.price, order.status
                );
            }
        }
        Err(e) => println!("   Error getting open orders: {}", e),
    }

    println!("\n7. Getting a specific order...");
    let order_id = "replace_with_order_id";
    match client.get_order("linear", order_id).await {
        Ok(orders) => {
            if !orders.list.is_empty()
                && let Some(order) = orders.list.first()
            {
                println!("   Order details:");
                println!("     Order ID: {}", order.order_id);
                println!("     Symbol: {}", order.symbol);
                println!("     Side: {}", order.side);
                println!("     Type: {}", order.order_type);
                println!("     Price: {}", order.price);
                println!("     Quantity: {}", order.qty);
                println!("     Filled: {}", order.cum_exec_qty);
                println!("     Status: {}", order.status);
            } else {
                println!("   No order found with ID: {}", order_id);
            }
        }
        Err(e) => println!("   Error getting order: {}", e),
    }

    println!("\n8. Canceling a specific order...");
    let cancel_order_id = "replace_with_order_id";
    match client
        .cancel_order("linear", cancel_order_id, "BTCUSDT")
        .await
    {
        Ok(_) => println!("   Order canceled successfully!"),
        Err(e) => println!("   Error canceling order: {}", e),
    }

    println!("\n9. Canceling all orders for a symbol...");
    match client.cancel_all_orders("linear", "BTCUSDT").await {
        Ok(_) => println!("   All orders for BTCUSDT canceled successfully!"),
        Err(e) => println!("   Error canceling all orders: {}", e),
    }

    println!("\nOrder management examples completed!");

    Ok(())
}
