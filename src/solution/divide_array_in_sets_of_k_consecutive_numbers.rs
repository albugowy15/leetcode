use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        if n as i32 % k != 0 {
            return false;
        }
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(n);
        nums.sort();

        for num in nums.iter() {
            map.entry(*num).and_modify(|val| *val += 1).or_insert(1);
        }

        for num in nums.iter() {
            if map.contains_key(&num) {
                for i in (num + 1)..(num + k) {
                    match map.get_mut(&i) {
                        Some(notval) => {
                            if *notval > 1 {
                                *notval -= 1;
                            } else {
                                map.remove(&i);
                            }
                        }
                        None => return false,
                    };
                }
                if let Some(cnt) = map.get_mut(&num) {
                    if *cnt > 1 {
                        *cnt -= 1;
                    } else {
                        map.remove(&num);
                    }
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestCaseInput {
        nums: Vec<i32>,
        k: i32,
    }
    struct TestCase {
        input: TestCaseInput,
        output: bool,
    }
    #[test]
    fn test_is_possible_divide() {
        let test_cases = [
            TestCase {
                input: TestCaseInput {
                    nums: vec![1, 2, 3, 3, 4, 4, 5, 6],
                    k: 4,
                },
                output: true,
            },
            TestCase {
                input: TestCaseInput {
                    nums: vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11],
                    k: 3,
                },
                output: true,
            },
            TestCase {
                input: TestCaseInput {
                    nums: vec![1, 2, 3, 4],
                    k: 3,
                },
                output: false,
            },
        ];

        for tc in test_cases.into_iter() {
            let input = tc.input.clone();
            let result = Solution::is_possible_divide(tc.input.nums, tc.input.k);
            assert_eq!(result, tc.output, "testing {:?}", input);
        }
    }
}
