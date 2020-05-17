pub struct Solution {}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut ds: [usize; 26] = [0; 26];
        let mut dp: [usize; 26] = [0; 26];
        for c in p.chars() {
            dp[(c as u8 - b'a') as usize] += 1;
        }
        let mut answer: Vec<i32> = Vec::new();
        let s: &[u8] = s.as_bytes();
        for (i, c) in s.iter().enumerate() {
            ds[(c - b'a') as usize] += 1;
            if i >= p.len() {
                ds[(s[i - p.len()] as u8 - b'a') as usize] -= 1;
            }
            if ds == dp {
                answer.push((i + 1 - p.len()) as i32);
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
            vec![0, 6],
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![0, 1, 2],
            Solution::find_anagrams("abab".to_string(), "ab".to_string())
        );
    }
}
