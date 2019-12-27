pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut answers: Vec<i32> = vec![0; s.len() + 1];
        answers[s.len()] = 1;
        let v: Vec<char> = s.chars().collect();
        for i in (0..v.len()).rev() {
            let n = v[i] as u8 - '0' as u8;
            if n > 0 {
                answers[i] = answers[i + 1];
                if i < v.len() - 1 && n * 10 + v[i + 1] as u8 - '0' as u8 <= 26 {
                    answers[i] += answers[i + 2];
                }
            }
        }
        return answers[0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        assert_eq!(2, Solution::num_decodings("12".to_string()));
    }

    #[test]
    fn example_02() {
        assert_eq!(3, Solution::num_decodings("226".to_string()));
    }
}
