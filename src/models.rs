use serde::{Deserialize,  Serialize};
use std::fmt::{self, Display};
use crate::binance_model;
use crate::binance_uswap_model;
use crate::huobi_uswap_model;
use crate::okex_model;


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Subscription {
    BinanceSpotMStream,
    BinanceSpotOrder,
    BinanceUSwapMStream,
    BinanceUSwapOrder,

    HuobiUSwapMarketStream,
    HuobiUSwapOrderStream,

    OkexMarketStream,
    OkexOrderStream,
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
    BinanceSpotBalanceUpdate(BnWsRx<binance_model::BalanceUpdate>),
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
    BinanceUSwapAccountUpdateEvent(binance_uswap_model::AccountUpdateEvent),
    BinanceUSwapOrderTradeEvent(binance_uswap_model::OrderTradeEvent),
    BinanceUSwapListenKeyEvent(binance_uswap_model::ListenKeyEvent),
    BinanceUSwapMarginCallEvent(binance_uswap_model::MarginCallEvent),
    BinanceUSwapAccountConfigEvent(binance_uswap_model::AccountConfigEvent),
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


    //Huobi USDT Cross Swap
    //Market
    HuobiUSwapIncrementalOrderBook(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::IncrementalOrderBook>),
    HuobiUSwapOrderBook(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::OrderBook>),
    HuobiUSwapBBO(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::BBO>),
    HuobiUSwapKline(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::Kline>),
    HuobiUSwapTradeDetail(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::TradeDetail>),
    //Account
    HuobiUSwapAccount(huobi_uswap_model::WSAccountResponse<Vec<huobi_uswap_model::Account>>),
    HuobiUSwapOrder(huobi_uswap_model::OrderWSResponse),
    HuobiUSwapMatchOrder(huobi_uswap_model::MatchOrderWSResponse),
    HuobiUSwapPosition(huobi_uswap_model::WSAccountResponse<Vec<huobi_uswap_model::Position>>),

    //Okex
    OkexOrderBook(okex_model::WsRsp<okex_model::OrderBook>),
    OkexTrade(okex_model::WsRsp<okex_model::Trade>),
    OkexOrder(okex_model::WsRsp<okex_model::Order>),
    OkexAccountPosition(okex_model::WsRsp<okex_model::BalancePositionData>),

    Text(String),

}



#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BinanceSpotWebsocketEvent {
    //Binance Spot
    BinanceSpotAccountUpdate(BnWsRx<binance_model::AccountUpdateEvent>),
    BinanceSpotBalanceUpdate(BnWsRx<binance_model::BalanceUpdate>),
    BinanceSpotOrderTrade(BnWsRx<binance_model::OrderTradeEvent>),
    BinanceSpotAggrTrades(BnWsRx<binance_model::AggrTradesEvent>),
    BinanceSpotTrade(BnWsRx<binance_model::TradeEvent>),
    BinanceSpotOrderBook(BnWsRx<binance_model::OrderBook>),
    BinanceSpotDayTicker(BnWsRx<binance_model::DayTickerEvent>),
    BinanceSpotDayTickerAll(BnWsRx<Vec<binance_model::DayTickerEvent>>),
    BinanceSpotKline(BnWsRx<binance_model::KlineEvent>),
    BinanceSpotDepthOrderBook(BnWsRx<binance_model::DepthOrderBookEvent>),
    BinanceSpotBookTicker(BnWsRx<binance_model::BookTickerEvent>),

    Text(String),

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
    BinanceUSwapAccountUpdateEvent(binance_uswap_model::AccountUpdateEvent),
    BinanceUSwapOrderTradeEvent(binance_uswap_model::OrderTradeEvent),
    BinanceUSwapListenKeyEvent(binance_uswap_model::ListenKeyEvent),
    BinanceUSwapMarginCallEvent(binance_uswap_model::MarginCallEvent),
    BinanceUSwapAccountConfigEvent(binance_uswap_model::AccountConfigEvent),
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

    Text(String),
}




#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum HuobiUSwapWebsocketEvent {
    //Ping,Sub,Op
    HuobiUSwapMarketPing(huobi_uswap_model::MarketPing),
    HuobiUSwapSubStatus(huobi_uswap_model::SubStatus),
    //Market
    HuobiUSwapIncrementalOrderBook(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::IncrementalOrderBook>),
    HuobiUSwapOrderBook(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::OrderBook>),
    HuobiUSwapBBO(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::BBO>),
    HuobiUSwapKline(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::Kline>),
    HuobiUSwapTradeDetail(huobi_uswap_model::WSMarketResponse<huobi_uswap_model::TradeDetail>),
    //Account
    HuobiUSwapAccount(huobi_uswap_model::WSAccountResponse<Vec<huobi_uswap_model::Account>>),
    HuobiUSwapOrder(huobi_uswap_model::OrderWSResponse),
    HuobiUSwapMatchOrder(huobi_uswap_model::MatchOrderWSResponse),
    HuobiUSwapPosition(huobi_uswap_model::WSAccountResponse<Vec<huobi_uswap_model::Position>>),
    //sub status
    HuobiUSwapOpStatus(huobi_uswap_model::OpStatus),

    Text(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum OkexWebsocketEvent {
    OkexSubRsp(okex_model::SubRsp),
    OkexSubEvent(okex_model::SubEvent),

    OkexOrderBook(okex_model::WsRsp<okex_model::OrderBook>),
    OkexTrade(okex_model::WsRsp<okex_model::Trade>),

    OkexOrder(okex_model::WsRsp<okex_model::Order>),

    OkexAccountPosition(okex_model::WsRsp<okex_model::BalancePositionData>),

    Pong,
}