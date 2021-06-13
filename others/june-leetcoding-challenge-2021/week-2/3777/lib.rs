use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let hm = words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect::<HashMap<_, _>>();
        let mut answer = Vec::new();
        for (i, word) in words.iter().enumerate() {
            if let Some(&j) = hm.get(&word.chars().rev().collect::<String>()) {
                if i != j {
                    answer.push([i as i32, j as i32].to_vec());
                }
            }
            {
                let w = word.chars().rev().collect::<Vec<_>>();
                for k in 0..w.len() {
                    if (0..=k / 2).all(|l| w[l] == w[k - l]) {
                        if let Some(&j) = hm.get(&w[k + 1..].iter().collect::<String>()) {
                            answer.push([i as i32, j as i32].to_vec());
                        }
                    }
                }
            }
            {
                let w = word.chars().collect::<Vec<_>>();
                for k in 0..w.len() {
                    if (0..=k / 2).all(|l| w[l] == w[k - l]) {
                        if let Some(&j) = hm.get(&w[k + 1..].iter().rev().collect::<String>()) {
                            answer.push([j as i32, i as i32].to_vec());
                        }
                    }
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
            vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]],
            Solution::palindrome_pairs(
                vec!["abcd", "dcba", "lls", "s", "sssll"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![0, 1], vec![1, 0]],
            Solution::palindrome_pairs(
                vec!["bat", "tab", "cat"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![vec![0, 1], vec![1, 0]],
            Solution::palindrome_pairs(vec!["a", ""].into_iter().map(String::from).collect())
        )
    }
}
