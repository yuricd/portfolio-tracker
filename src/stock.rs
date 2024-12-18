use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::asset::Asset;

#[derive(Clone, PartialEq)]
enum Operation {
    BUY,
    SELL,
}

#[derive(Clone)]
pub struct Stock {
    ticker: String,
    name: String,
}

impl Stock {
    fn get_current_price(&self) -> Decimal {
        dec!(14.5)
    }
}

#[derive(Clone)]
pub struct StockTrade {
    stock: Stock,
    price: Decimal,
    amount: Decimal,
    operation: Operation,
    date: DateTime<Utc>,
}

impl Asset for Vec<StockTrade> {
    fn add(&mut self, stock_trade: StockTrade) -> &Self {
        self.push(stock_trade);
        self
    }

    fn calculate_average_price(&self, stock: &Stock) -> Decimal {
        let buy_trades = self.iter().fold((dec!(0), dec!(0)), |acc, e| {
            match e.stock.ticker == stock.ticker && e.operation == Operation::BUY {
                true => (acc.0 + e.amount * e.price, acc.1 + e.amount), // (price_sum, amount_sum)
                false => (acc.0, acc.1),
            }
        });

        let (total_price, total_amount) = buy_trades;
        match total_amount {
            amount if amount.is_zero() => dec!(0),
            _ => (total_price / total_amount).round_dp(2),
        }
    }

    fn available(&self, stock: Stock) -> Decimal {
        let buy_trades_amount = self.iter().fold(dec!(0), |acc, trade| {
            match trade.stock.ticker == stock.ticker && trade.operation == Operation::BUY {
                true => acc + trade.amount,
                false => acc,
            }
        });

        let sell_trades_amount = self.iter().fold(dec!(0), |acc, trade| {
            match trade.stock.ticker == stock.ticker && trade.operation == Operation::SELL {
                true => acc + trade.amount,
                false => acc,
            }
        });
        buy_trades_amount - sell_trades_amount
    }

    fn calculate_profit(&self, stock: Stock, amount: Decimal, unit_sell_price: Decimal) -> Decimal {
        let average_unit_buy_price = self.calculate_average_price(&stock);
        let buy_price = average_unit_buy_price * amount;
        let sell_price = unit_sell_price * amount;
        let profit = sell_price - buy_price;
        profit
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::Portfolio;

    use super::*;

    fn setup_stock() -> Stock {
        Stock {
            name: "Test".to_string(),
            ticker: "TTT".to_string(),
        }
    }

    fn setup_trade(price: Decimal, amount: Decimal) -> StockTrade {
        StockTrade {
            price,
            amount,
            operation: Operation::BUY,
            date: Utc::now(),
            stock: setup_stock(),
        }
    }

    #[test]
    fn test_add() {
        let stock_trade = setup_trade(dec!(14.56), dec!(2));

        let mut portfolio = Portfolio {
            equities: vec![stock_trade.clone()],
        };

        assert_eq!(portfolio.equities.len(), 1);

        portfolio.equities.add(stock_trade.clone());
        assert_eq!(portfolio.equities.len(), 2);
    }

    #[test]
    fn test_calculate_average_price() {
        let stock_trade_1 = setup_trade(dec!(14.56), dec!(2));
        let stock_trade_2 = setup_trade(dec!(20), dec!(10));

        let mut portfolio = Portfolio { equities: vec![] };

        portfolio.equities.add(stock_trade_1.clone());
        portfolio.equities.add(stock_trade_2.clone());

        assert_eq!(
            portfolio.equities.calculate_average_price(&setup_stock()),
            dec!(19.09)
        );
    }

    #[test]
    fn test_available() {
        let _1_buy_trade_1 = StockTrade {
            amount: dec!(10),
            price: dec!(4.99),
            operation: Operation::BUY,
            stock: setup_stock(),
            date: Utc::now(),
        };

        let _2_sell_trade_1 = StockTrade {
            amount: dec!(8),
            price: dec!(5.20),
            operation: Operation::SELL,
            stock: setup_stock(),
            date: Utc::now(),
        };

        let _3_buy_trade_2 = StockTrade {
            amount: dec!(15),
            price: dec!(4.90),
            operation: Operation::BUY,
            stock: setup_stock(),
            date: Utc::now(),
        };

        let mut portfolio = Portfolio { equities: vec![] };

        portfolio.equities.add(_1_buy_trade_1.clone());
        portfolio.equities.add(_2_sell_trade_1.clone());
        portfolio.equities.add(_3_buy_trade_2.clone());

        assert_eq!(portfolio.equities.available(setup_stock()), dec!(17));
    }

    #[test]
    fn test_calculate_profit() {
        let _1_buy_trade_1 = StockTrade {
            amount: dec!(300),
            price: dec!(10),
            operation: Operation::BUY,
            stock: setup_stock(),
            date: Utc::now(),
        };

        let _2_sell_trade_1 = StockTrade {
            amount: dec!(200),
            price: dec!(12),
            operation: Operation::BUY,
            stock: setup_stock(),
            date: Utc::now(),
        };
        // buy average price: 10.8

        let mut portfolio = Portfolio { equities: vec![] };

        portfolio.equities.add(_1_buy_trade_1.clone());
        portfolio.equities.add(_2_sell_trade_1.clone());

        assert_eq!(
            portfolio
                .equities
                .calculate_profit(setup_stock(), dec!(300), dec!(15)),
            dec!(1260)
        );
    }
}
