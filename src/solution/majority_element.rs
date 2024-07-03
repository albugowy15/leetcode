struct Solution;
// Problem: https://leetcode.com/problems/majority-element

impl Solution {
    // pub fn majority_element(nums: Vec<i32>) -> i32 {
    //     let mut freq: HashMap<i32, i32> = HashMap::new();
    //     let min_appearance: i32 = nums.len() as i32 / 2;
    //     for num in nums {
    //         match freq.get_mut(&num) {
    //             Some(counter) => {
    //                 *counter += 1;
    //                 if *counter > min_appearance {
    //                     return num;
    //                 }
    //             }
    //             None => {
    //                 freq.insert(num, 1);
    //                 if 1 > min_appearance {
    //                     return num;
    //                 }
    //             }
    //         }
    //     }
    //     -1
    // }
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut counter, mut res) = (0, 0);
        nums.iter().for_each(|num| {
            if counter == 0 {
                res = *num;
            }
            match num.cmp(&res) {
                std::cmp::Ordering::Equal => counter += 1,
                _ => counter -= 1,
            }
        });
        res
    }
}

#[cfg(test)]
mod tests {
    struct MajorityElementTestCase {
        nums: Vec<i32>,
        output: i32,
    }
    #[test]
    fn test_majority_element() {
        let test_cases = vec![
            MajorityElementTestCase {
                nums: vec![3, 2, 3],
                output: 3,
            },
            MajorityElementTestCase {
                nums: vec![2, 2, 1, 1, 1, 2, 2],
                output: 2,
            },
        ];

        for test in test_cases.into_iter() {
            assert_eq!(test.output, super::Solution::majority_element(test.nums));
        }
    }
}
