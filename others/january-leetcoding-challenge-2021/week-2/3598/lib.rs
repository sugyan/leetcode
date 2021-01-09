pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut v = vec![0; word_list.len()];
        let mut srcs = vec![&begin_word];
        for i in 1.. {
            let mut dsts = Vec::new();
            for &src in srcs.iter() {
                for j in (0..word_list.len())
                    .filter(|&j| {
                        v[j] == 0
                            && word_list[j]
                                .chars()
                                .zip(src.chars())
                                .filter(|&cs| cs.0 != cs.1)
                                .count()
                                == 1
                    })
                    .collect::<Vec<_>>()
                {
                    if word_list[j] == end_word {
                        return i + 1;
                    }
                    v[j] = i;
                    dsts.push(&word_list[j]);
                }
            }
            if dsts.is_empty() {
                break;
            }
            srcs = dsts;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            5,
            Solution::ladder_length(
                String::from("hit"),
                String::from("cog"),
                ["hot", "dot", "dog", "lot", "log", "cog"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::ladder_length(
                String::from("hit"),
                String::from("cog"),
                ["hot", "dot", "dog", "lot", "log"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }
}
