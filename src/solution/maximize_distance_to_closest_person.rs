use crate::solution::Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut k = 0;
        let mut ans: i32 = 0;

        for i in 0..n {
            if seats[i] == 1 {
                k = 0;
            } else {
                k += 1;
                ans = ans.max((k + 1) / 2);
            }
        }

        for i in 0..n {
            if seats[i] == 1 {
                ans = ans.max(i as i32);
                break;
            }
        }

        for i in (0..n).rev() {
            if seats[i] == 1 {
                ans = ans.max((n as i32) - 1 - (i as i32));
                break;
            }
        }

        return ans;
    }
}

// brute force O(n * n)
// loop through each position
// calculate distance from left to right
// has a counter to store the max distance to left and right
// return the counter

// calculate distance from left and right
//
#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        input: Vec<i32>,
        output: i32,
    }

    #[test]
    fn test_max_dist_to_closest() {
        let test_cases = [
            TestCase {
                input: vec![1, 0, 0, 0, 1, 0, 1],
                output: 2,
            },
            TestCase {
                input: vec![1, 0, 0, 0],
                output: 3,
            },
            TestCase {
                input: vec![0, 1],
                output: 1,
            },
        ];

        for tc in test_cases {
            let result = Solution::max_dist_to_closest(tc.input);
            assert_eq!(result, tc.output);
        }
    }
}
