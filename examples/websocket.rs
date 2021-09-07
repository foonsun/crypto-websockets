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
    let access_key = "";
    let secret_key = "";
    let passphrase = "";
    let mut credentials: HashMap<Subscription, (String, String, String) > = HashMap::new();
    /*
    let binance_access_key = "".to_string();
    let binance_secret_key = "".to_string();
    credentials.insert(Subscription::BinanceUSwapOrder, (binance_access_key,binance_secret_key));
    */

    
    credentials.insert(Subscription::OkexOrderStream, (access_key.to_string(), secret_key.to_string(), passphrase.to_string()));

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

            WebsocketEvent::HuobiUSwapOrderBook(event) => println!{"HuobiUSwapOrderbook: {:?}", event},
            WebsocketEvent::HuobiUSwapIncrementalOrderBook(event) => println!{"HuobiUSwapIncrementalOrderbook: {:?}", event},
            WebsocketEvent::HuobiUSwapBBO(event) => println!{"HuobiUSwapBBO: {:?}", event},
            WebsocketEvent::HuobiUSwapKline(event) => println!{"HuobiUSwapKline: {:?}", event},
            WebsocketEvent::HuobiUSwapTradeDetail(event) => println!{"HuobiUSwapTradeDetail: {:?}", event},
            WebsocketEvent::HuobiUSwapAccount(event) => println!{"HuobiUSwapAccount: {:?}", event},
            WebsocketEvent::HuobiUSwapOrder(event) => println!{"HuobiUSwapOrder: {:?}", event},
            WebsocketEvent::HuobiUSwapMatchOrder(event) => println!{"HuobiUSwapMatchOrder: {:?}", event},
            WebsocketEvent::HuobiUSwapPosition(event) => println!{"HuobiUSwapPosition: {:?}", event},

            WebsocketEvent::OkexOrderBook(event) => println!{"Okex Orderbook: {:?}", event},
            WebsocketEvent::OkexTrade(event) => println!{"Okex Trade: {:?}", event},
            WebsocketEvent::OkexAccountPosition(event) => println!{"Okex Account Position: {:?}", event},
            WebsocketEvent::OkexOrder(event) => println!{"Okex Order: {:?}", event},
            WebsocketEvent::OkexAccount(event) => println!{"Okex Account:{:?}", event},
            WebsocketEvent::OkexPosition(event) => println!{"Okex Position: {:?}", event},

            _ => (),
        };

        Ok(())
    });

    let mut subs: HashMap<Subscription, Vec<&str> > = HashMap::new();

    let binance_market_topics = vec![
        "btcusdt@depth5@100ms",
        "ethusdt@depth5@100ms",
        // "ethusdt@depth@100ms",
        // "btcusdt@depth@100ms",
    ];

    // subs.insert(Subscription::BinanceSpotMStream, binance_market_topics);

    /*
    // generate listenkey
    let listenkey = "";

    let binance_account_topics = vec![
        listenkey.clone()
    ];

   subs.insert(Subscription::BinanceSpotOrder, binance_account_topics);
   */

    let binance_uswap_market_topics = vec![
        "dotusdt@depth5@100ms",
        "linkusdt@depth5@100ms",
        // "dotusdt@depth@100ms",
        // "linkusdt@depth@100ms",
        // "BTCUP@tokenNav",
    ];

    // subs.insert(Subscription::BinanceUSwapMStream, binance_uswap_market_topics);

    let huobi_uswap_market_topics = vec![
        "market.BTC-USDT.kline.1min",
        "market.BTC-USDT.depth.step0",
        "market.BTC-USDT.depth.size_20.high_freq",
        "market.BTC-USDT.bbo",
        "market.BTC-USDT.trade.detail",
    ];
    // subs.insert(Subscription::HuobiUSwapMarketStream, huobi_uswap_market_topics);

    let huobi_uswap_order_topics = vec![
        "orders_cross.btc-usdt",
        "matchOrders_cross.btc-usdt",
        "accounts_cross.usdt",
        "positions_cross.btc-usdt",
    ];
    // subs.insert(Subscription::HuobiUSwapOrderStream, huobi_uswap_order_topics);

    let okex_market_topics = vec![
        "BTC-USDT"
    ];
    // subs.insert(Subscription::OkexMarketStream, okex_market_topics);

    let okex_order_topics = vec![
        "SWAP"
    ];
    subs.insert(Subscription::OkexOrderStream, okex_order_topics);

    if let Err(e) = ws.connect(subs).await {
        println!("### websocket error: {:?}", e);
    }

    Ok(())
}