pub struct Solution;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut waiting = vec![Vec::new(); 26];
        for w in &words {
            let w = w.as_bytes();
            waiting[(w[0] - b'a') as usize].push(&w[1..]);
        }
        let mut answer = 0;
        for &u in s.as_bytes() {
            for w in &waiting[(u - b'a') as usize].drain(..).collect::<Vec<_>>() {
                if w.is_empty() {
                    answer += 1;
                } else {
                    waiting[(w[0] - b'a') as usize].push(&w[1..]);
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::num_matching_subseq(
                String::from("abcde"),
                vec!["a", "bb", "acd", "ace"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::num_matching_subseq(
                String::from("dsahjpjauf"),
                vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
}
