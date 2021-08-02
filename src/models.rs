use serde::{Deserialize,  Serialize};
use std::fmt::{self, Display};
use crate::binance_model;
use crate::binance_uswap_model;


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Subscription {
    BinanceSpotMStream,
    BinanceSpotOrder,
    BinanceUSwapMStream,
    BinanceUSwapOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BnWsRx<T> {
    pub stream: String,
    pub data: T,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketEvent {
    //Binance Spot
    BinanceSpotAccountUpdate(BnWsRx<binance_model::AccountUpdateEvent>),
    BinanceSpotOrderTrade(BnWsRx<binance_model::OrderTradeEvent>),
    BinanceSpotAggrTrades(BnWsRx<binance_model::AggrTradesEvent>),
    BinanceSpotTrade(BnWsRx<binance_model::TradeEvent>),
    BinanceSpotOrderBook(BnWsRx<binance_model::OrderBook>),
    BinanceSpotDayTicker(BnWsRx<binance_model::DayTickerEvent>),
    BinanceSpotDayTickerAll(BnWsRx<Vec<binance_model::DayTickerEvent>>),
    BinanceSpotKline(BnWsRx<binance_model::KlineEvent>),
    BinanceSpotDepthOrderBook(BnWsRx<binance_model::DepthOrderBookEvent>),
    BinanceSpotBookTicker(BnWsRx<binance_model::BookTickerEvent>),

    //Binance USDT Swap
    BinanceUSwapVec(BnWsRx<Vec<binance_uswap_model::DayTickerEvent>>),
    BinanceUSwapDayTickerEvent(BnWsRx<binance_uswap_model::DayTickerEvent>),
    BinanceUSwapBookTickerEvent(BnWsRx<binance_uswap_model::BookTickerEvent>),
    BinanceUSwapMiniTickerEvent(BnWsRx<binance_uswap_model::MiniTickerEvent>),
    BinanceUSwapVecMiniTickerEvent(BnWsRx<Vec<binance_uswap_model::MiniTickerEvent>>),
    BinanceUSwapAccountUpdateEvent(BnWsRx<binance_uswap_model::AccountUpdateEvent>),
    BinanceUSwapOrderTradeEvent(BnWsRx<binance_uswap_model::OrderTradeEvent>),
    BinanceUSwapAggrTradesEvent(BnWsRx<binance_uswap_model::AggrTradesEvent>),
    BinanceUSwapIndexPriceEvent(BnWsRx<binance_uswap_model::IndexPriceEvent>),
    BinanceUSwapMarkPriceEvent(BnWsRx<binance_uswap_model::MarkPriceEvent>),
    BinanceUSwapVecMarkPriceEvent(BnWsRx<Vec<binance_uswap_model::MarkPriceEvent>>),
    BinanceUSwapTradeEvent(BnWsRx<binance_uswap_model::TradeEvent>),
    BinanceUSwapKlineEvent(BnWsRx<binance_uswap_model::KlineEvent>),
    BinanceUSwapContinuousKlineEvent(BnWsRx<binance_uswap_model::ContinuousKlineEvent>),
    BinanceUSwapIndexKlineEvent(BnWsRx<binance_uswap_model::IndexKlineEvent>),
    BinanceUSwapLiquidationEvent(BnWsRx<binance_uswap_model::LiquidationEvent>),
    BinanceUSwapOrderBook(BnWsRx<binance_uswap_model::OrderBook>),
    BinanceUSwapDepthOrderBookEvent(BnWsRx<binance_uswap_model::DepthOrderBookEvent>),
    BinanceUSwapEtpNavEvent(BnWsRx<binance_uswap_model::EtpNavEvent>),

}



#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BinanceSpotWebsocketEvent {
    //Binance Spot
    BinanceSpotAccountUpdate(BnWsRx<binance_model::AccountUpdateEvent>),
    BinanceSpotOrderTrade(BnWsRx<binance_model::OrderTradeEvent>),
    BinanceSpotAggrTrades(BnWsRx<binance_model::AggrTradesEvent>),
    BinanceSpotTrade(BnWsRx<binance_model::TradeEvent>),
    BinanceSpotOrderBook(BnWsRx<binance_model::OrderBook>),
    BinanceSpotDayTicker(BnWsRx<binance_model::DayTickerEvent>),
    BinanceSpotDayTickerAll(BnWsRx<Vec<binance_model::DayTickerEvent>>),
    BinanceSpotKline(BnWsRx<binance_model::KlineEvent>),
    BinanceSpotDepthOrderBook(BnWsRx<binance_model::DepthOrderBookEvent>),
    BinanceSpotBookTicker(BnWsRx<binance_model::BookTickerEvent>),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BinanceUSwapWebsocketEvent {
    //Binance USDT Swap
    BinanceUSwapVec(BnWsRx<Vec<binance_uswap_model::DayTickerEvent>>),
    BinanceUSwapDayTickerEvent(BnWsRx<binance_uswap_model::DayTickerEvent>),
    BinanceUSwapBookTickerEvent(BnWsRx<binance_uswap_model::BookTickerEvent>),
    BinanceUSwapMiniTickerEvent(BnWsRx<binance_uswap_model::MiniTickerEvent>),
    BinanceUSwapVecMiniTickerEvent(BnWsRx<Vec<binance_uswap_model::MiniTickerEvent>>),
    BinanceUSwapAccountUpdateEvent(BnWsRx<binance_uswap_model::AccountUpdateEvent>),
    BinanceUSwapOrderTradeEvent(BnWsRx<binance_uswap_model::OrderTradeEvent>),
    BinanceUSwapAggrTradesEvent(BnWsRx<binance_uswap_model::AggrTradesEvent>),
    BinanceUSwapIndexPriceEvent(BnWsRx<binance_uswap_model::IndexPriceEvent>),
    BinanceUSwapMarkPriceEvent(BnWsRx<binance_uswap_model::MarkPriceEvent>),
    BinanceUSwapVecMarkPriceEvent(BnWsRx<Vec<binance_uswap_model::MarkPriceEvent>>),
    BinanceUSwapTradeEvent(BnWsRx<binance_uswap_model::TradeEvent>),
    BinanceUSwapKlineEvent(BnWsRx<binance_uswap_model::KlineEvent>),
    BinanceUSwapContinuousKlineEvent(BnWsRx<binance_uswap_model::ContinuousKlineEvent>),
    BinanceUSwapIndexKlineEvent(BnWsRx<binance_uswap_model::IndexKlineEvent>),
    BinanceUSwapLiquidationEvent(BnWsRx<binance_uswap_model::LiquidationEvent>),
    BinanceUSwapOrderBook(BnWsRx<binance_uswap_model::OrderBook>),
    BinanceUSwapDepthOrderBookEvent(BnWsRx<binance_uswap_model::DepthOrderBookEvent>),
    BinanceUSwapEtpNavEvent(BnWsRx<binance_uswap_model::EtpNavEvent>),

}
