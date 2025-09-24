use crate::solution::Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        let mut i = 0;
        while i < n {
            let end = std::cmp::min(i + k, n);
            chars[i..end].reverse();
            i += 2 * k;
        }

        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_str() {
        let test_cases = vec![
            ("abcdefg".to_string(), 2, "bacdfeg".to_string()),
            ("abcd".to_string(), 2, "bacd".to_string()),
            ("a".to_string(), 2, "a".to_string()),
            ("".to_string(), 2, "".to_string()),
            ("abcdefgh".to_string(), 3, "cbadefhg".to_string()),
        ];

        for (s, k, expected) in test_cases {
            assert_eq!(Solution::reverse_str(s.clone(), k), expected);
        }
    }
}
