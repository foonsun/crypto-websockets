use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Empty { }

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u16,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_precision: u64,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filters>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filters {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: String,
        max_price: String,
        tick_size: String,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        notional: Option<String>,
        min_notional: Option<String>,
        apply_to_market: Option<bool>,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: Option<u16> },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: Option<u16> },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: Option<u16> },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u16 },
    #[serde(rename = "MAX_POSITION")]
    #[serde(rename_all = "camelCase")]
    MaxPosition { max_position: String },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: f32,
    pub taker_commission: f32,
    pub buyer_commission: f32,
    pub seller_commission: f32,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub iceberg_qty: String,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub orig_quote_order_qty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled {
    pub symbol: String,
    pub orig_client_order_id: Option<String>,
    pub order_id: Option<u64>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: Option<i64>,
    pub client_order_id: String,
    pub transact_time: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    pub fills: Option<Vec<FillInfo>>,
}

fn default_stop_price() -> f64 {
    0.0
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FillInfo {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub commission: f64,
    pub commission_asset: String,
    pub trade_id: Option<u64>,
}
/// Response to a test order (endpoint /api/v3/order/test).
///
/// Currently, the API responds {} on a successfull test transaction,
/// hence this struct has no fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

impl Bids {
    pub fn new(price: f64, qty: f64) -> Bids {
        Bids { 
            price, 
            qty,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream {
    pub listen_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Success {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices {
    AllPrices(Vec<SymbolPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AveragePrice {
    pub mins: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<Tickers>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KlineSummaries {
    AllKlineSummaries(Vec<KlineSummary>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_qty: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub id: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    pub commission: String,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "string_or_float")]
    pub prev_close_price: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub match_time: u64,

    #[serde(rename = "a")]
    pub account_event: EventBalance, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventBalance {
    #[serde(rename = "m")]
    pub event: String,
    #[serde(rename = "B")]
    pub balance: Vec<BalanceItem>,
    #[serde(rename = "P")]
    pub position: Option<Vec<PositionItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceItem {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb")]
    pub balance: String,
    #[serde(rename = "cw")] 
    pub cross_balance: String,
    #[serde(rename = "bc")]
    pub balance_change: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionItem {
    #[serde(rename = "s")]
    pub asset: String,
    #[serde(rename = "pa")]
    pub position_amount: String,
    #[serde(rename = "ep")]
    pub entry_price: String,
    #[serde(rename = "cr")]
    pub accumulated_realized: String,
    #[serde(rename = "up")]
    pub unrealized_pnl: String,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw")]
    pub isolated_wallet: String,
    #[serde(rename = "ps")]
    pub position_side: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderTradeEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transact_time: u64,

    #[serde(rename = "o")]
    pub order: OrderItem,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderItem {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub client_order_id: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "o")]
    pub order_type: String,

    #[serde(rename = "f")]
    pub time_in_force: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "ap")]
    pub avg_price: String,

    #[serde(rename = "sp")]
    pub stop_price: String,

    #[serde(rename = "x")]
    pub execute_type: String,

    #[serde(rename = "X")]
    pub order_status: String,

    #[serde(rename = "i")]
    pub order_id: u64,

    #[serde(rename = "l")]
    pub order_last_filled_quantity: String,

    #[serde(rename = "z")]
    pub order_filled_accumulated_quantity: String,

    #[serde(rename = "L")]
    pub order_last_filled_price: String,

    #[serde(rename = "T")]
    pub order_trade_time: u64,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(rename = "R")]
    pub is_reduced: bool,

    #[serde(rename = "ot")]
    pub original_order_type: String,

    #[serde(rename = "ps")]
    pub position_side: String,

}

/// The Aggregate Trade Streams push trade information that is aggregated for a single taker order.
///
/// Stream Name: \<symbol\>@aggTrade
///
/// Update Speed: Real-time
///
/// https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#aggregate-trade-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggrTradesEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub aggregated_trade_id: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "f")]
    pub first_break_trade_id: u64,

    #[serde(rename = "l")]
    pub last_break_trade_id: u64,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

/// The Trade Streams push raw trade information; each trade has a unique buyer and seller.
///
/// Stream Name: \<symbol\>@trade
///
/// Update Speed: Real-time
///
/// https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#trade-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "b")]
    pub buyer_order_id: u64,

    #[serde(rename = "a")]
    pub seller_order_id: u64,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "i")]
    pub pair: String,

    #[serde(rename = "p")]
    pub price: String,

}
// https://binance-docs.github.io/apidocs/futures/en/#mark-price-stream
// https://binance-docs.github.io/apidocs/delivery/en/#mark-price-stream
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceEvent {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "P")]
    pub estimate_settle_price: String,

    #[serde(rename = "T")]
    pub next_funding_time: u64,

    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "i")]
    pub index_price: Option<String>,

    #[serde(rename = "p")]
    pub mark_price: String,

    #[serde(rename = "r")]
    pub funding_rate: String,

    #[serde(rename = "s")]
    pub symbol: String,
}


// Object({"E": Number(1626118018407), "e": String("forceOrder"), "o": Object({"S": String("SELL"), "T": Number(1626118018404), "X": String("FILLED"), "ap": String("33028.07"), "f": String("IOC"), "l": String("0.010"), "o": String("LIMIT"), "p": String("32896.00"), "q": String("0.010"), "s": String("BTCUSDT"), "z": String("0.010")})})
// https://binance-docs.github.io/apidocs/futures/en/#liquidation-order-streams

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "o")]
    pub liquidation_order: LiquidationOrder,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "o")]
    pub order_type: String,

    #[serde(rename = "f")]
    pub time_in_force: String,

    #[serde(rename = "q")]
    pub original_quantity: String,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "ap")]
    pub average_price: String,

    #[serde(rename = "X")]
    pub order_status: String,

    #[serde(rename = "l")]
    pub order_last_filled_quantity: String,

    #[serde(rename = "z")]
    pub order_filled_accumulated_quantity: String,

    #[serde(rename = "T")]
    pub order_trade_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerEvent {
    #[serde(rename = "u")]
    pub update_id: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "b")]
    pub best_bid: String,

    #[serde(rename = "B")]
    pub best_bid_qty: String,

    #[serde(rename = "a")]
    pub best_ask: String,

    #[serde(rename = "A")]
    pub best_ask_qty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price_change: String,

    #[serde(rename = "P")]
    pub price_change_percent: String,

    #[serde(rename = "w")]
    pub average_price: String,

    #[serde(rename = "x")]
    pub prev_close: String,

    #[serde(rename = "c")]
    pub current_close: String,

    #[serde(rename = "Q")]
    pub current_close_qty: String,

    #[serde(rename = "b")]
    pub best_bid: String,

    #[serde(rename = "B")]
    pub best_bid_qty: String,

    #[serde(rename = "a")]
    pub best_ask: String,

    #[serde(rename = "A")]
    pub best_ask_qty: String,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "q")]
    pub quote_volume: String,

    #[serde(rename = "O")]
    pub open_time: u64,

    #[serde(rename = "C")]
    pub close_time: u64,

    #[serde(rename = "F")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "q")]
    pub quote_volume: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub kline: Kline,
}

// https://binance-docs.github.io/apidocs/futures/en/#continuous-contract-kline-candlestick-streams

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "ps")]
    pub pair: String,

    #[serde(rename = "ct")]
    pub contract_type: String,

    #[serde(rename = "k")]
    pub kline: ContinuousKline,
}


// https://binance-docs.github.io/apidocs/delivery/en/#index-kline-candlestick-streams

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexKlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "ps")]
    pub pair: String,

    #[serde(rename = "k")]
    pub kline: IndexKline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineSummary {
    pub open_time: i64,

    pub open: f64,

    pub high: f64,

    pub low: f64,

    pub close: f64,

    pub volume: f64,

    pub close_time: i64,

    pub quote_asset_volume: f64,

    pub number_of_trades: i64,

    pub taker_buy_base_asset_volume: f64,

    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: i64,

    #[serde(rename = "T")]
    pub end_time: i64,
    
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i32,

    #[serde(rename = "L")]
    pub last_trade_id: i32,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: i32,

    #[serde(rename = "x")]
    pub is_final_bar: bool,

    #[serde(rename = "q")]
    pub quote_volume: String,

    #[serde(rename = "V")]
    pub active_buy_volume: String,

    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,

    #[serde(skip, rename = "B")]
    pub ignore_me: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKline {
    #[serde(rename = "t")]
    pub start_time: i64,

    #[serde(rename = "T")]
    pub end_time: i64,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: i64,

    #[serde(rename = "x")]
    pub is_final_bar: bool,

    #[serde(rename = "q")]
    pub quote_volume: String,

    #[serde(rename = "V")]
    pub active_buy_volume: String,

    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,

    #[serde(skip, rename = "B")]
    pub ignore_me: String,
}

// https://binance-docs.github.io/apidocs/delivery/en/#index-kline-candlestick-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexKline {
    #[serde(rename = "t")]
    pub start_time: i64,

    #[serde(rename = "T")]
    pub end_time: i64,

    #[serde(skip, rename = "s")]
    pub ignore_me: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: i64,

    #[serde(rename = "x")]
    pub is_final_bar: bool,

    #[serde(skip, rename = "q")]
    pub ignore_me2: String,

    #[serde(skip, rename = "V")]
    pub ignore_me3: String,

    #[serde(skip, rename = "Q")]
    pub ignore_me4: String,

    #[serde(skip, rename = "B")]
    pub ignore_me5: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthOrderBookEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "U")]
    pub first_update_id: u64,

    #[serde(rename = "u")]
    pub final_update_id: u64,

    #[serde(rename = "pu")]
    #[serde(default)]
    pub previous_final_update_id: Option<u64>,

    #[serde(rename = "b")]
    pub bids: Vec<Bids>,

    #[serde(rename = "a")]
    pub asks: Vec<Asks>,
}

/// Response to the Savings API get all coins request
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub coin: String,
    pub deposit_all_enable: bool,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub freeze: f64,
    #[serde(with = "string_or_float")]
    pub ipoable: f64,
    #[serde(with = "string_or_float")]
    pub ipoing: f64,
    pub is_legal_money: bool,
    #[serde(with = "string_or_float")]
    pub locked: f64,
    pub name: String,
    pub network_list: Vec<Network>,
    #[serde(with = "string_or_float")]
    pub storage: f64,
    pub trading: bool,
    pub withdraw_all_enable: bool,
    #[serde(with = "string_or_float")]
    pub withdrawing: f64,
}

/// Part of the Savings API get all coins response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub address_regex: String,
    pub coin: String,
    /// shown only when "depositEnable" is false.
    pub deposit_desc: Option<String>,
    pub deposit_enable: bool,
    pub is_default: bool,
    pub memo_regex: String,
    /// min number for balance confirmation
    pub min_confirm: u32,
    pub name: String,
    pub network: String,
    pub reset_address_status: bool,
    pub special_tips: Option<String>,
    /// confirmation number for balance unlock
    pub un_lock_confirm: u32,
    /// shown only when "withdrawEnable" is false.
    pub withdraw_desc: Option<String>,
    pub withdraw_enable: bool,
    #[serde(with = "string_or_float")]
    pub withdraw_fee: f64,
    #[serde(with = "string_or_float")]
    pub withdraw_min: f64,
    // pub insert_time: Option<u64>, //commented out for now, because they are not inside the actual response (only the api doc example)
    // pub update_time: Option<u64>,
    pub withdraw_integer_multiple: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetail {
    #[serde(with = "string_or_float")]
    pub min_withdraw_amount: f64,
    /// false if ALL of networks' are false
    pub deposit_status: bool,
    #[serde(with = "string_or_float")]
    pub withdraw_fee: f64,
    /// false if ALL of networks' are false
    pub withdraw_status: bool,
    /// reason
    pub deposit_tip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EtpNavEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "n")]
    pub nav: f64,

    #[serde(rename = "l")]
    pub leverage: f64,

    #[serde(rename = "t")]
    pub to: f64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EtpKlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: EtpNavKline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EtpNavKline {
    #[serde(rename = "t")]
    pub kline_start_ts: u64,
    #[serde(rename = "T")]
    pub kline_end_ts: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_nav_ts: u64,
    #[serde(rename = "L")] 
    pub last_nav_ts: u64,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub actual_leverage: String,
    #[serde(rename = "n")]
    pub nav_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositAddress {
    pub address: String,
    pub coin: String,
    pub tag: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListenKeyEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarginCallEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "cw")]
    pub cross_balance: String,
    #[serde(rename = "p")]
    pub position: Vec<MarginPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarginPosition {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(rename = "pa")]
    pub position_amount: String,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw")]
    pub isolated_balance: String,
    #[serde(rename = "mp")]
    pub mark_price: String,
    #[serde(rename = "up")]
    pub unrealized_pnl: String,
    #[serde(rename = "mm")]
    pub maintenance_margin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountConfigEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transact_time: u64,
    #[serde(rename = "ac")]
    pub account_config: Option<AccountConfig>,
    #[serde(rename = "ai")]
    pub multi_asset: Option<MultiAssetConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountConfig {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub leverage: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiAssetConfig {
    #[serde(rename = "j")]
    pub multi_asset_mode: bool,
}

pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            },
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => crate::binance_uswap_model::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        Ok(Some(crate::binance_uswap_model::string_or_float::deserialize(deserializer)?))
    }
}

pub(crate) mod string_or_bool {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: fmt::Display,
            S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Bool(bool),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Bool(i) => Ok(i),
        }
    }
}