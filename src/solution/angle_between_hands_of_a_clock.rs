use crate::solution::Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        // The minute hand moves 360° in 60 minutes
        // So it moves 6° per minute
        let minute_angle = minutes as f64 * 6.0;

        // Base movement: 360° ÷ 12 hours = 30° per hour
        // Continuous movement: The hour hand also moves as minutes pass
        // Minute adjustment: 30° ÷ 60 minutes = 0.5° per minute
        let hour_angle = (hour % 12) as f64 * 30.0 + minutes as f64 * 0.5;

        let diff = (hour_angle - minute_angle).abs();

        return diff.min(360.0 - diff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        hour: i32,
        minutes: i32,
        expected: f64,
    }

    #[test]
    fn test_angle_clock() {
        let test_cases = vec![
            TestCase {
                hour: 12,
                minutes: 30,
                expected: 165.0,
            },
            TestCase {
                hour: 3,
                minutes: 30,
                expected: 75.0,
            },
            TestCase {
                hour: 3,
                minutes: 15,
                expected: 7.5,
            },
            TestCase {
                hour: 4,
                minutes: 50,
                expected: 155.0,
            },
            TestCase {
                hour: 12,
                minutes: 0,
                expected: 0.0,
            },
        ];

        for case in test_cases {
            let result = Solution::angle_clock(case.hour, case.minutes);
            assert!(
                (result - case.expected).abs() < 1e-5,
                "Failed for hour: {}, minutes: {}. Expected {}, got {}",
                case.hour,
                case.minutes,
                case.expected,
                result
            );
        }
    }
}
