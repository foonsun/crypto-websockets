use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubStatus {
    pub id: String,
    pub subbed: Option<String>,
    pub ts: u64,
    pub status: String,
    #[serde(rename = "err-code")]
    pub err_code: Option<String>,
    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPing {
    pub ping: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Ts {
    St(String),
    It(u64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpStatus {
    pub op: String,
    #[serde(rename = "type")]
    pub otype: Option<String>,
    pub ts: Ts,
    #[serde(rename = "err-code")]
    pub err_code: Option<u32>,
    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
    pub cid: Option<String>,
    pub topic: Option<String>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WSMarketResponse<T> {
    pub ch: String,
    pub ts: u64,
    pub tick: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WSAccountResponse<T> {
    pub op: String,
    pub topic: String,
    pub ts: u64,
    pub uid: Option<String>,
    pub event: String,
    pub data: T,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBook {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub mrid: u64,
    pub id: u64,
    pub ts: u64,
    pub version: u64,
    pub ch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncrementalOrderBook {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub mrid: u64,
    pub id: u64,
    pub ts: u64,
    pub version: u64,
    pub ch: String,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kline {
    #[serde(rename = "id")]
    pub timestamp: u64,
    #[serde(rename = "vol")]
    pub volume: f64,
    pub count: f64,
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub amount: f64,
    pub mrid: Option<u64>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BBO {
    pub bid: (f64, f64),
    pub ask: (f64, f64),
    pub id: u64,
    pub ts: u64,
    pub version: u64,
    pub ch: String,
    pub mrid: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeDetail {
    pub id: u64,
    pub ts: u64,
    pub data: Vec<TradeDetailItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeDetailItem {
    pub amount: u32,
    pub ts: u64,
    pub id: u64,
    pub price: f64,
    pub direction: String,
    pub quantity: f64,
    pub trade_turnover: f64
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub margin_mode: String,
    pub margin_account: String,
    pub margin_asset: String,
    pub margin_balance: f64,
    pub margin_static: f64,
    pub margin_position: f64,
    pub margin_frozen: f64,
    pub profit_real: f64,
    pub profit_unreal: f64,
    pub risk_rate: Option<f64>,
    pub liquidation_price: Option<f64>,
    pub withdraw_available: f64,
    pub contract_detail: Vec<AccountDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountDetail {
    pub symbol: String,
    pub contract_code: String,
    pub margin_position: f64,
    pub margin_frozen: f64,
    pub margin_available: f64,
    pub profit_unreal: f64,
    pub liquidation_price: Option<f64>,
    pub lever_rate: u32,
    pub adjust_factor: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderWSResponse {
    pub op: String,
    pub topic: String,
    pub uid: String,
    pub ts: u64,
    pub symbol: String,
    pub contract_code: String,
    pub volume: u32,
    pub price: f64,
    pub order_price_type: String,
    pub direction: String,
    pub offset: String,
    pub status: u32,
    pub lever_rate: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: Option<u64>,
    pub order_source: String,
    pub order_type: u32,
    pub created_at: u64,
    pub trade_volume: u32,
    pub trade_turnover: f64,
    pub fee: f64,
    pub trade_avg_price: f64,
    pub margin_frozen: f64,
    pub margin_asset: String,
    pub profit: f64,
    pub liquidation_type: String,
    pub canceled_at: u64,
    pub fee_asset: String,
    pub margin_mode: String,
    pub margin_account: String,
    pub trade: Vec<TradeSubItem>,
} 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchOrderWSResponse {
    pub op: String,
    pub topic: String,
    pub uid: String,
    pub ts: u64,
    pub symbol: String,
    pub contract_code: String,
    pub margin_mode: String,
    pub margin_account: String,
    pub status: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: Option<u64>,
    pub order_type: u32,
    pub volume: u32,
    pub trade_volume: u32,
    pub direction: String,
    pub offset: String,
    pub lever_rate: u32,
    pub price: f64,
    pub created_at: u64,
    pub order_source: String,
    pub order_price_type: String,
    pub trade: Vec<TradeSubItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeSubItem {
    pub trade_id: u64,
    pub id: String,
    pub trade_volume: u32,
    pub trade_price: f64,
    pub trade_fee: Option<f64>,
    pub fee_asset: Option<String>,
    pub trade_turnover: f64,
    pub created_at: u64,
    pub role: String,
    pub profit: f64,
    pub real_profit: f64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub symbol: String,
    pub contract_code: String,
    pub volume: f64,
    pub available: f64,
    pub frozen: f64,
    pub cost_open: f64,
    pub cost_hold: f64,
    pub profit_unreal: f64,
    pub profit_rate: f64,
    pub profit: f64,
    pub margin_asset: String,
    pub position_margin: f64,
    pub lever_rate: u32,
    pub direction: String,
    pub last_price: f64,
    pub margin_mode: String,
    pub margin_account: String,
}










