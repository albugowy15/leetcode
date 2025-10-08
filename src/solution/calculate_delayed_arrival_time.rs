use crate::solution::Solution;

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        let result = arrival_time + delayed_time % 24;
        if result >= 24 {
            return result % 24;
        } else {
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    struct TestCaseInput {
        arrival_time: i32,
        delayed_time: i32,
    }

    struct TestCase {
        input: TestCaseInput,
        output: i32,
    }
    #[test]
    fn test_find_delayed_arrival_time() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    arrival_time: 15,
                    delayed_time: 5,
                },
                output: 20,
            },
            TestCase {
                input: TestCaseInput {
                    arrival_time: 13,
                    delayed_time: 11,
                },
                output: 0,
            },
        ];

        for tc in test_cases {
            assert_eq!(
                Solution::find_delayed_arrival_time(tc.input.arrival_time, tc.input.delayed_time),
                tc.output
            );
        }
    }
}
