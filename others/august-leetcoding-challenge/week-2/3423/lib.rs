pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut d: [i32; 128] = [0; 128];
        for &b in s.as_bytes() {
            d[b as usize] += 1;
        }
        let mut answer = 0;
        for n in d.iter() {
            answer += n;
            if n % 2 != 0 {
                answer -= 1;
                if answer % 2 == 0 {
                    answer += 1;
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
        assert_eq!(7, Solution::longest_palindrome("abccccdd".to_string()));
    }
}
