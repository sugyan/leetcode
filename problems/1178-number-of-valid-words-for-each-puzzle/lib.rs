pub struct Solution;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let v = words
            .iter()
            .map(|w| w.bytes().fold(0_u32, |acc, u| acc | 1 << (u - b'a')))
            .collect::<Vec<_>>();
        let groups = v
            .iter()
            .enumerate()
            .fold(vec![Vec::new(); 26], |mut acc, (i, u)| {
                (0..26).for_each(|j| {
                    if u & (1 << j) != 0 {
                        acc[j].push(i);
                    }
                });
                acc
            });
        puzzles
            .iter()
            .map(|p| {
                let mask = p.bytes().fold(0_u32, |acc, u| acc | 1 << (u - b'a'));
                groups[(p.as_bytes()[0] - b'a') as usize]
                    .iter()
                    .filter(|&i| v[*i] & mask == v[*i])
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
