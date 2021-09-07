use serde::{Deserialize, Serialize};
use crate::parser::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "channel")]
pub enum Channel {
    #[serde(rename = "books")]
    Books {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "books5")]
    Books5 {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "books50-l2-tbt")]
    Books50L2Tbt {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "books-l2-tbt")]
    BooksL2Tbt {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "instruments")]
    Instruments {
        #[serde(rename = "instType")]
        inst_type: InstType,
    },
    #[serde(rename = "orders")]
    Orders {
        #[serde(rename = "instType")]
        inst_type: InstType,
        uly: Option<String>,
        #[serde(rename = "instId")]
        inst_id: Option<String>,
    },
    #[serde(rename = "price-limit")]
    PriceLimit {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "tickers")]
    Tickers {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "trades")]
    Trades {
        #[serde(rename = "instId")]
        inst_id: String,
    },

    #[serde(rename = "balance_and_position")]
    BalancePosition {
        
    },

    #[serde(rename = "account")]
    Account {

    },

    #[serde(rename = "positions")]
    Position {
    }

}

impl Channel {
    pub fn books(inst_id: &str) -> Self {
        Self::Books {
            inst_id: inst_id.into(),
        }
    }
    pub fn books5(inst_id: &str) -> Self {
        Self::Books5 {
            inst_id: inst_id.into(),
        }
    }

    pub fn books50_l2_tbt(inst_id: &str) -> Self {
        Self::Books50L2Tbt {
            inst_id: inst_id.into(),
        }
    }

    pub fn books_l2_tbt(inst_id: &str) -> Self {
        Self::BooksL2Tbt {
            inst_id: inst_id.into(),
        }
    }

    pub fn instruments(inst_type: InstType) -> Self {
        Self::Instruments { inst_type }
    }

    pub fn tickers(inst_id: &str) -> Self {
        Self::Tickers {
            inst_id: inst_id.into(),
        }
    }

    pub fn price_limit(inst_id: &str) -> Self {
        Self::PriceLimit {
            inst_id: inst_id.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubRsp {
    pub event: String,
    pub arg: Channel,
} 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubEvent {
    pub event: String,
    pub code: String,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsRsp<T> {
    pub arg: Channel,
    pub action: Option<Action>,
    pub data: Vec<T>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Snapshot,
    Update,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepthInfo (
    #[serde(deserialize_with = "crate::parser::from_str")] 
    pub f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub f64,
);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub asks: Vec<DepthInfo>,
    pub bids: Vec<DepthInfo>,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub ts: DateTime<Utc>,
    pub checksum: i64,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimit {
    pub inst_id: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub buy_lmt: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub sell_lmt: f64,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub inst_id: String,
    pub trade_id: String,
    #[serde(rename = "px")]
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub price: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    #[serde(rename = "sz")]
    pub size: f64,
    pub side: String,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub ts: DateTime<Utc>,
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Alias {
    ThisWeek,
    NextWeek,
    Quarter,
    NextQuarter,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ExecType {
    T,
    M,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstType {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
    Any,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MgnMode {
    Cross,
    Isolated,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TdMode {
    Cross,
    Isolated,
    Cash,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdType {
    Market,
    Limit,
    PostOnly,
    Fok,
    Ioc,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PosSide {
    Long,
    Short,
    Net,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdState {
    Canceled,
    Live,
    PartiallyFilled,
    Filled,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OptType {
    C,
    P,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CtType {
    Linear,
    Inverse,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstrumentState {
    Live,
    Suspend,
    Preopen,
    Settlement,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub inst_type: InstType,
    pub inst_id: String,
    pub ccy: String,
    pub ord_id: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub cl_ord_id: Option<String>,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub tag: Option<String>,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub px: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub sz: f64,
    pub ord_type: OrdType,
    pub side: String,
    pub pos_side: String,
    pub td_mode: TdMode,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub fill_sz: f64,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub fill_px: Option<f64>,
    pub trade_id: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub acc_fill_sz: f64,
    pub fill_time: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub fill_fee: f64,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub fill_fee_ccy: Option<String>,
    #[serde(deserialize_with = "crate::parser::deserialize_str_opt")]
    pub exec_type: Option<ExecType>,
    pub state: OrdState,
    pub avg_px: String,
    pub lever: String,
    pub tp_trigger_px: String,
    pub tp_ord_px: String,
    pub sl_trigger_px: String,
    pub sl_ord_px: String,
    pub fee_ccy: String,
    pub fee: String,
    pub rebate_ccy: String,
    pub rebate: String,
    pub pnl: String,
    pub category: String,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub u_time: DateTime<Utc>,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub c_time: DateTime<Utc>,
    pub req_id: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub amend_result: Option<i64>,
    pub code: String,
    pub msg: String,
}


#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancePositionData {
    pub p_time: String,
    pub event_type: String,
    pub bal_data: Option<Vec<BalData>>,
    pub pos_data: Option<Vec<PosData>>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalData {
    pub ccy: String,
    pub cash_bal: String,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub u_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PosData {
    pub pos_id: String,
    pub trade_id: String,
    pub inst_id: String,
    pub inst_type: String,
    pub mgn_mode: String,
    pub pos_side: String,
    pub pos: String,
    pub ccy: String,
    pub pos_ccy: String,
    pub avg_px: String,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub u_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceData {
    #[serde(deserialize_with = "ts_milliseconds")]
    pub u_time: DateTime<Utc>,

    pub total_eq: String,
    pub adj_eq: String,
    pub iso_eq: String,
    pub ord_froz: String,
    pub imr: String,
    pub mmr: String,
    pub notional_usd: String,
    pub mgn_ratio: String,
    pub details: Vec<BalanceDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceDetail {
    pub avail_bal: String,
    pub avail_eq: String,
    pub ccy: String,
    pub cash_bal: String,
    pub u_time: String,
    pub dis_eq: String,
    pub eq: String,
    pub eq_usd: String,
    pub frozen_bal: String,
    pub max_loan: String,
    pub mgn_ratio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    pub inst_type: String,
    pub mgn_mode: String,
    pub pos_id: String,
    pub pos_side: String,
    pub pos: String,
    pub pos_ccy: String,
    pub avail_pos: String,
    pub avg_px: String,
    pub upl: String,
    pub upl_ratio: String,
    pub inst_id: String,
    pub lever: String,
    pub imr: String,
    pub margin: String,
    pub mgn_ratio: String,
    pub mmr: String,
    pub liab: String,
    pub liab_ccy: String,
    pub interest: String,
    pub notional_usd: String,
    pub adl: String,
    pub ccy: String,
    pub last: String,
    pub c_time: String,
    pub u_time: String,
    pub p_time: String,
}



