pub struct Solution {}

impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let n = shift
            .iter()
            .map(|e| e[1] as usize * if e[0] == 0 { 1 } else { s.len() - 1 })
            .sum::<usize>()
            % s.len();
        (&s[n..]).to_string() + &s[0..n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "cab",
            Solution::string_shift("abc".to_string(), vec![vec![0, 1], vec![1, 2]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "efgabcd",
            Solution::string_shift(
                "abcdefg".to_string(),
                vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]
            )
        );
    }
}
