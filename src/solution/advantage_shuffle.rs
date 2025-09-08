use crate::solution::Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums1 = nums1;
        sorted_nums1.sort();
        let mut sorted_nums2 = nums2.clone();
        sorted_nums2.sort();

        let mut assigned: HashMap<i32, VecDeque<i32>> = HashMap::new();
        nums2.iter().for_each(|b| {
            assigned.entry(*b).or_insert_with(VecDeque::new);
        });

        let mut remaining: VecDeque<i32> = VecDeque::new();

        let mut j = 0;
        sorted_nums1.iter().for_each(|a| {
            if *a > sorted_nums2[j] && j < sorted_nums2.len() {
                if let Some(mut_val) = assigned.get_mut(&sorted_nums2[j]) {
                    mut_val.push_back(*a);
                    j += 1;
                }
            } else {
                remaining.push_back(*a);
            }
        });

        let mut ans: Vec<i32> = vec![0; nums2.len()];
        for i in 0..nums2.len() {
            if let Some(ass_val) = assigned.get(&nums2[i]) {
                if ass_val.len() > 0 {
                    if let Some(mut_ass) = assigned.get_mut(&nums2[i]) {
                        if let Some(pop_ok) = mut_ass.pop_front() {
                            ans[i] = pop_ok;
                        }
                    }
                } else {
                    if let Some(pop_remaing) = remaining.pop_front() {
                        ans[i] = pop_remaing;
                    }
                }
            }
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    struct TestCaseInput {
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    }
    struct TestCase {
        input: TestCaseInput,
        output: Vec<i32>,
    }

    use super::Solution;

    #[test]
    fn test_advantage_count() {
        let test_cases = [
            TestCase {
                input: TestCaseInput {
                    nums1: vec![2, 7, 11, 15],
                    nums2: vec![1, 10, 4, 11],
                },
                output: vec![2, 11, 7, 15],
            },
            TestCase {
                input: TestCaseInput {
                    nums1: vec![12, 24, 8, 32],
                    nums2: vec![13, 25, 32, 11],
                },
                output: vec![24, 32, 8, 12],
            },
        ];

        for tc in test_cases.into_iter() {
            let result = Solution::advantage_count(tc.input.nums1, tc.input.nums2);
            assert_eq!(result, tc.output);
        }
    }
}
