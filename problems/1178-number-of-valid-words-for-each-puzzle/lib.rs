pub struct Solution;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let v = words
            .iter()
            .map(|w| w.bytes().fold(0_u32, |acc, u| acc | 1 << (u - b'a')))
            .collect::<Vec<_>>();
        puzzles
            .iter()
            .map(|p| {
                let mask = p.bytes().fold(0_u32, |acc, u| acc | 1 << (u - b'a'));
                let first = 1_u32 << (p.as_bytes()[0] as u8 - b'a');
                v.iter()
                    .filter(|&w| first & w > 0 && mask & w == *w)
                    .count() as i32
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
            vec![1, 1, 3, 2, 4, 0],
            Solution::find_num_of_valid_words(
                vec!["aaaa", "asas", "able", "ability", "actt", "actor", "access"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                vec!["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![0, 1, 3, 2, 0],
            Solution::find_num_of_valid_words(
                vec!["apple", "pleas", "please"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                vec!["aelwxyz", "aelpxyz", "aelpsxy", "saelpxy", "xaelpsy"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            )
        );
    }
}
