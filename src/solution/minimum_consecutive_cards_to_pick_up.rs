use crate::solution::Solution;

use std::collections::HashMap;
impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut elm_idxs = HashMap::new();

        let mut ans: Option<i32> = None;
        for (idx, card) in cards.into_iter().enumerate() {
            if let Some(prev_idx) = elm_idxs.get_mut(&card) {
                let diff = (idx as i32 - *prev_idx as i32) + 1;
                ans = Some(ans.map_or(diff, |v| v.min(diff)));
                *prev_idx = idx;
            } else {
                elm_idxs.insert(card, idx);
            }
        }
        ans.unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCase {
        input: Vec<i32>,
        expect: i32,
    }

    #[test]
    fn test_minimum_card_pickup() {
        let test_cases = vec![
            TestCase {
                input: vec![3, 4, 2, 3, 4, 7],
                expect: 4,
            },
            TestCase {
                input: vec![1, 0, 5, 3],
                expect: -1,
            },
            TestCase {
                input: vec![3, 4, 2, 8, 9, 3],
                expect: 6,
            },
            TestCase {
                input: vec![1, 2, 3, 4, 2, 6],
                expect: 4,
            },
            TestCase {
                input: vec![
                    77, 10, 11, 51, 69, 83, 33, 94, 0, 42, 86, 41, 65, 40, 72, 8, 53, 31, 43, 22,
                    9, 94, 45, 80, 40, 0, 84, 34, 76, 28, 7, 79, 80, 93, 20, 82, 36, 74, 82, 89,
                    74, 77, 27, 54, 44, 93, 98, 44, 39, 74, 36, 9, 22, 57, 70, 98, 19, 68, 33, 68,
                    49, 86, 20, 50, 43,
                ],
                expect: 3,
            },
        ];

        for tc in test_cases {
            let ans = Solution::minimum_card_pickup(tc.input);
            assert_eq!(ans, tc.expect);
        }
    }
}
