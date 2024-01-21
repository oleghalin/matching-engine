use std::collections::{HashMap, VecDeque};
use rust_decimal::prelude::*;

pub type Price = Decimal;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderSide {
    Ask,
    Bid
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order {
    pub size: u64,
    pub price: Price,
    pub side: OrderSide
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, VecDeque<Order>>,
    bids: HashMap<Price, VecDeque<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, side: OrderSide, price: Price, size: u64) {
        let order = Order {
            size,
            price,
            side: side.clone()
        };

        match side {
            OrderSide::Ask => {
                let entry = self.asks.entry(price).or_insert(VecDeque::new());
                entry.push_back(order);
            },
            OrderSide::Bid => {
                let entry = self.bids.entry(price).or_insert(VecDeque::new());
                entry.push_back(order);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_add_order() {
        let mut order_book = OrderBook::new();
        order_book.add_order(OrderSide::Ask, dec!(100), 100);
        order_book.add_order(OrderSide::Bid, dec!(100), 50);

        // Assert length of price levels
        assert_eq!(order_book.asks.len(), 1);
        assert_eq!(order_book.bids.len(), 1);

        // Assert length of orders at price level
        assert_eq!(order_book.asks.get(&dec!(100)).unwrap().len(), 1);
        assert_eq!(order_book.bids.get(&dec!(100)).unwrap().len(), 1);

        assert_eq!(order_book.asks.get(&dec!(100)).unwrap().get(0).unwrap().size, 100);
        assert_eq!(order_book.bids.get(&dec!(100)).unwrap().get(0).unwrap().size, 50);
    }
}