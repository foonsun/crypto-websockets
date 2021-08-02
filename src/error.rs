use serde::{Deserialize, Serialize};
use snafu::*;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Deserialize, Serialize, Debug, Clone, Snafu)]
pub enum Error {
    #[snafu(display("Huobi Future error: {}: {}", code, msg))]
    HuobiError { code: i64, msg: String },
    #[snafu(display("Assets not found"))]
    AssetsNotFound,
    #[snafu(display("Symbol not found"))]
    SymbolNotFound,
    #[snafu(display("No Api key set for private api"))]
    NoApiKeySet,
    #[snafu(display("No stream is subscribed"))]
    NoStreamSubscribed,
}
