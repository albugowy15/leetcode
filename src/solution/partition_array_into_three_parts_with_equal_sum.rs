use crate::solution::Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let total_sum: i32 = arr.iter().sum();
        if total_sum % 3 != 0 && total_sum != 0 {
            return false;
        }
        let sub_sum = total_sum / 3;
        let mut current_sum = 0;
        let mut counter = 0;
        for elm in arr {
            current_sum += elm;
            if current_sum == sub_sum {
                counter += 1;
                current_sum = 0;
            }
        }
        return counter >= 3;
    }
}

// greedy approach
// calculate sum from all elements
// divide by 3
// if cannot divided by 3 return false
// required_sum = total_Sum / 2;
// required sum adalah sum yang harus dicapai untuk setiap subarray
//
//

#[cfg(test)]
mod tests {
    use super::*;
    struct CanThreePartsEqualSum {
        input: Vec<i32>,
        output: bool,
    }

    #[test]
    fn test_can_three_parts_equal_sum() {
        let test_cases = vec![
            CanThreePartsEqualSum {
                input: vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1],
                output: true,
            },
            CanThreePartsEqualSum {
                input: vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1],
                output: false,
            },
            CanThreePartsEqualSum {
                input: vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4],
                output: true,
            },
            CanThreePartsEqualSum {
                input: vec![0, 0, 0, 0],
                output: true,
            },
        ];

        for test in test_cases {
            let result = Solution::can_three_parts_equal_sum(test.input);
            assert_eq!(result, test.output);
        }
    }
}
