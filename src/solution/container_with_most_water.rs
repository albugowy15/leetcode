use super::Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left_pt, mut right_pt) = (0, height.len() - 1);
        let mut max_area = 0;

        while left_pt < right_pt {
            let (x, y) = (
                (right_pt - left_pt) as i32,
                height[left_pt].min(height[right_pt]),
            );
            max_area = max_area.max(x * y);

            if height[right_pt] > height[left_pt] {
                left_pt += 1;
            } else {
                right_pt -= 1;
            }
        }
        max_area
    }
}

#[test]
fn test_max_area() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(49, Solution::max_area(height));

    let height = vec![1, 1];
    assert_eq!(1, Solution::max_area(height));
}
