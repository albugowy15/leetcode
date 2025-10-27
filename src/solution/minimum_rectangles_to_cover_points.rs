use crate::solution::Solution;

impl Solution {
    pub fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
        // sort by x
        points.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut counter = 0;
        let mut i = 0;
        while i < points.len() {
            let start = points[i][0];
            while i + 1 < points.len() && points[i + 1][0] - start <= w {
                i += 1;
            }
            counter += 1;
            i += 1;
        }
        return counter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCaseInput {
        points: Vec<Vec<i32>>,
        w: i32,
    }
    struct TestCase {
        input: TestCaseInput,
        output: i32,
    }

    #[test]
    fn test_min_rectangles_to_cover_points() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    points: vec![
                        vec![2, 1],
                        vec![1, 0],
                        vec![1, 4],
                        vec![1, 8],
                        vec![3, 5],
                        vec![4, 6],
                    ],
                    w: 2,
                },
                output: 2,
            },
            TestCase {
                input: TestCaseInput {
                    points: vec![
                        vec![0, 0],
                        vec![1, 1],
                        vec![2, 2],
                        vec![3, 3],
                        vec![4, 4],
                        vec![5, 5],
                        vec![6, 6],
                    ],
                    w: 2,
                },
                output: 3,
            },
            TestCase {
                input: TestCaseInput {
                    points: vec![vec![2, 3], vec![1, 2]],
                    w: 0,
                },
                output: 2,
            },
        ];

        for tc in test_cases {
            assert_eq!(
                Solution::min_rectangles_to_cover_points(tc.input.points, tc.input.w),
                tc.output
            );
        }
    }
}
