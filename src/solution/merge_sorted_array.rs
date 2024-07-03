struct Solution;
// Problems: https://leetcode.com/problems/merge-sorted-array

#[allow(dead_code)]
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = n as usize;
        let mut k = i + j - 1;

        // merge in reverse order
        while i > 0 && j > 0 {
            let curr_nums1 = nums1[i - 1];
            let curr_nums2 = nums2[j - 1];
            if curr_nums1 > curr_nums2 {
                nums1[k] = curr_nums1;
                i -= 1;
            } else {
                nums1[k] = curr_nums2;
                j -= 1;
            }
            k -= 1;
        }

        // fill nums1 with leftover nums2 elements
        while j > 0 {
            nums1[k] = nums2[j - 1];
            j -= 1;
            k -= 1;
        }
    }
}
