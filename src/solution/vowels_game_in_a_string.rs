use super::Solution;

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        let total_vowels = s
            .chars()
            .filter(|ch| return *ch == 'a' || *ch == 'i' || *ch == 'u' || *ch == 'e' || *ch == 'o')
            .count();

        total_vowels != 0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    struct DoesAliceWinTestCase {
        input: String,
        output: bool,
    }

    #[test]
    fn test_does_alice_win() {
        let test_cases = vec![
            DoesAliceWinTestCase {
                input: String::from("leetcoder"),
                output: true,
            },
            DoesAliceWinTestCase {
                input: String::from("bbcd"),
                output: false,
            },
        ];

        for test in test_cases {
            let result = Solution::does_alice_win(test.input);
            assert_eq!(result, test.output);
        }
    }
}
