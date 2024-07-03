struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique_ptr = 1;
        let mut unique_counter = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[unique_ptr] = nums[i];
                unique_ptr += 1;
                unique_counter = 1;
            } else if unique_counter < 2 {
                nums[unique_ptr] = nums[i];
                unique_ptr += 1;
                unique_counter += 1;
            }
        }
        return unique_ptr as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_from_sorted_array_ii() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let expected_k = 5;
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(expected_k, k);

        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let expected_k = 7;
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(expected_k, k);
    }
}
