use crate::solution::Solution;

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        return num.trim_matches('0').to_string();
    }
}
