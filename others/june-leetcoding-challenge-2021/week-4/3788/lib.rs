pub struct Solution;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let s = s.as_bytes();
        let words = words.iter().map(|w| w.as_bytes()).collect::<Vec<_>>();
        let mut nexts = vec![Vec::new(); 26];
        let mut index = vec![0; words.len()];
        for (i, w) in words.iter().enumerate() {
            nexts[(w[0] - b'a') as usize].push(i);
        }
        let mut answer = 0;
        for u in s {
            let v = nexts[(u - b'a') as usize].drain(..).collect::<Vec<_>>();
            for &i in &v {
                index[i] += 1;
                if index[i] == words[i].len() {
                    answer += 1;
                } else {
                    nexts[(words[i][index[i]] - b'a') as usize].push(i);
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
