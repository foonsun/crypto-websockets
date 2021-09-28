use crate::{
    error::Error,
    models::*,
};
use failure::Fallible;
use futures::{prelude::*, stream::SplitStream, stream::SplitSink};
use serde_json::from_str;
use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use streamunordered::{StreamUnordered, StreamYield};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio::time::Interval;
use tracing::*;
use tungstenite::Message;
use url::Url;
use std::io::Read;
use flate2::read::GzDecoder;


type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub type StoredStream = SplitStream<WSStream>;
pub type StoredSink = SplitSink<WSStream, tungstenite::Message>;

#[allow(clippy::module_name_repetitions)]
pub struct Websocket  {
    credentials: HashMap<Subscription, (String, String, String)>,
    subscriptions: HashMap<Subscription, usize>,
    pub streams: StreamUnordered<StoredStream>,
    pub tokens: HashMap<usize, Subscription>,
    pub sinks: HashMap<Subscription, StoredSink>,
    pub handler: Box<dyn FnMut(WebsocketEvent) -> Fallible<()>>,
    pub ping_timer: Interval,
}

impl Websocket {
    pub fn new<Callback: 'static>(credentials: HashMap<Subscription,(String, String, String)>, handler: Callback) -> Self
    where
        Callback: FnMut(WebsocketEvent) -> Fallible<()> + Send
    {
        Self {
            credentials: credentials.clone(),
            subscriptions: HashMap::new(),
            tokens: HashMap::new(),
            streams: StreamUnordered::new(),
            sinks: HashMap::new(),
            handler: Box::new(handler),
            ping_timer: tokio::time::interval(Duration::from_secs(5)),
        }
    }

    pub async fn subscribe(&mut self, subscription: Subscription, topics: &Vec<&str> ) -> Fallible<()> {
        let ws_url = match subscription {
            Subscription::BinanceSpotMStream => "wss://stream.binance.com:9443",
            Subscription::BinanceSpotOrder => "wss://stream.binance.com:9443",
            Subscription::BinanceUSwapMStream => "wss://fstream.binance.com",
            Subscription::BinanceUSwapOrder => "wss://fstream.binance.com",

            Subscription::HuobiUSwapMarketStream => "wss://api.hbdm.vn",
            Subscription::HuobiUSwapOrderStream => "wss://api.hbdm.vn",

            Subscription::OkexMarketStream => "wss://wsaws.okex.com:8443",
            Subscription::OkexOrderStream => "wss://wsaws.okex.com:8443",

            Subscription::FtxMarketStream => "wss://ftx.com",
            Subscription::FtxOrderStream => "wss://ftx.com",
        };

        let end = match subscription {
            Subscription::BinanceSpotMStream => 
                format!("/stream?streams={}", topics.join("/")),
            Subscription::BinanceSpotOrder => 
                format!("/stream?streams={}", topics.join("/")),
            Subscription::BinanceUSwapMStream => 
                format!("/stream?streams={}", topics.join("/")),
            Subscription::BinanceUSwapOrder =>
                format!("/ws/{}", topics.join("/")),
            Subscription::HuobiUSwapMarketStream =>
                format!("/linear-swap-ws"),
            Subscription::HuobiUSwapOrderStream =>
                format!("/linear-swap-notification"),
            Subscription::OkexMarketStream =>
                format!("/ws/v5/public"),
            Subscription::OkexOrderStream =>
                format!("/ws/v5/private"),
            Subscription::FtxMarketStream =>
                format!("/ws"),
            Subscription::FtxOrderStream =>
                format!("/ws"),
        };

        trace!("[Websocket] Subscribing to '{:?}'", subscription.clone());

        let endpoint = Url::parse(&format!("{}{}", ws_url, end)).unwrap();
        
        let (ws_stream, _) = connect_async(endpoint.clone()).await?;
        info!("[Websocket] websocket handshake has been successfully completed.{:?}", endpoint.clone());

        let (sink, stream) = ws_stream.split();
        
        let token = self.streams.insert(stream);

        self.sinks.insert(subscription.clone(), sink);
        self.subscriptions.insert(subscription.clone(), token);
        self.tokens.insert(token, subscription.clone());

        Ok(())

    }

    pub fn unsubscribe(&mut self, subscription: &Subscription) -> Option<StoredStream> {
        let streams = Pin::new(&mut self.streams);
        self.subscriptions
            .get(subscription)
            .and_then(|token| StreamUnordered::take(streams, *token))
    }

    pub fn check_key(&self, subscription: &Subscription) -> Fallible<(&str, &str)> {
        match self.credentials.get(&subscription).as_ref() {
            None => Err(Error::NoApiKeySet.into()),
            Some((k, s,p )) => Ok((k, s)),
        }
    }

    pub fn okex_check_key(&self, subscription: &Subscription) -> Fallible<(&str, &str, &str)> {
        match self.credentials.get(&subscription).as_ref() {
            None => Err(Error::NoApiKeySet.into()),
            Some((k, s, p)) => Ok((k, s, p)),
        }
    }

    pub fn ftx_check_key(&self, subscription: &Subscription) -> Fallible<(&str, &str, &str)> {
        match self.credentials.get(&subscription).as_ref() {
            None => Err(Error::NoApiKeySet.into()),
            Some((k, s, account)) => Ok((k, s, account)),
        }
    }

    pub fn parse_message(&self, msg: Message, token: usize) -> Fallible<(WebsocketEvent, usize)> {
        let subscription = self.tokens.get(&token).unwrap();
        let bin = match msg {
            // Message::Text(msg) => return Ok(WebsocketEvent::Text(msg)),
            Message::Text(msg) => {
                let message: WebsocketEvent = from_str(&msg)?;
                println!("1.{:?}", message.clone());
                return Ok((message, token));
            },
            Message::Binary(b) => b,
            Message::Pong(c) => c,
            Message::Ping(d) => d,
            Message::Close(..) => return Err(failure::format_err!("Socket closed")),
        };

        let mut d = GzDecoder::new(&*bin);
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();

        trace!("Incoming websocket message {:?}", s);
        
        let message: WebsocketEvent = from_str(&s)?;

        // print!("messsgae: {:?}", message );

        Ok((message, token))
    }


}

impl  Stream for Websocket {
    type Item = Fallible<(Message, usize)>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.as_mut().get_mut().streams).poll_next(cx) {
            Poll::Ready(Some((y, token))) => match y {
                StreamYield::Item(item) => {
                    Poll::Ready({
                        Some(
                            match item.map_err(failure::Error::from)
                            {
                                Ok(msg) => Ok((msg, token)),
                                Err(err) => Err(failure::Error::from(err)),
                            }
                        )
                    })
                }
                StreamYield::Finished(_) => Poll::Pending,
            },
            Poll::Ready(None) => Poll::Ready(Some(Err(Error::NoStreamSubscribed.into()))),
            Poll::Pending => Poll::Pending,
        }
    }
}

