pub struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pattern = pattern
            .as_bytes()
            .iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();
        let filter = |word: &String| {
            let mut d = [None; 26];
            for (w, &i) in word.as_bytes().iter().zip(&pattern) {
                if let Some(b) = d[i] {
                    if b != w {
                        return false;
                    }
                } else {
                    d[i] = Some(w);
                }
            }
            let mut seen = [false; 26];
            for i in d.iter().filter_map(|o| o.map(|u| (u - b'a') as usize)) {
                if seen[i] {
                    return false;
                }
                seen[i] = true;
            }
            true
        };
        words.into_iter().filter(filter).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["mee", "aqq"],
            Solution::find_and_replace_pattern(
                vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                String::from("abb")
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec!["a", "b", "c"],
            Solution::find_and_replace_pattern(
                vec!["a", "b", "c"].into_iter().map(String::from).collect(),
                String::from("a")
            )
        );
    }
}
