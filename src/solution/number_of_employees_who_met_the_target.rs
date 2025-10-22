use crate::solution::Solution;

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        return hours
            .into_iter()
            .filter(|hour| *hour >= target)
            .count()
            .try_into()
            .unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[derive(Debug, Clone)]
    struct TestCaseInput {
        hours: Vec<i32>,
        target: i32,
    }

    struct TestCase {
        input: TestCaseInput,
        expected: i32,
    }

    #[test]
    fn test_number_of_employees_who_met_target() {
        let tests = vec![
            TestCase {
                input: TestCaseInput {
                    hours: vec![0, 1, 2, 3, 4],
                    target: 2,
                },
                expected: 3,
            },
            TestCase {
                input: TestCaseInput {
                    hours: vec![5, 1, 4, 2, 2],
                    target: 6,
                },
                expected: 0,
            },
        ];

        for tc in tests {
            assert_eq!(
                Solution::number_of_employees_who_met_target(
                    tc.input.clone().hours,
                    tc.input.target
                ),
                tc.expected,
                "Test case: {:?}",
                tc.input
            )
        }
    }
}
