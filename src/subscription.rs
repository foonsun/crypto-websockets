use crate::{
    models::*, 
    websocket::*,
};
use std::{
    collections::HashMap,
    collections::BTreeMap,
};
use failure::Fallible;
use futures::prelude::*;
use tracing::*;
use std::time::Duration;
use std::thread;
use tungstenite::Message;
use serde_json::{json, from_str};
use ring::{digest, hmac};
use flate2::read::GzDecoder;
use std::io::Read;
use tokio::net::TcpStream;
use streamunordered::{StreamUnordered, StreamYield};
use futures::{prelude::*, stream::SplitStream, stream::SplitSink};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};


pub const WS_URL: &str = "wss://api.hbdm.vn";
pub const WS_HOST: &str = "api.hbdm.vn";

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub type StoredStream = SplitStream<WSStream>;
pub type StoredSink = SplitSink<WSStream, tungstenite::Message>;

#[allow(clippy::module_name_repetitions)]
pub struct HuobiWebsocket  {
    credential: Option<(String, String)>,
    subscriptions: HashMap<Subscription, usize>,
    tokens: HashMap<usize, Subscription>,
    streams: StreamUnordered<StoredStream>,
    pub sinks: HashMap<Subscription, StoredSink>,
    pub handler: Box<dyn FnMut(WebsocketEvent) -> Fallible<()>>,
}




impl Websocket {

    pub async fn connect(
        &mut self,
        subs: HashMap<Subscription, Vec<&str>>,
    ) -> Fallible<()> {
        for (subscription, topics) in &subs {
            if *subscription == Subscription::BinanceSpotMStream {
                self.subscribe(Subscription::BinanceSpotMStream, topics).await?;
            }
            if *subscription == Subscription::BinanceSpotOrder {
                self.subscribe(Subscription::BinanceSpotOrder, topics).await?;
            }
            if *subscription == Subscription::BinanceUSwapMStream {
                self.subscribe(Subscription::BinanceUSwapMStream, topics).await?;
            }
            if *subscription == Subscription::BinanceUSwapOrder {
                self.subscribe(Subscription::BinanceUSwapOrder, topics).await?;
            }
            if *subscription == Subscription::HuobiUSwapMarketStream {
                self.subscribe(Subscription::HuobiUSwapMarketStream, topics).await?;
                self.huobi_sub_market(Subscription::HuobiUSwapMarketStream, topics).await?;
            }
            if *subscription == Subscription::HuobiUSwapOrderStream {
                self.subscribe(Subscription::HuobiUSwapOrderStream, topics).await?;
                let mut params: BTreeMap<String, String> = BTreeMap::new();
                let signature = self.generate_signature(subscription.clone(), "api.hbdm.vn", "/linear-swap-notification", &mut params);
                let message = json!({
                    "AccessKeyId": params.get(&"AccessKeyId".to_string()),
                    "SignatureMethod": params.get(&"SignatureMethod".to_string()),
                    "SignatureVersion": params.get(&"SignatureVersion".to_string()),
                    "Timestamp": params.get(&"Timestamp".to_string()),
                    "Signature": signature,
                    "op": "auth".to_string(),
                    "type": "api".to_string(),   
                });

                let sink = self.sinks.get_mut(&Subscription::HuobiUSwapOrderStream).unwrap();
                sink.send(tungstenite::Message::Text(message.to_string())).await?;
            }
        }

        self.rx_handler(&subs).await?;

        Ok(())
    }


    pub async fn reconnect(
        &mut self, 
        subs: HashMap<Subscription, Vec<&str>>,
    ) -> Fallible<()> {
        for (subscription, topics) in &subs {
            self.unsubscribe(subscription);
        }
        thread::sleep(Duration::from_millis(5000));
        self.connect(subs).await?;

        Ok(())

    }



    async fn rx_handler(&mut self, subs: &HashMap<Subscription, Vec<&str>>) -> Fallible<()> {
        while let Some((message, token)) = self.try_next().await? {
            let subscription = self.tokens.get(&token).unwrap().clone();
            let bin = match message {
                Message::Text(message) => {
                    if subscription == Subscription::BinanceSpotMStream {
                        let msg: BinanceSpotWebsocketEvent = from_str(&message)?;
                        match msg {
                            BinanceSpotWebsocketEvent::BinanceSpotAggrTrades(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotAggrTrades(msg.clone()))?,
                            BinanceSpotWebsocketEvent::BinanceSpotTrade(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotTrade(msg.clone()))?,
                            BinanceSpotWebsocketEvent::BinanceSpotOrderBook(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotOrderBook(msg.clone()))?,
                            BinanceSpotWebsocketEvent::BinanceSpotDayTicker(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotDayTicker(msg.clone()))?,
                            BinanceSpotWebsocketEvent::BinanceSpotDayTickerAll(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotDayTickerAll(msg.clone()))?,
                            BinanceSpotWebsocketEvent::BinanceSpotKline(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotKline(msg.clone()))?,
                            BinanceSpotWebsocketEvent::BinanceSpotDepthOrderBook(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotDepthOrderBook(msg.clone()))?,
                            _ => (),
                        }
                    }
                    else if subscription == Subscription::BinanceSpotOrder {
                        let msg: BinanceSpotWebsocketEvent = from_str(&message)?;
                        match msg {
                            BinanceSpotWebsocketEvent::BinanceSpotAccountUpdate(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotAccountUpdate(msg.clone()))?,
                            BinanceSpotWebsocketEvent::BinanceSpotOrderTrade(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotOrderTrade(msg.clone()))?,
                            BinanceSpotWebsocketEvent::BinanceSpotBalanceUpdate(ref msg) => (self.handler)(WebsocketEvent::BinanceSpotBalanceUpdate(msg.clone()))?,
                            _ => (),
                        }
                    } 
                    else if subscription == Subscription::BinanceUSwapMStream {
                        let msg: BinanceUSwapWebsocketEvent = from_str(&message)?;
                        match msg {
                            BinanceUSwapWebsocketEvent::BinanceUSwapBookTickerEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapBookTickerEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapAggrTradesEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapAggrTradesEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapDayTickerEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapDayTickerEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapMiniTickerEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapMiniTickerEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapVec(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapVec(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapIndexPriceEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapIndexPriceEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapMarkPriceEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapMarkPriceEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapVecMarkPriceEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapVecMarkPriceEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapTradeEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapTradeEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapContinuousKlineEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapContinuousKlineEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapKlineEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapKlineEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapIndexKlineEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapIndexKlineEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapLiquidationEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapLiquidationEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapOrderBook(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapOrderBook(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapDepthOrderBookEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapDepthOrderBookEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapEtpNavEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapEtpNavEvent(msg.clone()))?,
                            _ => (),
                        }
                    }
                    else if subscription == Subscription::BinanceUSwapOrder {
                        debug!("binance uswap websocket message:{:?}", message);
                        let msg: BinanceUSwapWebsocketEvent = from_str(&message)?;
                        match msg {
                            BinanceUSwapWebsocketEvent::BinanceUSwapOrderTradeEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapOrderTradeEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapAccountUpdateEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapAccountUpdateEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapListenKeyEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapListenKeyEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapMarginCallEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapMarginCallEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapAccountConfigEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapAccountConfigEvent(msg.clone()))?,
                            _ => (),
                        }
                    }
                    else {
                        return Ok(());
                    }

                },
                Message::Binary(b) => {
                    if subscription == Subscription::HuobiUSwapMarketStream {
                        let mut d = GzDecoder::new(&*b);
                        let mut s = String::new();
                        d.read_to_string(&mut s).unwrap();

                        trace!("Incoming websocket message {:?}", s);
                        
                        let msg: HuobiUSwapWebsocketEvent = from_str(&s)?;
                        match msg {
                            HuobiUSwapWebsocketEvent::HuobiUSwapMarketPing(ref msg) => {
                                let ts = chrono::Local::now().timestamp_millis();
                                let message = json!({
                                   "pong": ts,       
                                });
                                let sink = self.sinks.get_mut(&subscription).unwrap();
                                sink.send(tungstenite::Message::Text(message.to_string())).await?;

                            },
                            HuobiUSwapWebsocketEvent::HuobiUSwapSubStatus(ref msg) => {
                                info!("sub status:{:?}", msg.clone());
                            },
                            HuobiUSwapWebsocketEvent::HuobiUSwapOrderBook(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapOrderBook(msg.clone()))?,
                            HuobiUSwapWebsocketEvent::HuobiUSwapIncrementalOrderBook(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapIncrementalOrderBook(msg.clone()))?,
                            HuobiUSwapWebsocketEvent::HuobiUSwapBBO(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapBBO(msg.clone()))?,
                            HuobiUSwapWebsocketEvent::HuobiUSwapKline(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapKline(msg.clone()))?,
                            HuobiUSwapWebsocketEvent::HuobiUSwapTradeDetail(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapTradeDetail(msg.clone()))?,
                            _ => (),
                        }
                    }
                    if subscription == Subscription::HuobiUSwapOrderStream {
                        let mut d = GzDecoder::new(&*b);
                        let mut s = String::new();
                        d.read_to_string(&mut s).unwrap();

                        trace!("Incoming websocket message {:?}", s);
                        
                        let msg: HuobiUSwapWebsocketEvent = from_str(&s)?;
                        match msg {
                            HuobiUSwapWebsocketEvent::HuobiUSwapOpStatus(ref msg) => {
                                if msg.op == "ping" {
                                    let ts = chrono::Local::now().timestamp_millis();
                                    let message = json!({
                                        "op": "pong",
                                        "ts": ts,       
                                    });
                                    debug!("### op pong: {:?}", message);
                                    let sink = self.sinks.get_mut(&subscription).unwrap();
                                    sink.send(tungstenite::Message::Text(message.to_string())).await?;
                
                                }
                                if msg.op == "auth" {
                                    if let Some(err_code) = msg.err_code {
                                        if err_code == 0 {
                                            self.huobi_sub_account(subscription, subs).await?;
                                        }
                                    }
                                }
                                if let Some(_err_code) = msg.err_code {
                                    info!("{:?}", msg);
                                }

                            },
                            HuobiUSwapWebsocketEvent::HuobiUSwapSubStatus(ref msg) => {
                                info!("sub status:{:?}", msg.clone());
                            },
                            HuobiUSwapWebsocketEvent::HuobiUSwapAccount(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapAccount(msg.clone()))?,
                            HuobiUSwapWebsocketEvent::HuobiUSwapOrder(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapOrder(msg.clone()))?,
                            HuobiUSwapWebsocketEvent::HuobiUSwapMatchOrder(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapMatchOrder(msg.clone()))?,
                            HuobiUSwapWebsocketEvent::HuobiUSwapPosition(ref msg) => (self.handler)(WebsocketEvent::HuobiUSwapPosition(msg.clone()))?,
                            _ => (),
                        }

                    }
                },

                Message::Pong(c) => (),
                Message::Ping(d) => (),
                Message::Close(..) => return Err(failure::format_err!("Socket closed")),
            };
            // huobi ws is gziped,so need to parse it again.
        }

        Ok(())
    }


    async fn huobi_sub_market(&mut self, subscription: Subscription, topics: &[&str]) -> Fallible<()> {
        for topic in topics {
            let message = json!({
                "sub": topic,
                "id": "huobiusdtswap_rust"
            });
            let sink = self.sinks.get_mut(&subscription).unwrap();
            sink.send(tungstenite::Message::Text(message.to_string())).await?;

        }
        
        Ok(())

    }

    async fn huobi_sub_account(&mut self, subscription: Subscription,subs: &HashMap<Subscription, Vec<&str> >) -> Fallible<()> {
        let topics = subs.get(&subscription).unwrap();
        for topic in topics {
            let message = json!({
                "op": "sub",
                "cid": "huobiusdtswap_rust",
                "topic": topic,
                });
            let sink = self.sinks.get_mut(&subscription).unwrap();
            sink.send(tungstenite::Message::Text(message.to_string())).await?;
        }

        Ok(())
    }


    fn generate_signature(&mut self, subscription: Subscription, wspoint: &str, suffix: &str, params: & mut BTreeMap<String, String>) -> String
    {
        let (key, secret) = self.check_key(&subscription).expect("no key");

        params.insert("AccessKeyId".to_string(), key.to_string());
        params.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());
        params.insert("SignatureVersion".to_string(), "2".to_string());
        let utc_time = chrono::Utc::now();
        let utctimes = utc_time.format("%Y-%m-%dT%H:%M:%S").to_string();
        params.insert("Timestamp".to_string(), utctimes); 

        let build_params = build_query_string(params.clone());

        let format_str = format!("{}\n{}\n{}\n{}", "GET", wspoint, suffix, build_params,); 

        sign_hmac_sha256_base64(
                    secret,
                    &format_str,
            )

    }

}

pub fn build_query_string(parameters: BTreeMap<String, String>) -> String {
    parameters
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, percent_encode(&value)))
        .collect::<Vec<String>>()
        .join("&")
}

pub fn sign_hmac_sha256_base64(secret: &str, digest: &str) -> String {
    use data_encoding::BASE64;

    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let signature = hmac::sign(&signed_key, digest.as_bytes());
    BASE64.encode(signature.as_ref())
}

pub fn percent_encode(source: &str) -> String {
    use percent_encoding::{define_encode_set, utf8_percent_encode, USERINFO_ENCODE_SET};
    define_encode_set! {
        pub CUSTOM_ENCODE_SET = [USERINFO_ENCODE_SET] | { '+', ',' }
    }
    utf8_percent_encode(source, CUSTOM_ENCODE_SET).to_string()
}