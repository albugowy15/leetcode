use crate::solution::Solution;

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut steps = 0;
        let mut remaining_capacity = capacity;
        for i in 0..plants.len() {
            remaining_capacity -= plants[i];
            steps += 1;
            if remaining_capacity < 0 {
                remaining_capacity = capacity - plants[i];
                steps += 2 * i as i32;
            }
        }
        return steps;
    }
}

#[cfg(test)]
mod test {
    struct TestCaseInput {
        plants: Vec<i32>,
        capacity: i32,
    }
    struct TestCase {
        input: TestCaseInput,
        output: i32,
    }

    use super::*;

    #[test]
    fn test_watering_plants() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    plants: vec![2, 2, 3, 3],
                    capacity: 5,
                },
                output: 14,
            },
            TestCase {
                input: TestCaseInput {
                    plants: vec![1, 1, 1, 4, 2, 3],
                    capacity: 4,
                },
                output: 30,
            },
            TestCase {
                input: TestCaseInput {
                    plants: vec![7, 7, 7, 7, 7, 7, 7],
                    capacity: 8,
                },
                output: 49,
            },
        ];

        for tc in test_cases {
            assert_eq!(
                Solution::watering_plants(tc.input.plants, tc.input.capacity),
                tc.output
            );
        }
    }
}
