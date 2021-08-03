use crate::{
    models::*, 
    websocket::*,
};
use std::{
    collections::HashMap,
};
use failure::Fallible;
use futures::prelude::*;
use tracing::*;
use std::time::Duration;
use std::thread;
use tungstenite::Message;
use serde_json::from_str;



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
                        let msg: BinanceUSwapWebsocketEvent = from_str(&message)?;
                        match msg {
                            BinanceUSwapWebsocketEvent::BinanceUSwapOrderTradeEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapOrderTradeEvent(msg.clone()))?,
                            BinanceUSwapWebsocketEvent::BinanceUSwapAccountUpdateEvent(ref msg) => (self.handler)(WebsocketEvent::BinanceUSwapAccountUpdateEvent(msg.clone()))?,
                            _ => (),
                        }
                    }
                    else {
                        ()
                    }

                },
                Message::Binary(b) => (),
                Message::Pong(c) => (),
                Message::Ping(d) => (),
                Message::Close(..) => return Err(failure::format_err!("Socket closed")),
            };
            // huobi ws is gziped,so need to parse it again.
        }

        Ok(())
    }
}