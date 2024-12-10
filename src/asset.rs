use rust_decimal::Decimal;

use crate::stock::{Stock, StockTrade};

pub trait Asset {
    fn add(&mut self, stock_trade: StockTrade) -> &Self;
    fn calculate_average_price(&self, stock: &Stock) -> Decimal;
    fn available(&self, stock: Stock) -> Decimal;
    fn calculate_profit(&self, stock: Stock, amount: Decimal, unit_sell_price: Decimal) -> Decimal;
}
