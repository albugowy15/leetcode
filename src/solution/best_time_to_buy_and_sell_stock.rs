use crate::solution::Solution;
use std::cmp;

// Problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy, mut sell, mut max) = (0, 1, 0);

        while sell < prices.len() {
            let (buy_price, sell_price) = (prices[buy], prices[sell]);
            match buy_price.cmp(&sell_price) {
                cmp::Ordering::Greater => buy += 1,
                _ => {
                    let profit = sell_price - buy_price;
                    max = max.max(profit);
                    sell += 1;
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    struct MaxProfitTestCase {
        prices: Vec<i32>,
        res: i32,
    }
    #[test]
    fn test_max_profit() {
        let test_cases = [
            MaxProfitTestCase {
                prices: vec![7, 1, 5, 3, 6, 4],
                res: 5,
            },
            MaxProfitTestCase {
                prices: vec![7, 6, 4, 3, 1],
                res: 0,
            },
            MaxProfitTestCase {
                prices: vec![7],
                res: 0,
            },
            MaxProfitTestCase {
                prices: vec![7, 5],
                res: 0,
            },
        ];

        for test in test_cases {
            assert_eq!(test.res, super::Solution::max_profit(test.prices));
        }
    }
}
