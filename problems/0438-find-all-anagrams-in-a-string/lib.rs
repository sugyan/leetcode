pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut cs = [0; 26];
        let cp = p.bytes().fold([0; 26], |mut acc, u| {
            acc[(u - b'a') as usize] += 1;
            acc
        });
        let s = s.bytes().map(|u| (u - b'a') as usize).collect::<Vec<_>>();
        let mut answer = Vec::new();
        for i in 0..s.len() {
            cs[s[i]] += 1;
            if i >= p.len() {
                cs[s[i - p.len()]] -= 1;
            }
            if cs == cp {
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
            Solution::find_anagrams(String::from("cbaebabacd"), String::from("abc"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![0, 1, 2],
            Solution::find_anagrams(String::from("abab"), String::from("ab"))
        );
    }
}
