use std::collections::HashMap;
use crate::match_engine::order_book::OrderBook;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pair {
    base: String,
    quote: String,
}

impl Pair {
    pub fn new(base: &str, quote: &str) -> Self {
        Self {
            base: base.to_string(),
            quote: quote.to_string(),
        }
    }
}

pub struct Engine {
    order_book: HashMap<Pair, OrderBook>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            order_book: HashMap::new(),
        }
    }

    pub fn add_market(&mut self, base: &str, quote: &str) {
        let pair = Pair::new(base, quote);

        self.order_book.insert(pair, OrderBook::new());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_market() {
        let mut engine = Engine::new();

        engine.add_market("BTC", "USDT");

        assert_eq!(engine.order_book.len(), 1);
    }
}