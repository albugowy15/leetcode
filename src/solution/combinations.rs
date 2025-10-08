use crate::solution::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(n as usize * k as usize);
        for i in 1..n {
            let mut temp = Vec::new();
            for j in (i + 1)..=n {
                temp.push(j)
            }
            ans.push(temp);
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCaseInput {
        n: i32,
        k: i32,
    }

    struct TestCase {
        input: TestCaseInput,
        output: Vec<Vec<i32>>,
    }

    #[test]
    fn test_combine() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput { n: 4, k: 2 },
                output: vec![
                    [1, 2].to_vec(),
                    [1, 3].to_vec(),
                    [1, 4].to_vec(),
                    [2, 3].to_vec(),
                    [2, 4].to_vec(),
                    [3, 4].to_vec(),
                ],
            },
            TestCase {
                input: TestCaseInput { n: 1, k: 1 },
                output: vec![
                    [1, 2].to_vec(),
                    [1, 3].to_vec(),
                    [1, 4].to_vec(),
                    [2, 3].to_vec(),
                    [2, 4].to_vec(),
                    [3, 4].to_vec(),
                ],
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::combine(tc.input.n, tc.input.k), tc.output);
        }
    }
}
