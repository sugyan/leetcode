pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut d = [0; 128];
        let (mut begin, mut answer) = (0, 0);
        for (i, c) in (1..).zip(s.chars()) {
            let ord = c as usize;
            if d[ord] > begin {
                begin = d[ord];
            }
            if i - begin > answer {
                answer = i - begin;
            }
            d[ord] = i;
        }
        return answer;
    }
}

fn main() {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring("bbbbb".to_string())
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
    }
}
