pub struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut answer = vec![std::i32::MAX; s.len()];
        {
            let mut prev = None;
            for (i, &b) in s.as_bytes().iter().enumerate() {
                if b as char == c {
                    prev = Some(i);
                }
                if let Some(p) = prev {
                    answer[i] = std::cmp::min(answer[i], (i - p) as i32);
                }
            }
        }
        {
            let mut prev = None;
            for (i, &b) in s.as_bytes().iter().enumerate().rev() {
                if b as char == c {
                    prev = Some(i);
                }
                if let Some(p) = prev {
                    answer[i] = std::cmp::min(answer[i], (p - i) as i32);
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
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0],
            Solution::shortest_to_char(String::from("loveleetcode"), 'e')
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![3, 2, 1, 0],
            Solution::shortest_to_char(String::from("aaab"), 'b')
        );
    }
}
