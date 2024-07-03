struct Solution;
// Problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_prof = 0;
        for i in 0..(prices.len() - 1) {
            let buy_price = prices[i];
            let sell_price = prices[i + 1];
            let profit = sell_price - buy_price;
            if profit >= 0 {
                max_prof += profit;
            }
        }

        max_prof
    }
}

#[cfg(test)]
mod tests {
    struct MaxProfitTestCase {
        prices: Vec<i32>,
        result: i32,
    }
    #[test]
    fn test_best_time_to_buy_and_sell_stock_ii() {
        let test_cases = [
            MaxProfitTestCase {
                prices: vec![7, 1, 5, 3, 6, 4],
                result: 7,
            },
            MaxProfitTestCase {
                prices: vec![1, 2, 3, 4, 5],
                result: 4,
            },
            MaxProfitTestCase {
                prices: vec![7, 6, 4, 3, 1],
                result: 0,
            },
            MaxProfitTestCase {
                prices: vec![7],
                result: 0,
            },
        ];

        for test in test_cases {
            assert_eq!(test.result, super::Solution::max_profit(test.prices));
        }
    }
}
