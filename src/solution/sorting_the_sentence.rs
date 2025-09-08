use crate::solution::Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words: Vec<&str> = s.split_whitespace().collect();
        words.sort_by_key(|word| word.chars().last().unwrap());
        return words
            .iter()
            .map(|word| &word[..word.len() - 1])
            .collect::<Vec<&str>>()
            .join(" ");
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestSet {
        s: String,
        output: String,
    }

    #[test]
    fn test_sort_sentence() {
        let test_sets = vec![
            TestSet {
                s: String::from("is2 sentence4 This1 a3"),
                output: String::from("This is a sentence"),
            },
            TestSet {
                s: String::from("Myself2 Me1 I4 and3"),
                output: String::from("Me Myself and I"),
            },
        ];

        for tc in test_sets {
            assert_eq!(Solution::sort_sentence(tc.s), tc.output);
        }
    }
}
