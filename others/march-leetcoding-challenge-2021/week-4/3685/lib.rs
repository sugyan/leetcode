pub struct Solution;

impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let char_counts = |s: &String| -> [usize; 26] {
            s.as_bytes().iter().fold([0; 26], |mut acc, &u| {
                acc[(u - b'a') as usize] += 1;
                acc
            })
        };
        let target = b.iter().map(char_counts).fold(vec![0; 26], |acc, x| {
            acc.iter()
                .enumerate()
                .map(|(i, &count)| x[i].max(count))
                .collect()
        });
        a.into_iter()
            .filter(|s| {
                char_counts(s)
                    .iter()
                    .enumerate()
                    .all(|(i, &count)| count >= target[i])
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["facebook", "google", "leetcode"],
            Solution::word_subsets(
                vec!["amazon", "apple", "facebook", "google", "leetcode"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                vec!["e", "o"].into_iter().map(str::to_string).collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec!["apple", "google", "leetcode"],
            Solution::word_subsets(
                vec!["amazon", "apple", "facebook", "google", "leetcode"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                vec!["l", "e"].into_iter().map(str::to_string).collect()
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec!["facebook", "google"],
            Solution::word_subsets(
                vec!["amazon", "apple", "facebook", "google", "leetcode"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                vec!["e", "oo"].into_iter().map(str::to_string).collect()
            )
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            vec!["google", "leetcode"],
            Solution::word_subsets(
                vec!["amazon", "apple", "facebook", "google", "leetcode"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                vec!["lo", "eo"].into_iter().map(str::to_string).collect()
            )
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            vec!["facebook", "leetcode"],
            Solution::word_subsets(
                vec!["amazon", "apple", "facebook", "google", "leetcode"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                vec!["ec", "oc", "ceo"]
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            )
        );
    }
}
