use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn main() {
    println!("Hello, world!");
    let stock = Stock {
        name: "Test".to_string(),
        ticker: "TTT".to_string(),
    };

    let stock_trade = StockTrade {
        price: dec!(10),
        amount: dec!(2),
        operation: Operation::BUY,
        date: Utc::now(),
        stock,
    };

    let mut portfolio = Portfolio {
        equities: vec![stock_trade.clone()],
    };

    portfolio.equities.add(stock_trade);
}

#[derive(Clone, PartialEq)]
enum Operation {
    BUY,
    SELL,
}

#[derive(Clone)]
struct Stock {
    ticker: String,
    name: String,
}

impl Stock {
    fn get_current_price(&self) -> Decimal {
        dec!(14.5)
    }
}

#[derive(Clone)]
struct StockTrade {
    stock: Stock,
    price: Decimal,
    amount: Decimal,
    operation: Operation,
    date: DateTime<Utc>,
}

struct Portfolio {
    equities: Vec<StockTrade>,
}

trait Asset {
    fn calculate_average_price(&self, stock: Stock) -> Decimal;
    fn add(&mut self, stock_trade: StockTrade) -> &Self;
}

impl Asset for Vec<StockTrade> {
    fn calculate_average_price(&self, stock: Stock) -> Decimal {
        let buy_trades = self.iter().fold((dec!(0), dec!(0)), |acc, e| {
            match e.stock.ticker == stock.ticker && e.operation == Operation::BUY {
                true => (acc.0 + e.amount * e.price, acc.1 + e.amount), // (price_sum, amount_sum)
                false => (dec!(0), dec!(0)),
            }
        });

        let (total_price, total_amount) = buy_trades;
        if total_amount == dec!(0) {
            return dec!(0);
        }

        (total_price / total_amount).round_dp(2)
    }

    fn add(&mut self, stock_trade: StockTrade) -> &Self {
        self.push(stock_trade);
        self
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

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
            portfolio.equities.calculate_average_price(setup_stock()),
            dec!(19.09)
        );
    }
}
