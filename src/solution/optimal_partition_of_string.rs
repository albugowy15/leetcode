// Problem: https://leetcode.com/optimal-partition-of-string
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut map: HashMap<char, bool> = HashMap::new();
        let mut counter = 1;
        for c in s.chars() {
            if map.contains_key(&c) {
                counter += 1;
                map.clear();
            }
            map.insert(c, true);
        }

        return counter;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    pub fn test_partition_string() {
        struct PartitionStringTestCase {
            input: String,
            output: i32,
        }
        let test_cases: Vec<PartitionStringTestCase> = vec![
            PartitionStringTestCase {
                input: "abacaba".to_string(),
                output: 4,
            },
            PartitionStringTestCase {
                input: "ssssss".to_string(),
                output: 6,
            },
        ];

        for test in test_cases.into_iter() {
            let output = Solution::partition_string(test.input);
            assert_eq!(test.output, output);
        }
    }
}
