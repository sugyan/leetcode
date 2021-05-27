pub struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let v = words
            .iter()
            .map(|word| {
                word.as_bytes().iter().fold([false; 26], |mut acc, x| {
                    acc[(x - b'a') as usize] = true;
                    acc
                })
            })
            .collect::<Vec<_>>();
        let mut answer = 0;
        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                if !(0..26).any(|k| v[i][k] && v[j][k]) {
                    answer = answer.max(words[i].len() * words[j].len())
                }
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            16,
            Solution::max_product(
                vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::max_product(
                vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::max_product(
                vec!["a", "aa", "aaa", "aaaa"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
}
