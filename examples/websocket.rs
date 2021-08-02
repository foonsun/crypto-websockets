use crypto_websockets::{Subscription, WebsocketEvent, Websocket};
use failure::Fallible;
use std::{
    collections::HashMap,
};
extern crate simple_logger;

#[tokio::main]
async fn main() -> Fallible<()> {
    // simple_logger::init().unwrap();

    // let access_key = "";
    // let secret_key = "";
    let mut credentials: HashMap<Subscription, (String, String) > = HashMap::new();
    let binance_access_key = "".to_string();
    let binance_secret_key = "".to_string();
    credentials.insert(Subscription::BinanceUSwapOrder, (binance_access_key,binance_secret_key));

    let mut ws: Websocket = Websocket::new(credentials, |event: WebsocketEvent| {
        match event {
            WebsocketEvent::BinanceSpotOrderBook(event) => println!{"BinanceSpotOrderBook: {:?}", event},
            WebsocketEvent::BinanceSpotDepthOrderBook(event) => println!{"BinanceSpotDepthOrderBook: {:?}", event},
            WebsocketEvent::BinanceSpotAccountUpdate(event) => println!{"BinanceSpotAccountUpdate: {:?}", event},
            WebsocketEvent::BinanceSpotOrderTrade(event) => println!{"BinanceSpotOrderTrade: {:?}", event},
            WebsocketEvent::BinanceUSwapEtpNavEvent(event) => println!{"BinanceUSwapEtpNavEvent: {:?}", event},
            WebsocketEvent::BinanceUSwapOrderBook(event) => println!{"BinanceUSwapEtpNavEvent: {:?}", event},
            WebsocketEvent::BinanceUSwapDepthOrderBookEvent(event) => println!{"BinanceUSwapDepthOrderBook: {:?}", event},
            WebsocketEvent::BinanceUSwapAccountUpdateEvent(event) => println!{"BinanceUSwapAccountUpdateEvent: {:?}", event},
            WebsocketEvent::BinanceUSwapOrderTradeEvent(event) => println!{"BinanceUSwapOrderTradeEvent: {:?}", event},

            _ => (),
        };

        Ok(())
    });

    let mut subs: HashMap<Subscription, Vec<&str> > = HashMap::new();

    let binance_market_topics = vec![
        "btcusdt@depth5@100ms",
        "ethusdt@depth5@100ms",
        "ethusdt@depth@100ms",
        "btcusdt@depth@100ms",
    ];

    subs.insert(Subscription::BinanceSpotMStream, binance_market_topics);

    // generate listenkey
    let listenkey = "";

    let binance_account_topics = vec![
        listenkey.clone()
    ];

   subs.insert(Subscription::BinanceSpotOrder, binance_account_topics);

    let binance_uswap_market_topics = vec![
        "dotusdt@depth5@100ms",
        "linkusdt@depth5@100ms",
        "dotusdt@depth@100ms",
        "linkusdt@depth@100ms",
        "BTCUP@tokenNav",
    ];

    subs.insert(Subscription::BinanceUSwapMStream, binance_uswap_market_topics);
    if let Err(e) = ws.connect(subs).await {
        println!("### websocket error: {:?}", e);
    }

    Ok(())
}