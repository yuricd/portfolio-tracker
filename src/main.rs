mod asset;
mod stock;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use stock::{Stock, StockTrade};

fn main() {
    println!("Hello, world!");
}

struct Portfolio {
    equities: Vec<StockTrade>,
}
