pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let v = s.as_bytes();
        let mut longest = (0, 0);
        let find_palindrome = |l: usize, r: usize| -> (usize, usize) {
            if r == v.len() || v[l] != v[r] {
                return (l, l);
            }
            let (mut l, mut r) = (l, r);
            while l > 0 && r < v.len() - 1 && v[l - 1] == v[r + 1] {
                l -= 1;
                r += 1;
            }
            (l, r)
        };
        for i in 0..v.len() {
            for j in 0..2 {
                let ret = find_palindrome(i, i + j);
                if ret.1 - ret.0 > longest.1 - longest.0 {
                    longest = ret;
                }
            }
        }
        s[longest.0..=longest.1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("bab", Solution::longest_palindrome(String::from("babad")))
    }

    #[test]
    fn example_2() {
        assert_eq!("bb", Solution::longest_palindrome(String::from("cbbd")));
    }

    #[test]
    fn example_3() {
        assert_eq!("a", Solution::longest_palindrome(String::from("a")));
    }

    #[test]
    fn example_4() {
        assert_eq!("a", Solution::longest_palindrome(String::from("ac")));
    }
}
